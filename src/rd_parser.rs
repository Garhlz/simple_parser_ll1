use crate::lexer::Token;
use crate::symbol::Terminal;

#[derive(Debug, Clone)]
pub struct RdNode {
    pub label: String,
    pub children: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct RdParseOutput {
    pub root: usize,
    pub nodes: Vec<RdNode>,
    pub trace: Vec<String>,
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
    nodes: Vec<RdNode>,
    trace: Vec<String>,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            pos: 0,
            nodes: Vec::new(),
            trace: Vec::new(),
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn check(&self, kind: Terminal) -> bool {
        self.peek().kind == kind
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn add_node(&mut self, label: impl Into<String>, children: Vec<usize>) -> usize {
        self.nodes.push(RdNode {
            label: label.into(),
            children,
        });
        self.nodes.len() - 1
    }

    fn terminal(&mut self, kind: Terminal) -> Result<usize, String> {
        let token = self.peek().clone();
        if token.kind != kind {
            return Err(format!(
                "递归下降错误: 期待 `{}`, 实际读到 `{}` (offset {})",
                kind, token.kind, token.offset
            ));
        }
        self.trace.push(format!("match {}", kind));
        self.advance();
        Ok(self.add_node(kind.to_string(), vec![]))
    }

    fn enter(&mut self, name: &str) {
        self.trace.push(format!("enter {}", name));
    }

    fn parse_program(&mut self) -> Result<usize, String> {
        self.enter("parse_program");
        let stmt_list = self.parse_stmt_list()?;
        if !self.check(Terminal::End) {
            let token = self.peek();
            return Err(format!(
                "递归下降错误: 程序结束处期待 `#`, 实际读到 `{}` (offset {})",
                token.kind, token.offset
            ));
        }
        Ok(self.add_node("Program", vec![stmt_list]))
    }

    fn parse_stmt_list(&mut self) -> Result<usize, String> {
        self.enter("parse_stmt_list");
        let mut children = vec![self.parse_statement()?];
        while self.can_start_statement() {
            children.push(self.parse_statement()?);
        }
        Ok(self.add_node("StmtList", children))
    }

    fn can_start_statement(&self) -> bool {
        matches!(
            self.peek().kind,
            Terminal::Let | Terminal::If | Terminal::While | Terminal::LBrace | Terminal::Id
        )
    }

    fn parse_statement(&mut self) -> Result<usize, String> {
        self.enter("parse_statement");
        let child = match self.peek().kind {
            Terminal::Let => {
                let simple = self.parse_decl_stmt()?;
                let semi = self.terminal(Terminal::Semicolon)?;
                self.add_node("Statement", vec![simple, semi])
            }
            Terminal::Id => {
                let assign = self.parse_assign_stmt()?;
                let semi = self.terminal(Terminal::Semicolon)?;
                self.add_node("Statement", vec![assign, semi])
            }
            Terminal::If => {
                let if_stmt = self.parse_if_stmt()?;
                self.add_node("Statement", vec![if_stmt])
            }
            Terminal::While => {
                let while_stmt = self.parse_while_stmt()?;
                self.add_node("Statement", vec![while_stmt])
            }
            Terminal::LBrace => {
                let block = self.parse_block()?;
                self.add_node("Statement", vec![block])
            }
            _ => {
                let token = self.peek();
                return Err(format!(
                    "递归下降错误: 非法语句起始符 `{}` (offset {})",
                    token.kind, token.offset
                ));
            }
        };
        Ok(child)
    }

    fn parse_block(&mut self) -> Result<usize, String> {
        self.enter("parse_block");
        let lbrace = self.terminal(Terminal::LBrace)?;
        let mut children = vec![lbrace];
        if self.can_start_statement() {
            children.push(self.parse_stmt_list()?);
        } else {
            children.push(self.add_node("ε", vec![]));
        }
        let rbrace = self.terminal(Terminal::RBrace)?;
        children.push(rbrace);
        Ok(self.add_node("Block", children))
    }

    fn parse_decl_stmt(&mut self) -> Result<usize, String> {
        self.enter("parse_decl_stmt");
        let let_kw = self.terminal(Terminal::Let)?;
        let variable = self.parse_variable()?;
        let mut children = vec![let_kw, variable];
        if self.check(Terminal::Assign) {
            let eq = self.terminal(Terminal::Assign)?;
            let expr = self.parse_expr()?;
            children.push(eq);
            children.push(expr);
        } else {
            children.push(self.add_node("ε", vec![]));
        }
        Ok(self.add_node("DeclStmt", children))
    }

    fn parse_assign_stmt(&mut self) -> Result<usize, String> {
        self.enter("parse_assign_stmt");
        let variable = self.parse_variable()?;
        let eq = self.terminal(Terminal::Assign)?;
        let expr = self.parse_expr()?;
        Ok(self.add_node("AssignStmt", vec![variable, eq, expr]))
    }

    fn parse_if_stmt(&mut self) -> Result<usize, String> {
        self.enter("parse_if_stmt");
        let if_kw = self.terminal(Terminal::If)?;
        let lp = self.terminal(Terminal::LParen)?;
        let cond = self.parse_bool_expr()?;
        let rp = self.terminal(Terminal::RParen)?;
        let then_block = self.parse_block()?;
        let mut children = vec![if_kw, lp, cond, rp, then_block];
        if self.check(Terminal::Else) {
            let else_kw = self.terminal(Terminal::Else)?;
            let else_body = if self.check(Terminal::If) {
                self.parse_if_stmt()?
            } else {
                self.parse_block()?
            };
            children.push(else_kw);
            children.push(else_body);
        } else {
            children.push(self.add_node("ε", vec![]));
        }
        Ok(self.add_node("IfStmt", children))
    }

    fn parse_while_stmt(&mut self) -> Result<usize, String> {
        self.enter("parse_while_stmt");
        let while_kw = self.terminal(Terminal::While)?;
        let lp = self.terminal(Terminal::LParen)?;
        let cond = self.parse_bool_expr()?;
        let rp = self.terminal(Terminal::RParen)?;
        let block = self.parse_block()?;
        Ok(self.add_node("WhileStmt", vec![while_kw, lp, cond, rp, block]))
    }

    fn parse_variable(&mut self) -> Result<usize, String> {
        self.enter("parse_variable");
        let id = self.terminal(Terminal::Id)?;
        Ok(self.add_node("Variable", vec![id]))
    }

    fn parse_expr(&mut self) -> Result<usize, String> {
        self.enter("parse_expr");
        let expr = self.parse_expr_bp(0)?;
        Ok(self.add_node("Expr", vec![expr]))
    }

    fn parse_expr_bp(&mut self, min_bp: u8) -> Result<usize, String> {
        let mut left = self.parse_expr_atom()?;
        loop {
            let op = match self.peek().kind {
                Terminal::Plus => (1, 2, Terminal::Plus),
                Terminal::Minus => (1, 2, Terminal::Minus),
                Terminal::Star => (3, 4, Terminal::Star),
                Terminal::Slash => (3, 4, Terminal::Slash),
                _ => break,
            };
            if op.0 < min_bp {
                break;
            }
            let oper = self.terminal(op.2)?;
            let right = self.parse_expr_bp(op.1)?;
            left = self.add_node("BinaryExpr", vec![left, oper, right]);
        }
        Ok(left)
    }

    fn parse_expr_atom(&mut self) -> Result<usize, String> {
        match self.peek().kind {
            Terminal::Id => self.parse_variable(),
            Terminal::Num => {
                let num = self.terminal(Terminal::Num)?;
                Ok(self.add_node("Number", vec![num]))
            }
            Terminal::LParen => {
                let lp = self.terminal(Terminal::LParen)?;
                let expr = self.parse_expr()?;
                let rp = self.terminal(Terminal::RParen)?;
                Ok(self.add_node("GroupedExpr", vec![lp, expr, rp]))
            }
            _ => {
                let token = self.peek();
                Err(format!(
                    "递归下降错误: 非法表达式起始符 `{}` (offset {})",
                    token.kind, token.offset
                ))
            }
        }
    }

    fn parse_bool_expr(&mut self) -> Result<usize, String> {
        self.enter("parse_bool_expr");
        let expr = self.parse_bool_bp(0)?;
        Ok(self.add_node("BoolExpr", vec![expr]))
    }

    fn parse_bool_bp(&mut self, min_bp: u8) -> Result<usize, String> {
        let mut left = if self.check(Terminal::Not) {
            let not_kw = self.terminal(Terminal::Not)?;
            let rhs = self.parse_bool_bp(5)?;
            self.add_node("NotExpr", vec![not_kw, rhs])
        } else {
            self.parse_bool_atom()?
        };

        loop {
            let op = match self.peek().kind {
                Terminal::Or => (1, 2, Terminal::Or),
                Terminal::And => (3, 4, Terminal::And),
                _ => break,
            };
            if op.0 < min_bp {
                break;
            }
            let oper = self.terminal(op.2)?;
            let right = self.parse_bool_bp(op.1)?;
            left = self.add_node("LogicalExpr", vec![left, oper, right]);
        }
        Ok(left)
    }

    fn parse_bool_atom(&mut self) -> Result<usize, String> {
        match self.peek().kind {
            Terminal::True => {
                let t = self.terminal(Terminal::True)?;
                Ok(self.add_node("BoolLiteral", vec![t]))
            }
            Terminal::False => {
                let f = self.terminal(Terminal::False)?;
                Ok(self.add_node("BoolLiteral", vec![f]))
            }
            Terminal::LParen => {
                let lp = self.terminal(Terminal::LParen)?;
                let expr = self.parse_bool_expr()?;
                let rp = self.terminal(Terminal::RParen)?;
                Ok(self.add_node("GroupedBoolExpr", vec![lp, expr, rp]))
            }
            Terminal::Id | Terminal::Num => self.parse_relation(),
            _ => {
                let token = self.peek();
                Err(format!(
                    "递归下降错误: 非法布尔表达式起始符 `{}` (offset {})",
                    token.kind, token.offset
                ))
            }
        }
    }

    fn parse_relation(&mut self) -> Result<usize, String> {
        let left = match self.peek().kind {
            Terminal::Id => self.parse_variable()?,
            Terminal::Num => {
                let num = self.terminal(Terminal::Num)?;
                self.add_node("Number", vec![num])
            }
            _ => {
                let token = self.peek();
                return Err(format!(
                    "递归下降错误: 比较表达式左侧非法 `{}` (offset {})",
                    token.kind, token.offset
                ));
            }
        };

        let op = match self.peek().kind {
            Terminal::Eq => self.terminal(Terminal::Eq)?,
            Terminal::Ne => self.terminal(Terminal::Ne)?,
            Terminal::Lt => self.terminal(Terminal::Lt)?,
            Terminal::Le => self.terminal(Terminal::Le)?,
            Terminal::Gt => self.terminal(Terminal::Gt)?,
            Terminal::Ge => self.terminal(Terminal::Ge)?,
            _ => {
                let token = self.peek();
                return Err(format!(
                    "递归下降错误: 期待比较运算符，实际读到 `{}` (offset {})",
                    token.kind, token.offset
                ));
            }
        };

        let right = self.parse_expr()?;
        Ok(self.add_node("Relation", vec![left, op, right]))
    }
}

pub fn parse(tokens: &[Token]) -> Result<RdParseOutput, String> {
    if tokens.is_empty() {
        return Err("输入 token 序列为空".into());
    }

    let mut parser = Parser::new(tokens);
    let root = parser.parse_program()?;
    if !parser.check(Terminal::End) {
        let token = parser.peek();
        return Err(format!(
            "递归下降错误: 输入未完全消费，当前位置是 `{}` (offset {})",
            token.kind, token.offset
        ));
    }
    Ok(RdParseOutput {
        root,
        nodes: parser.nodes,
        trace: parser.trace,
    })
}

pub fn format_tree(nodes: &[RdNode], root: usize) -> String {
    fn dfs(nodes: &[RdNode], node_id: usize, depth: usize, out: &mut String) {
        out.push_str(&"  ".repeat(depth));
        out.push_str(&nodes[node_id].label);
        out.push('\n');
        for &child in &nodes[node_id].children {
            dfs(nodes, child, depth + 1, out);
        }
    }

    let mut out = String::new();
    dfs(nodes, root, 0, &mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::{format_tree, parse};
    use crate::lexer::tokenize;

    fn parse_ok(input: &str) -> String {
        let tokens = tokenize(input).expect("tokenize should succeed");
        let output = parse(&tokens).expect("rd parse should succeed");
        format_tree(&output.nodes, output.root)
    }

    #[test]
    fn test_parse_if_while_block() {
        let tree = parse_ok(
            "if ( x < 10 or false ) { let y = 1; } else { while ( x > 0 ) { x = x - 1; } }",
        );
        assert!(tree.contains("IfStmt"));
        assert!(tree.contains("WhileStmt"));
        assert!(tree.contains("LogicalExpr"));
    }

    #[test]
    fn test_parse_decl_and_assign() {
        let tree = parse_ok("{ let a = 1; a = a - 1; }");
        assert!(tree.contains("DeclStmt"));
        assert!(tree.contains("AssignStmt"));
        assert!(tree.contains("BinaryExpr"));
    }

    #[test]
    fn test_parse_error() {
        let tokens = tokenize("while ( x > ) { x = 1; }").expect("tokenize should succeed");
        let err = parse(&tokens).expect_err("rd parse should fail");
        assert!(err.contains("递归下降错误"));
    }
}
