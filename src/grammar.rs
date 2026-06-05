use crate::symbol::{NonTerminal, Symbol, Terminal};
use std::collections::HashSet;
use std::fmt;

/// 产生式：一条文法规则
/// 如果 rhs 为空 Vec，表示 ε（空串）
#[derive(Clone)]
pub struct Production {
    pub lhs: NonTerminal,
    pub rhs: Vec<Symbol>,
}

/// 文法：产生式集合 + 开始符号
pub struct Grammar {
    pub name: String,
    pub start: NonTerminal,
    pub productions: Vec<Production>,
}

// ============================================================
// Display 实现
// ============================================================

impl fmt::Display for Production {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lhs)?;
        if self.rhs.is_empty() {
            write!(f, " → ε")?;
        } else {
            write!(f, " →")?;
            for sym in &self.rhs {
                write!(f, " {sym}")?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "文法: {}", self.name)?;
        writeln!(f, "开始符号: {}\n", self.start)?;
        for (i, p) in self.productions.iter().enumerate() {
            writeln!(f, "  ({})  {p}", i + 1)?;
        }
        Ok(())
    }
}

// ============================================================
// 快捷构造函数（减少 boilerplate）
// ============================================================

fn t(terminal: Terminal) -> Symbol {
    Symbol::Terminal(terminal)
}

fn nt(non_terminal: NonTerminal) -> Symbol {
    Symbol::NonTerminal(non_terminal)
}

fn prod(lhs: NonTerminal, rhs: Vec<Symbol>) -> Production {
    Production { lhs, rhs }
}

// ============================================================
// 文法定义
// ============================================================

impl Grammar {
    /// Simple 原文法（含直接左递归，不可直接用于 LL(1) 分析）
    pub fn original() -> Self {
        let productions = vec![
            // Program → StmtList
            prod(NonTerminal::Program, vec![nt(NonTerminal::StmtList)]),
            // StmtList → StmtList Statement | Statement
            prod(
                NonTerminal::StmtList,
                vec![nt(NonTerminal::StmtList), nt(NonTerminal::Statement)],
            ),
            prod(NonTerminal::StmtList, vec![nt(NonTerminal::Statement)]),
            // Statement → SimpleStmt ; | IfStmt | WhileStmt | Block
            prod(
                NonTerminal::Statement,
                vec![nt(NonTerminal::SimpleStmt), t(Terminal::Semicolon)],
            ),
            prod(NonTerminal::Statement, vec![nt(NonTerminal::IfStmt)]),
            prod(NonTerminal::Statement, vec![nt(NonTerminal::WhileStmt)]),
            prod(NonTerminal::Statement, vec![nt(NonTerminal::Block)]),
            // Block → { StmtList } | { }
            prod(
                NonTerminal::Block,
                vec![
                    t(Terminal::LBrace),
                    nt(NonTerminal::StmtList),
                    t(Terminal::RBrace),
                ],
            ),
            prod(
                NonTerminal::Block,
                vec![t(Terminal::LBrace), t(Terminal::RBrace)],
            ),
            // SimpleStmt → DeclStmt | AssignStmt
            prod(NonTerminal::SimpleStmt, vec![nt(NonTerminal::DeclStmt)]),
            prod(NonTerminal::SimpleStmt, vec![nt(NonTerminal::AssignStmt)]),
            // DeclStmt → let Variable DeclInit
            prod(
                NonTerminal::DeclStmt,
                vec![
                    t(Terminal::Let),
                    nt(NonTerminal::Variable),
                    nt(NonTerminal::DeclInit),
                ],
            ),
            // DeclInit → = Expr | ε
            prod(
                NonTerminal::DeclInit,
                vec![t(Terminal::Assign), nt(NonTerminal::Expr)],
            ),
            prod(NonTerminal::DeclInit, vec![]),
            // AssignStmt → Variable = Expr
            prod(
                NonTerminal::AssignStmt,
                vec![
                    nt(NonTerminal::Variable),
                    t(Terminal::Assign),
                    nt(NonTerminal::Expr),
                ],
            ),
            // IfStmt → if ( BoolExpr ) Block ElsePart
            prod(
                NonTerminal::IfStmt,
                vec![
                    t(Terminal::If),
                    t(Terminal::LParen),
                    nt(NonTerminal::BoolExpr),
                    t(Terminal::RParen),
                    nt(NonTerminal::Block),
                    nt(NonTerminal::ElsePart),
                ],
            ),
            // ElsePart → else ElseBody | ε
            prod(
                NonTerminal::ElsePart,
                vec![t(Terminal::Else), nt(NonTerminal::ElseBody)],
            ),
            prod(NonTerminal::ElsePart, vec![]),
            // ElseBody → Block | IfStmt
            prod(NonTerminal::ElseBody, vec![nt(NonTerminal::Block)]),
            prod(NonTerminal::ElseBody, vec![nt(NonTerminal::IfStmt)]),
            // WhileStmt → while ( BoolExpr ) Block
            prod(
                NonTerminal::WhileStmt,
                vec![
                    t(Terminal::While),
                    t(Terminal::LParen),
                    nt(NonTerminal::BoolExpr),
                    t(Terminal::RParen),
                    nt(NonTerminal::Block),
                ],
            ),
            // Expr → Expr + Term | Expr - Term | Term   ← 直接左递归
            prod(
                NonTerminal::Expr,
                vec![
                    nt(NonTerminal::Expr),
                    t(Terminal::Plus),
                    nt(NonTerminal::Term),
                ],
            ),
            prod(
                NonTerminal::Expr,
                vec![
                    nt(NonTerminal::Expr),
                    t(Terminal::Minus),
                    nt(NonTerminal::Term),
                ],
            ),
            prod(NonTerminal::Expr, vec![nt(NonTerminal::Term)]),
            // Term → Term * Factor | Term / Factor | Factor   ← 直接左递归
            prod(
                NonTerminal::Term,
                vec![
                    nt(NonTerminal::Term),
                    t(Terminal::Star),
                    nt(NonTerminal::Factor),
                ],
            ),
            prod(
                NonTerminal::Term,
                vec![
                    nt(NonTerminal::Term),
                    t(Terminal::Slash),
                    nt(NonTerminal::Factor),
                ],
            ),
            prod(NonTerminal::Term, vec![nt(NonTerminal::Factor)]),
            // Factor → ( Expr ) | Variable | num
            prod(
                NonTerminal::Factor,
                vec![
                    t(Terminal::LParen),
                    nt(NonTerminal::Expr),
                    t(Terminal::RParen),
                ],
            ),
            prod(NonTerminal::Factor, vec![nt(NonTerminal::Variable)]),
            prod(NonTerminal::Factor, vec![t(Terminal::Num)]),
            // BoolExpr → BoolExpr or BoolAnd | BoolAnd   ← 直接左递归
            prod(
                NonTerminal::BoolExpr,
                vec![
                    nt(NonTerminal::BoolExpr),
                    t(Terminal::Or),
                    nt(NonTerminal::BoolAnd),
                ],
            ),
            prod(NonTerminal::BoolExpr, vec![nt(NonTerminal::BoolAnd)]),
            // BoolAnd → BoolAnd and BoolNot | BoolNot   ← 直接左递归
            prod(
                NonTerminal::BoolAnd,
                vec![
                    nt(NonTerminal::BoolAnd),
                    t(Terminal::And),
                    nt(NonTerminal::BoolNot),
                ],
            ),
            prod(NonTerminal::BoolAnd, vec![nt(NonTerminal::BoolNot)]),
            // BoolNot → not BoolNot | BoolAtom
            prod(
                NonTerminal::BoolNot,
                vec![t(Terminal::Not), nt(NonTerminal::BoolNot)],
            ),
            prod(NonTerminal::BoolNot, vec![nt(NonTerminal::BoolAtom)]),
            // BoolAtom → ( BoolExpr ) | true | false | Relation
            prod(
                NonTerminal::BoolAtom,
                vec![
                    t(Terminal::LParen),
                    nt(NonTerminal::BoolExpr),
                    t(Terminal::RParen),
                ],
            ),
            prod(NonTerminal::BoolAtom, vec![t(Terminal::True)]),
            prod(NonTerminal::BoolAtom, vec![t(Terminal::False)]),
            prod(NonTerminal::BoolAtom, vec![nt(NonTerminal::Relation)]),
            // Relation → Variable RelOp Expr | num RelOp Expr
            prod(
                NonTerminal::Relation,
                vec![
                    nt(NonTerminal::Variable),
                    nt(NonTerminal::RelOp),
                    nt(NonTerminal::Expr),
                ],
            ),
            prod(
                NonTerminal::Relation,
                vec![
                    t(Terminal::Num),
                    nt(NonTerminal::RelOp),
                    nt(NonTerminal::Expr),
                ],
            ),
            // RelOp → == | != | < | <= | > | >=
            prod(NonTerminal::RelOp, vec![t(Terminal::Eq)]),
            prod(NonTerminal::RelOp, vec![t(Terminal::Ne)]),
            prod(NonTerminal::RelOp, vec![t(Terminal::Lt)]),
            prod(NonTerminal::RelOp, vec![t(Terminal::Le)]),
            prod(NonTerminal::RelOp, vec![t(Terminal::Gt)]),
            prod(NonTerminal::RelOp, vec![t(Terminal::Ge)]),
            // Variable → id
            prod(NonTerminal::Variable, vec![t(Terminal::Id)]),
        ];

        Self {
            name: "Simple 原文法（含直接左递归）".into(),
            start: NonTerminal::Program,
            productions,
        }
    }

    /// LL(1) 改造后文法：消除左递归，并将布尔条件与普通语句分层
    pub fn ll1() -> Self {
        let productions = vec![
            // Program → StmtList
            prod(NonTerminal::Program, vec![nt(NonTerminal::StmtList)]),
            // StmtList → Statement StmtListTail
            prod(
                NonTerminal::StmtList,
                vec![nt(NonTerminal::Statement), nt(NonTerminal::StmtListTail)],
            ),
            // StmtListTail → Statement StmtListTail | ε
            prod(
                NonTerminal::StmtListTail,
                vec![nt(NonTerminal::Statement), nt(NonTerminal::StmtListTail)],
            ),
            prod(NonTerminal::StmtListTail, vec![]), // ε
            // Statement → SimpleStmt ; | IfStmt | WhileStmt | Block
            prod(
                NonTerminal::Statement,
                vec![nt(NonTerminal::SimpleStmt), t(Terminal::Semicolon)],
            ),
            prod(NonTerminal::Statement, vec![nt(NonTerminal::IfStmt)]),
            prod(NonTerminal::Statement, vec![nt(NonTerminal::WhileStmt)]),
            prod(NonTerminal::Statement, vec![nt(NonTerminal::Block)]),
            // Block → { BlockBody }
            prod(
                NonTerminal::Block,
                vec![
                    t(Terminal::LBrace),
                    nt(NonTerminal::BlockBody),
                    t(Terminal::RBrace),
                ],
            ),
            // BlockBody → StmtList | ε
            prod(NonTerminal::BlockBody, vec![nt(NonTerminal::StmtList)]),
            prod(NonTerminal::BlockBody, vec![]),
            // SimpleStmt → DeclStmt | AssignStmt
            prod(NonTerminal::SimpleStmt, vec![nt(NonTerminal::DeclStmt)]),
            prod(NonTerminal::SimpleStmt, vec![nt(NonTerminal::AssignStmt)]),
            // DeclStmt → let Variable DeclInit
            prod(
                NonTerminal::DeclStmt,
                vec![
                    t(Terminal::Let),
                    nt(NonTerminal::Variable),
                    nt(NonTerminal::DeclInit),
                ],
            ),
            // DeclInit → = Expr | ε
            prod(
                NonTerminal::DeclInit,
                vec![t(Terminal::Assign), nt(NonTerminal::Expr)],
            ),
            prod(NonTerminal::DeclInit, vec![]),
            // AssignStmt → Variable = Expr
            prod(
                NonTerminal::AssignStmt,
                vec![
                    nt(NonTerminal::Variable),
                    t(Terminal::Assign),
                    nt(NonTerminal::Expr),
                ],
            ),
            // IfStmt → if ( BoolExpr ) Block ElsePart
            prod(
                NonTerminal::IfStmt,
                vec![
                    t(Terminal::If),
                    t(Terminal::LParen),
                    nt(NonTerminal::BoolExpr),
                    t(Terminal::RParen),
                    nt(NonTerminal::Block),
                    nt(NonTerminal::ElsePart),
                ],
            ),
            // ElsePart → else ElseBody | ε
            prod(
                NonTerminal::ElsePart,
                vec![t(Terminal::Else), nt(NonTerminal::ElseBody)],
            ),
            prod(NonTerminal::ElsePart, vec![]),
            // ElseBody → Block | IfStmt
            prod(NonTerminal::ElseBody, vec![nt(NonTerminal::Block)]),
            prod(NonTerminal::ElseBody, vec![nt(NonTerminal::IfStmt)]),
            // WhileStmt → while ( BoolExpr ) Block
            prod(
                NonTerminal::WhileStmt,
                vec![
                    t(Terminal::While),
                    t(Terminal::LParen),
                    nt(NonTerminal::BoolExpr),
                    t(Terminal::RParen),
                    nt(NonTerminal::Block),
                ],
            ),
            // Expr → Term ExprTail
            prod(
                NonTerminal::Expr,
                vec![nt(NonTerminal::Term), nt(NonTerminal::ExprTail)],
            ),
            // ExprTail → + Term ExprTail | - Term ExprTail | ε
            prod(
                NonTerminal::ExprTail,
                vec![
                    t(Terminal::Plus),
                    nt(NonTerminal::Term),
                    nt(NonTerminal::ExprTail),
                ],
            ),
            prod(
                NonTerminal::ExprTail,
                vec![
                    t(Terminal::Minus),
                    nt(NonTerminal::Term),
                    nt(NonTerminal::ExprTail),
                ],
            ),
            prod(NonTerminal::ExprTail, vec![]), // ε
            // Term → Factor TermTail
            prod(
                NonTerminal::Term,
                vec![nt(NonTerminal::Factor), nt(NonTerminal::TermTail)],
            ),
            // TermTail → * Factor TermTail | / Factor TermTail | ε
            prod(
                NonTerminal::TermTail,
                vec![
                    t(Terminal::Star),
                    nt(NonTerminal::Factor),
                    nt(NonTerminal::TermTail),
                ],
            ),
            prod(
                NonTerminal::TermTail,
                vec![
                    t(Terminal::Slash),
                    nt(NonTerminal::Factor),
                    nt(NonTerminal::TermTail),
                ],
            ),
            prod(NonTerminal::TermTail, vec![]), // ε
            // Factor → ( Expr ) | Variable | num
            prod(
                NonTerminal::Factor,
                vec![
                    t(Terminal::LParen),
                    nt(NonTerminal::Expr),
                    t(Terminal::RParen),
                ],
            ),
            prod(NonTerminal::Factor, vec![nt(NonTerminal::Variable)]),
            prod(NonTerminal::Factor, vec![t(Terminal::Num)]),
            // BoolExpr → BoolAnd BoolExprTail
            prod(
                NonTerminal::BoolExpr,
                vec![nt(NonTerminal::BoolAnd), nt(NonTerminal::BoolExprTail)],
            ),
            // BoolExprTail → or BoolAnd BoolExprTail | ε
            prod(
                NonTerminal::BoolExprTail,
                vec![
                    t(Terminal::Or),
                    nt(NonTerminal::BoolAnd),
                    nt(NonTerminal::BoolExprTail),
                ],
            ),
            prod(NonTerminal::BoolExprTail, vec![]), // ε
            // BoolAnd → BoolNot BoolAndTail
            prod(
                NonTerminal::BoolAnd,
                vec![nt(NonTerminal::BoolNot), nt(NonTerminal::BoolAndTail)],
            ),
            // BoolAndTail → and BoolNot BoolAndTail | ε
            prod(
                NonTerminal::BoolAndTail,
                vec![
                    t(Terminal::And),
                    nt(NonTerminal::BoolNot),
                    nt(NonTerminal::BoolAndTail),
                ],
            ),
            prod(NonTerminal::BoolAndTail, vec![]),
            // BoolNot → not BoolNot | BoolAtom
            prod(
                NonTerminal::BoolNot,
                vec![t(Terminal::Not), nt(NonTerminal::BoolNot)],
            ),
            prod(NonTerminal::BoolNot, vec![nt(NonTerminal::BoolAtom)]),
            // BoolAtom → ( BoolExpr ) | true | false | Relation
            prod(
                NonTerminal::BoolAtom,
                vec![
                    t(Terminal::LParen),
                    nt(NonTerminal::BoolExpr),
                    t(Terminal::RParen),
                ],
            ),
            prod(NonTerminal::BoolAtom, vec![t(Terminal::True)]),
            prod(NonTerminal::BoolAtom, vec![t(Terminal::False)]),
            prod(NonTerminal::BoolAtom, vec![nt(NonTerminal::Relation)]),
            // Relation → Variable RelOp Expr | num RelOp Expr
            prod(
                NonTerminal::Relation,
                vec![
                    nt(NonTerminal::Variable),
                    nt(NonTerminal::RelOp),
                    nt(NonTerminal::Expr),
                ],
            ),
            prod(
                NonTerminal::Relation,
                vec![
                    t(Terminal::Num),
                    nt(NonTerminal::RelOp),
                    nt(NonTerminal::Expr),
                ],
            ),
            // RelOp → == | != | < | <= | > | >=
            prod(NonTerminal::RelOp, vec![t(Terminal::Eq)]),
            prod(NonTerminal::RelOp, vec![t(Terminal::Ne)]),
            prod(NonTerminal::RelOp, vec![t(Terminal::Lt)]),
            prod(NonTerminal::RelOp, vec![t(Terminal::Le)]),
            prod(NonTerminal::RelOp, vec![t(Terminal::Gt)]),
            prod(NonTerminal::RelOp, vec![t(Terminal::Ge)]),
            // Variable → id
            prod(NonTerminal::Variable, vec![t(Terminal::Id)]),
        ];

        Self {
            name: "LL(1) 改造后文法".into(),
            start: NonTerminal::Program,
            productions,
        }
    }

    pub fn nullable_nonterminal(&self) -> HashSet<NonTerminal> {
        let mut nullable_set = HashSet::new();
        loop {
            let prev_len = nullable_set.len();

            for prod in &self.productions {
                if nullable_set.contains(&prod.lhs) {
                    continue;
                }
                if prod.rhs.is_empty() {
                    nullable_set.insert(prod.lhs);
                    continue;
                }

                let all_nullable = prod.rhs.iter().all(|symbol| match symbol {
                    Symbol::NonTerminal(nt) => nullable_set.contains(nt), // 每个终结符都必须为空
                    Symbol::Terminal(_) => false,                         // 右侧不可以有终结符
                    Symbol::Epsilon => true,
                });

                if all_nullable {
                    nullable_set.insert(prod.lhs);
                }
            }
            // 如果这轮没有新成员，说明收敛，退出
            if nullable_set.len() == prev_len {
                break;
            }
        }
        nullable_set
    }
}
