use std::fmt;

// ============================================================
// 终结符：词法分析器产出的 token 类型
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Terminal {
    // 标识符和字面量类别
    Id,
    Num,
    // 关键字
    Let,
    True,
    False,
    If,
    Else,
    While,
    And,
    Or,
    Not,
    // 运算符和分隔符
    Plus,      // +
    Minus,     // -
    Star,      // *
    Slash,     // /
    Assign,    // =
    Eq,        // ==
    Ne,        // !=
    Lt,        // <
    Le,        // <=
    Gt,        // >
    Ge,        // >=
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    Semicolon, // ;
    End,       // #
}

impl fmt::Display for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Terminal::Id => write!(f, "id"),
            Terminal::Num => write!(f, "num"),
            Terminal::Let => write!(f, "let"),
            Terminal::True => write!(f, "true"),
            Terminal::False => write!(f, "false"),
            Terminal::If => write!(f, "if"),
            Terminal::Else => write!(f, "else"),
            Terminal::While => write!(f, "while"),
            Terminal::And => write!(f, "and"),
            Terminal::Or => write!(f, "or"),
            Terminal::Not => write!(f, "not"),
            Terminal::Plus => write!(f, "+"),
            Terminal::Minus => write!(f, "-"),
            Terminal::Star => write!(f, "*"),
            Terminal::Slash => write!(f, "/"),
            Terminal::Assign => write!(f, "="),
            Terminal::Eq => write!(f, "=="),
            Terminal::Ne => write!(f, "!="),
            Terminal::Lt => write!(f, "<"),
            Terminal::Le => write!(f, "<="),
            Terminal::Gt => write!(f, ">"),
            Terminal::Ge => write!(f, ">="),
            Terminal::LParen => write!(f, "("),
            Terminal::RParen => write!(f, ")"),
            Terminal::LBrace => write!(f, "{{"),
            Terminal::RBrace => write!(f, "}}"),
            Terminal::Semicolon => write!(f, ";"),
            Terminal::End => write!(f, "#"),
        }
    }
}

// ============================================================
// 非终结符：文法的语法变量（全语义化命名）
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NonTerminal {
    // 程序结构
    Program,      // P:   程序
    StmtList,     // L:   语句块
    StmtListTail, // L':  语句块尾部
    Block,        //      代码块
    BlockBody,    //      代码块内容

    // 语句
    Statement,  // S:   语句
    SimpleStmt, // A:   简单语句（声明或赋值）
    IfStmt,     // C:   if-else 条件语句
    WhileStmt,  //      while 循环
    ElsePart,   //      else 部分
    ElseBody,   //      else 后的主体（block 或 if）
    DeclStmt,   //      声明语句
    DeclInit,   //      声明初始化部分
    AssignStmt, //      赋值语句

    // 算术表达式
    Expr,     // E:   算术表达式
    ExprTail, // E':  算术表达式尾部（处理 + -）
    Term,     // T:   项
    TermTail, // T':  项尾部（处理 * /）
    Factor,   // F:   因子

    // 布尔表达式
    BoolExpr,     // B:   布尔表达式
    BoolExprTail, // B':  布尔表达式尾部（处理 or）
    BoolAnd,      //      and 层
    BoolAndTail,  //      and 尾部
    BoolNot,      //      not 层
    BoolAtom,     //      布尔基本项
    Relation,     //      比较表达式
    RelOp,        //      比较运算符

    // 其他
    Variable, // V:   变量
}

impl fmt::Display for NonTerminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            NonTerminal::Program => "Program",
            NonTerminal::StmtList => "StmtList",
            NonTerminal::StmtListTail => "StmtListTail",
            NonTerminal::Block => "Block",
            NonTerminal::BlockBody => "BlockBody",
            NonTerminal::Statement => "Statement",
            NonTerminal::SimpleStmt => "SimpleStmt",
            NonTerminal::IfStmt => "IfStmt",
            NonTerminal::WhileStmt => "WhileStmt",
            NonTerminal::ElsePart => "ElsePart",
            NonTerminal::ElseBody => "ElseBody",
            NonTerminal::DeclStmt => "DeclStmt",
            NonTerminal::DeclInit => "DeclInit",
            NonTerminal::AssignStmt => "AssignStmt",
            NonTerminal::Expr => "Expr",
            NonTerminal::ExprTail => "ExprTail",
            NonTerminal::Term => "Term",
            NonTerminal::TermTail => "TermTail",
            NonTerminal::Factor => "Factor",
            NonTerminal::BoolExpr => "BoolExpr",
            NonTerminal::BoolExprTail => "BoolExprTail",
            NonTerminal::BoolAnd => "BoolAnd",
            NonTerminal::BoolAndTail => "BoolAndTail",
            NonTerminal::BoolNot => "BoolNot",
            NonTerminal::BoolAtom => "BoolAtom",
            NonTerminal::Relation => "Relation",
            NonTerminal::RelOp => "RelOp",
            NonTerminal::Variable => "Variable",
        };
        write!(f, "{s}")
    }
}

// ============================================================
// 顶层符号：终结符 | 非终结符 | ε（空串）
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Symbol {
    Terminal(Terminal),
    NonTerminal(NonTerminal),
    Epsilon,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::Terminal(t) => write!(f, "{t}"),
            Symbol::NonTerminal(nt) => write!(f, "{nt}"),
            Symbol::Epsilon => write!(f, "ε"),
        }
    }
}

impl Terminal {
    pub const fn all() -> &'static [Terminal] {
        &[
            Terminal::Id,
            Terminal::Num,
            Terminal::Let,
            Terminal::True,
            Terminal::False,
            Terminal::If,
            Terminal::Else,
            Terminal::While,
            Terminal::And,
            Terminal::Or,
            Terminal::Not,
            Terminal::Plus,
            Terminal::Minus,
            Terminal::Star,
            Terminal::Slash,
            Terminal::Assign,
            Terminal::Eq,
            Terminal::Ne,
            Terminal::Lt,
            Terminal::Le,
            Terminal::Gt,
            Terminal::Ge,
            Terminal::LParen,
            Terminal::RParen,
            Terminal::LBrace,
            Terminal::RBrace,
            Terminal::Semicolon,
            Terminal::End,
        ]
    }
}

impl NonTerminal {
    pub const fn all() -> &'static [NonTerminal] {
        &[
            NonTerminal::Program,
            NonTerminal::StmtList,
            NonTerminal::StmtListTail,
            NonTerminal::Block,
            NonTerminal::BlockBody,
            NonTerminal::Statement,
            NonTerminal::SimpleStmt,
            NonTerminal::IfStmt,
            NonTerminal::WhileStmt,
            NonTerminal::ElsePart,
            NonTerminal::ElseBody,
            NonTerminal::DeclStmt,
            NonTerminal::DeclInit,
            NonTerminal::AssignStmt,
            NonTerminal::Expr,
            NonTerminal::ExprTail,
            NonTerminal::Term,
            NonTerminal::TermTail,
            NonTerminal::Factor,
            NonTerminal::BoolExpr,
            NonTerminal::BoolExprTail,
            NonTerminal::BoolAnd,
            NonTerminal::BoolAndTail,
            NonTerminal::BoolNot,
            NonTerminal::BoolAtom,
            NonTerminal::Relation,
            NonTerminal::RelOp,
            NonTerminal::Variable,
        ]
    }
}
