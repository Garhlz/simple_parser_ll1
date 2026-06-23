use std::fmt;

pub type CompileResult<T> = Result<T, CompileError>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CompileError {
    Lex {
        offset: usize,
        message: String,
    },
    Parse {
        offset: Option<usize>,
        message: String,
    },
    Codegen {
        message: String,
    },
    Cli {
        message: String,
    },
}

impl CompileError {
    pub fn lex(offset: usize, message: impl Into<String>) -> Self {
        Self::Lex {
            offset,
            message: message.into(),
        }
    }

    pub fn parse(offset: usize, message: impl Into<String>) -> Self {
        Self::Parse {
            offset: Some(offset),
            message: message.into(),
        }
    }

    pub fn parse_without_offset(message: impl Into<String>) -> Self {
        Self::Parse {
            offset: None,
            message: message.into(),
        }
    }

    pub fn codegen(message: impl Into<String>) -> Self {
        Self::Codegen {
            message: message.into(),
        }
    }

    pub fn cli(message: impl Into<String>) -> Self {
        Self::Cli {
            message: message.into(),
        }
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompileError::Lex { offset, message } => {
                write!(f, "词法错误 (offset {offset}): {message}")
            }
            CompileError::Parse {
                offset: Some(offset),
                message,
            } => {
                write!(f, "语法制导翻译错误: {message} (offset {offset})")
            }
            CompileError::Parse {
                offset: None,
                message,
            } => write!(f, "语法制导翻译错误: {message}"),
            CompileError::Codegen { message } => write!(f, "回填错误: {message}"),
            CompileError::Cli { message } => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for CompileError {}
