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
