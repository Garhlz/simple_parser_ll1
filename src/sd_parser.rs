use crate::{
    codegen::{BoolAttr, CodeGen, ExprAttr, JumpTarget, Operand, Quad},
    lexer::Token,
    symbol::Terminal,
};

#[derive(Debug)]
pub struct BoolParseOutput {
    pub codegen: CodeGen,
    pub attr: BoolAttr,
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
    codegen: CodeGen,
}

impl<'a> Parser<'a> {
    // 辅助函数
    fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            pos: 0,
            codegen: CodeGen::new(),
        }
    }

    fn peek(&self) -> Result<&Token, String> {
        self.tokens
            .get(self.pos)
            .ok_or_else(|| "语法制导翻译错误: 输入意外结束".into())
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn can_start_stmt(&self) -> bool {
        self.peek().is_ok_and(|token| {
            matches!(
                token.kind,
                Terminal::Id | Terminal::LBrace | Terminal::If | Terminal::While
            )
        })
    }

    /// 期待当前 token 是某种终结符，并且消费它
    fn expect_terminal(&mut self, kind: Terminal) -> Result<Token, String> {
        let token = self.peek()?.clone();
        if token.kind != kind {
            return Err(format!(
                "语法制导翻译错误: 期待 `{}`, 实际读到 `{}` (offset {})",
                kind, token.kind, token.offset
            ));
        }
        self.advance();
        Ok(token)
    }

    // 实际 parser 逻辑

    fn parse_program(&mut self) -> Result<(), String> {
        self.parse_stmt_list()?;
        self.expect_terminal(Terminal::End)?;
        Ok(())
    }

    fn parse_stmt_list(&mut self) -> Result<(), String> {
        while self.can_start_stmt() {
            self.parse_stmt()?;
        }
        Ok(())
    }

    fn parse_stmt(&mut self) -> Result<(), String> {
        match self.peek()?.kind {
            Terminal::Id => self.parse_assignment_stmt(),
            Terminal::LBrace => self.parse_block(),
            Terminal::If => self.parse_if_stmt(),
            Terminal::While => self.parse_while_stmt(),
            _ => {
                let token = self.peek()?;
                Err(format!(
                    "语法制导翻译错误: 非法语句起始符 `{}` (offset {})",
                    token.kind, token.offset
                ))
            }
        }
    }

    fn parse_block(&mut self) -> Result<(), String> {
        self.expect_terminal(Terminal::LBrace)?;
        self.parse_stmt_list()?;
        self.expect_terminal(Terminal::RBrace)?;
        Ok(())
    }

    fn parse_assignment_stmt(&mut self) -> Result<(), String> {
        let id = self.expect_terminal(Terminal::Id)?;
        self.expect_terminal(Terminal::Assign)?;
        let expr = self.parse_expr()?;
        self.expect_terminal(Terminal::Semicolon)?;

        self.codegen.emit(Quad::Assign {
            src: expr.place,
            dest: Operand::name(id.lexeme),
        });
        Ok(())
    }

    fn parse_if_stmt(&mut self) -> Result<(), String> {
        self.expect_terminal(Terminal::If)?;
        self.expect_terminal(Terminal::LParen)?;
        let attr = self.parse_or()?;
        self.expect_terminal(Terminal::RParen)?;

        let then_start = self.codegen.next_quad();

        // 真链回填
        self.codegen.backpatch(&attr.tc, then_start)?;

        self.parse_block()?;

        // 有else部分
        if matches!(self.peek()?.kind, Terminal::Else) {
            let jump_index = self.codegen.next_quad();
            self.codegen.emit(Quad::Jump {
                target: JumpTarget::Pending,
            });
            // if 执行完之后无条件跳转到else body之后
            self.expect_terminal(Terminal::Else)?;
            let else_body_start = self.codegen.next_quad();
            self.codegen.backpatch(&attr.fc, else_body_start)?;

            self.parse_block()?;

            // then body可以直接跳转到最后的位置
            let after_else = self.codegen.next_quad();
            self.codegen.backpatch(&[jump_index], after_else)?;
        } else {
            let after_then = self.codegen.next_quad();
            // 回填未确定的假链
            self.codegen.backpatch(&attr.fc, after_then)?;
        }

        Ok(())
    }

    fn parse_while_stmt(&mut self) -> Result<(), String> {
        self.expect_terminal(Terminal::While)?;
        self.expect_terminal(Terminal::LParen)?;

        let loop_begin = self.codegen.next_quad();

        let attr = self.parse_or()?;
        self.expect_terminal(Terminal::RParen)?;

        let then_body_start = self.codegen.next_quad();
        self.codegen.backpatch(&attr.tc, then_body_start)?;

        self.parse_block()?;

        // 无条件跳转回到 while 开始的地方
        self.codegen.emit(Quad::Jump {
            target: JumpTarget::Target(loop_begin),
        });

        let then_body_end = self.codegen.next_quad();
        self.codegen.backpatch(&attr.fc, then_body_end)?;

        Ok(())
    }

    fn parse_bool_expr(&mut self) -> Result<BoolAttr, String> {
        let attr = self.parse_or()?;
        self.expect_terminal(Terminal::End)?;
        Ok(attr)
    }

    // or 优先级最低，在最上层调用
    fn parse_or(&mut self) -> Result<BoolAttr, String> {
        let mut left = self.parse_and()?;

        // 可以连接多个短路 or
        while matches!(self.peek()?.kind, Terminal::Or) {
            self.expect_terminal(Terminal::Or)?;

            let right_start = self.codegen.next_quad();
            self.codegen.backpatch(&left.fc, right_start)?;

            let right = self.parse_and()?;

            left = BoolAttr {
                tc: CodeGen::merge(left.tc, right.tc),
                fc: right.fc,
            };
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<BoolAttr, String> {
        let mut left = self.parse_not()?;

        // 可以连接多个短路 and
        while matches!(self.peek()?.kind, Terminal::And) {
            self.expect_terminal(Terminal::And)?;

            let right_start = self.codegen.next_quad();
            self.codegen.backpatch(&left.tc, right_start)?;

            let right = self.parse_not()?;

            left = BoolAttr {
                tc: right.tc,
                fc: CodeGen::merge(left.fc, right.fc),
            };
        }

        Ok(left)
    }

    fn parse_not(&mut self) -> Result<BoolAttr, String> {
        if matches!(self.peek()?.kind, Terminal::Not) {
            self.expect_terminal(Terminal::Not)?;
            // 递归调用自己，可以连续使用多个not
            let mut left = self.parse_not()?;
            left = BoolAttr {
                tc: left.fc,
                fc: left.tc,
            };
            Ok(left)
        } else {
            self.parse_bool_atom()
        }
    }

    // 支持括号优先级
    fn parse_bool_atom(&mut self) -> Result<BoolAttr, String> {
        if matches!(self.peek()?.kind, Terminal::LParen) {
            self.expect_terminal(Terminal::LParen)?;
            let attr = self.parse_or()?;
            self.expect_terminal(Terminal::RParen)?;
            Ok(attr)
        } else {
            self.parse_relation()
        }
    }

    fn parse_relation(&mut self) -> Result<BoolAttr, String> {
        let left = self.parse_expr()?;
        let relop = self.parse_relop()?;
        let right = self.parse_expr()?;

        let true_jump = self.codegen.emit(Quad::JumpIf {
            relop,
            left: left.place,
            right: right.place,
            target: JumpTarget::Pending,
        });
        let false_jump = self.codegen.emit(Quad::Jump {
            target: JumpTarget::Pending,
        });

        Ok(BoolAttr {
            tc: CodeGen::makelist(true_jump),
            fc: CodeGen::makelist(false_jump),
        })
    }

    fn parse_relop(&mut self) -> Result<String, String> {
        let token = self.peek()?.clone();
        match token.kind {
            Terminal::Eq
            | Terminal::Ne
            | Terminal::Lt
            | Terminal::Le
            | Terminal::Gt
            | Terminal::Ge => {
                self.advance();
                Ok(token.lexeme)
            }
            _ => Err(format!(
                "语法制导翻译错误: 期待比较运算符，实际读到 `{}` (offset {})",
                token.kind, token.offset
            )),
        }
    }

    fn parse_expr(&mut self) -> Result<ExprAttr, String> {
        self.parse_expr_bp(0)
    }

    fn parse_expr_bp(&mut self, min_bp: u8) -> Result<ExprAttr, String> {
        let mut left = self.parse_expr_atom()?;

        loop {
            let (left_bp, right_bp, op) = match self.peek()?.kind {
                Terminal::Plus => (1, 2, "+"),
                Terminal::Minus => (1, 2, "-"),
                Terminal::Star => (3, 4, "*"),
                Terminal::Slash => (3, 4, "/"),
                _ => break,
            };

            if left_bp < min_bp {
                break;
            }

            self.advance();
            let right = self.parse_expr_bp(right_bp)?;
            // 在这里创建中间产物
            let temp = self.codegen.new_temp();
            self.codegen.emit(Quad::Binary {
                op: op.into(),
                left: left.place,
                right: right.place,
                dest: temp.clone(),
            });
            left = ExprAttr { place: temp };
        }

        Ok(left)
    }

    fn parse_expr_atom(&mut self) -> Result<ExprAttr, String> {
        match self.peek()?.kind {
            Terminal::Id => {
                let token = self.expect_terminal(Terminal::Id)?;
                Ok(ExprAttr {
                    place: Operand::name(token.lexeme),
                })
            }
            Terminal::Num => {
                let token = self.expect_terminal(Terminal::Num)?;
                Ok(ExprAttr {
                    place: Operand::name(token.lexeme),
                })
            }
            Terminal::LParen => {
                self.expect_terminal(Terminal::LParen)?;
                let expr = self.parse_expr()?;
                self.expect_terminal(Terminal::RParen)?;
                Ok(expr)
            }
            _ => {
                let token = self.peek()?;
                Err(format!(
                    "语法制导翻译错误: 非法表达式起始符 `{}` (offset {})",
                    token.kind, token.offset
                ))
            }
        }
    }
}

pub fn parse_program(tokens: &[Token]) -> Result<CodeGen, String> {
    if tokens.is_empty() {
        return Err("语法制导翻译错误: 输入 token 序列为空".into());
    }

    let mut parser = Parser::new(tokens);
    parser.parse_program()?;
    Ok(parser.codegen)
}

pub fn parse_assignment(tokens: &[Token]) -> Result<CodeGen, String> {
    if tokens.is_empty() {
        return Err("语法制导翻译错误: 输入 token 序列为空".into());
    }

    let mut parser = Parser::new(tokens);
    parser.parse_assignment_stmt()?;
    parser.expect_terminal(Terminal::End)?;
    Ok(parser.codegen)
}

pub fn parse_bool(tokens: &[Token]) -> Result<BoolParseOutput, String> {
    if tokens.is_empty() {
        return Err("语法制导翻译错误: 输入 token 序列为空".into());
    }

    let mut parser = Parser::new(tokens);
    let attr = parser.parse_bool_expr()?;
    Ok(BoolParseOutput {
        codegen: parser.codegen,
        attr,
    })
}

#[cfg(test)]
mod tests {
    use super::{parse_assignment, parse_bool, parse_program};
    use crate::lexer::tokenize;

    fn assign_quad_lines(input: &str) -> Vec<String> {
        let tokens = tokenize(input).expect("tokenize should succeed");
        let codegen = parse_assignment(&tokens).expect("parse assignment should succeed");
        codegen
            .quads
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn bool_quads_and_attr(input: &str) -> (Vec<String>, Vec<usize>, Vec<usize>) {
        let tokens = tokenize(input).expect("tokenize should succeed");
        let output = parse_bool(&tokens).expect("parse bool should succeed");
        let quads = output
            .codegen
            .quads
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        (quads, output.attr.tc, output.attr.fc)
    }

    fn program_quad_lines(input: &str) -> Vec<String> {
        let tokens = tokenize(input).expect("tokenize should succeed");
        let codegen = parse_program(&tokens).expect("parse program should succeed");
        codegen
            .quads
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_parse_simple_assignment_expr() {
        assert_eq!(
            assign_quad_lines("a = b + c;"),
            vec!["(+, b, c, t1)", "(=, t1, _, a)"]
        );
    }

    #[test]
    fn test_parse_assignment_expr_precedence() {
        assert_eq!(
            assign_quad_lines("a = b + c * e / g;"),
            vec![
                "(*, c, e, t1)",
                "(/, t1, g, t2)",
                "(+, b, t2, t3)",
                "(=, t3, _, a)",
            ]
        );
    }

    #[test]
    fn test_parse_assignment_expr_grouping() {
        assert_eq!(
            assign_quad_lines("a = (b + c) * d;"),
            vec!["(+, b, c, t1)", "(*, t1, d, t2)", "(=, t2, _, a)"]
        );
    }

    #[test]
    fn test_parse_assignment_error() {
        let tokens = tokenize("a = b + ;").expect("tokenize should succeed");
        let err = parse_assignment(&tokens).expect_err("parse should fail");

        assert!(err.contains("非法表达式起始符"));
    }

    #[test]
    fn test_parse_assignment_rejects_trailing_statement() {
        let tokens = tokenize("a = b + c; x = a * d;").expect("tokenize should succeed");
        let err = parse_assignment(&tokens).expect_err("parse should fail");

        assert!(err.contains("期待 `#`"));
    }

    #[test]
    fn test_parse_program_assignment_list() {
        assert_eq!(
            program_quad_lines("a = b + c; x = a * d;"),
            vec![
                "(+, b, c, t1)",
                "(=, t1, _, a)",
                "(*, a, d, t2)",
                "(=, t2, _, x)",
            ]
        );
    }

    #[test]
    fn test_parse_program_blocks() {
        assert_eq!(
            program_quad_lines("{ a = b + c; { x = a * d; } }"),
            vec![
                "(+, b, c, t1)",
                "(=, t1, _, a)",
                "(*, a, d, t2)",
                "(=, t2, _, x)",
            ]
        );
    }

    #[test]
    fn test_parse_program_empty_block() {
        assert_eq!(program_quad_lines("{ }"), Vec::<String>::new());
    }

    #[test]
    fn test_parse_program_if() {
        assert_eq!(
            program_quad_lines("if (a < b) { x = y + z; }"),
            vec![
                "(j<, a, b, 2)",
                "(j, _, _, 4)",
                "(+, y, z, t1)",
                "(=, t1, _, x)",
            ]
        );
    }

    #[test]
    fn test_parse_program_if_else() {
        assert_eq!(
            program_quad_lines("if (a < b) { x = y + z; } else { x = y - z; }"),
            vec![
                "(j<, a, b, 2)",
                "(j, _, _, 5)",
                "(+, y, z, t1)",
                "(=, t1, _, x)",
                "(j, _, _, 7)",
                "(-, y, z, t2)",
                "(=, t2, _, x)",
            ]
        );
    }

    #[test]
    fn test_parse_program_while() {
        assert_eq!(
            program_quad_lines("while (a < b) { a = a + 1; }"),
            vec![
                "(j<, a, b, 2)",
                "(j, _, _, 5)",
                "(+, a, 1, t1)",
                "(=, t1, _, a)",
                "(j, _, _, 0)",
            ]
        );
    }

    #[test]
    fn test_parse_program_nested_control_flow() {
        assert_eq!(
            program_quad_lines("while (a < b) { if (c != d) { x = x + 1; } }"),
            vec![
                "(j<, a, b, 2)",
                "(j, _, _, 7)",
                "(j!=, c, d, 4)",
                "(j, _, _, 6)",
                "(+, x, 1, t1)",
                "(=, t1, _, x)",
                "(j, _, _, 0)",
            ]
        );
    }

    #[test]
    fn test_parse_program_if_else_with_following_stmt() {
        assert_eq!(
            program_quad_lines("if (a < b) { x = y + z; } else { x = y - z; } k = x * 2;"),
            vec![
                "(j<, a, b, 2)",
                "(j, _, _, 5)",
                "(+, y, z, t1)",
                "(=, t1, _, x)",
                "(j, _, _, 7)",
                "(-, y, z, t2)",
                "(=, t2, _, x)",
                "(*, x, 2, t3)",
                "(=, t3, _, k)",
            ]
        );
    }

    #[test]
    fn test_parse_program_missing_rbrace_error() {
        let tokens = tokenize("{ a = b + c;").expect("tokenize should succeed");
        let err = parse_program(&tokens).expect_err("parse should fail");

        assert!(err.contains("期待 `}`"));
    }

    #[test]
    fn test_parse_bool_relation() {
        let (quads, tc, fc) = bool_quads_and_attr("a < b");

        assert_eq!(quads, vec!["(j<, a, b, _)", "(j, _, _, _)"]);
        assert_eq!(tc, vec![0]);
        assert_eq!(fc, vec![1]);
    }

    #[test]
    fn test_parse_bool_relation_with_arithmetic_sides() {
        let (quads, tc, fc) = bool_quads_and_attr("a + b <= c * d");

        assert_eq!(
            quads,
            vec![
                "(+, a, b, t1)",
                "(*, c, d, t2)",
                "(j<=, t1, t2, _)",
                "(j, _, _, _)",
            ]
        );
        assert_eq!(tc, vec![2]);
        assert_eq!(fc, vec![3]);
    }

    #[test]
    fn test_parse_bool_and() {
        let (quads, tc, fc) = bool_quads_and_attr("a < b and c > d");

        assert_eq!(
            quads,
            vec![
                "(j<, a, b, 2)",
                "(j, _, _, _)",
                "(j>, c, d, _)",
                "(j, _, _, _)",
            ]
        );
        assert_eq!(tc, vec![2]);
        assert_eq!(fc, vec![1, 3]);
    }

    #[test]
    fn test_parse_bool_or() {
        let (quads, tc, fc) = bool_quads_and_attr("a < b or c > d");

        assert_eq!(
            quads,
            vec![
                "(j<, a, b, _)",
                "(j, _, _, 2)",
                "(j>, c, d, _)",
                "(j, _, _, _)",
            ]
        );
        assert_eq!(tc, vec![0, 2]);
        assert_eq!(fc, vec![3]);
    }

    #[test]
    fn test_parse_bool_not() {
        let (quads, tc, fc) = bool_quads_and_attr("not a < b");

        assert_eq!(quads, vec!["(j<, a, b, _)", "(j, _, _, _)"]);
        assert_eq!(tc, vec![1]);
        assert_eq!(fc, vec![0]);
    }

    #[test]
    fn test_parse_bool_double_not() {
        let (quads, tc, fc) = bool_quads_and_attr("not not a < b");

        assert_eq!(quads, vec!["(j<, a, b, _)", "(j, _, _, _)"]);
        assert_eq!(tc, vec![0]);
        assert_eq!(fc, vec![1]);
    }

    #[test]
    fn test_parse_bool_repeated_and() {
        let (quads, tc, fc) = bool_quads_and_attr("a < b and c > d and e != f");

        assert_eq!(
            quads,
            vec![
                "(j<, a, b, 2)",
                "(j, _, _, _)",
                "(j>, c, d, 4)",
                "(j, _, _, _)",
                "(j!=, e, f, _)",
                "(j, _, _, _)",
            ]
        );
        assert_eq!(tc, vec![4]);
        assert_eq!(fc, vec![1, 3, 5]);
    }

    #[test]
    fn test_parse_bool_and_precedes_or() {
        let (quads, tc, fc) = bool_quads_and_attr("a < b or c > d and e != f");

        assert_eq!(
            quads,
            vec![
                "(j<, a, b, _)",
                "(j, _, _, 2)",
                "(j>, c, d, 4)",
                "(j, _, _, _)",
                "(j!=, e, f, _)",
                "(j, _, _, _)",
            ]
        );
        assert_eq!(tc, vec![0, 4]);
        assert_eq!(fc, vec![3, 5]);
    }

    #[test]
    fn test_parse_bool_grouping() {
        let (quads, tc, fc) = bool_quads_and_attr("(a < b or c > d) and e != f");

        assert_eq!(
            quads,
            vec![
                "(j<, a, b, 4)",
                "(j, _, _, 2)",
                "(j>, c, d, 4)",
                "(j, _, _, _)",
                "(j!=, e, f, _)",
                "(j, _, _, _)",
            ]
        );
        assert_eq!(tc, vec![4]);
        assert_eq!(fc, vec![3, 5]);
    }

    #[test]
    fn test_parse_bool_not_grouping() {
        let (quads, tc, fc) = bool_quads_and_attr("not (a < b or c > d)");

        assert_eq!(
            quads,
            vec![
                "(j<, a, b, _)",
                "(j, _, _, 2)",
                "(j>, c, d, _)",
                "(j, _, _, _)",
            ]
        );
        assert_eq!(tc, vec![3]);
        assert_eq!(fc, vec![0, 2]);
    }

    #[test]
    fn test_parse_bool_relation_error() {
        let tokens = tokenize("a + b").expect("tokenize should succeed");
        let err = parse_bool(&tokens).expect_err("parse should fail");

        assert!(err.contains("期待比较运算符"));
    }
}
