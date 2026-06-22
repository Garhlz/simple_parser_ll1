use std::fmt;

/// 普通操作数：变量、数字、临时变量，或四元组输出中的占位符 `_`。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operand {
    Name(String),
    #[allow(dead_code)]
    Empty,
}

impl Operand {
    pub fn name(value: impl Into<String>) -> Self {
        Self::Name(value.into())
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Name(value) => write!(f, "{value}"),
            Operand::Empty => write!(f, "_"),
        }
    }
}

/// 跳转目标：要么等待回填，要么已经确定为某条四元组编号。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JumpTarget {
    Pending,
    #[allow(dead_code)]
    Target(usize),
}

impl fmt::Display for JumpTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JumpTarget::Pending => write!(f, "_"),
            JumpTarget::Target(index) => write!(f, "{index}"),
        }
    }
}

/// 类型安全的四元组内部表示，打印时仍保持 `(op, arg1, arg2, result)` 格式。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Quad {
    Binary {
        op: String,
        left: Operand,
        right: Operand,
        dest: Operand,
    },
    Assign {
        src: Operand,
        dest: Operand,
    },
    Jump {
        target: JumpTarget,
    },
    JumpIf {
        relop: String,
        left: Operand,
        right: Operand,
        target: JumpTarget,
    },
}

impl fmt::Display for Quad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Quad::Binary {
                op,
                left,
                right,
                dest,
            } => write!(f, "({op}, {left}, {right}, {dest})"),
            Quad::Assign { src, dest } => write!(f, "(=, {src}, _, {dest})"),
            Quad::Jump { target } => write!(f, "(j, _, _, {target})"),
            Quad::JumpIf {
                relop,
                left,
                right,
                target,
            } => write!(f, "(j{relop}, {left}, {right}, {target})"),
        }
    }
}

pub fn format_quads(codegen: &CodeGen) -> String {
    let mut out = String::from("Quadruples:\n");
    for (index, quad) in codegen.quads.iter().enumerate() {
        out.push_str(&format!("{index}: {quad}\n"));
    }
    out
}

pub fn format_bool_result(codegen: &CodeGen, attr: &BoolAttr) -> String {
    let mut out = format_quads(codegen);
    out.push('\n');
    out.push_str(&format!("TC = {:?}\n", attr.tc));
    out.push_str(&format!("FC = {:?}\n", attr.fc));
    out
}

/// 算术表达式属性：表达式结果所在位置。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExprAttr {
    pub place: Operand,
}

/// 布尔表达式属性：待回填的真链和假链。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoolAttr {
    // 如果这个布尔表达式为真，哪些跳转指令还不知道该跳到哪里
    pub tc: Vec<usize>,
    // 如果这个布尔表达式为假，哪些跳转指令还不知道该跳到哪里
    pub fc: Vec<usize>,
}

/// 四元组生成器：管理四元组序列和临时变量编号。
#[derive(Debug, Default)]
pub struct CodeGen {
    pub quads: Vec<Quad>,
    temp_count: usize,
}

impl CodeGen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_temp(&mut self) -> Operand {
        self.temp_count += 1;
        Operand::name(format!("t{}", self.temp_count))
    }

    pub fn next_quad(&self) -> usize {
        self.quads.len()
    }

    pub fn emit(&mut self, quad: Quad) -> usize {
        let index = self.next_quad();
        self.quads.push(quad);
        index
    }

    pub fn makelist(index: usize) -> Vec<usize> {
        vec![index]
    }

    #[allow(dead_code)]
    pub fn merge(mut left: Vec<usize>, right: Vec<usize>) -> Vec<usize> {
        left.extend(right);
        left
    }

    /// 将链中所有记录的四元组都回填（跳转地址）
    #[allow(dead_code)]
    pub fn backpatch(&mut self, list: &[usize], target: usize) -> Result<(), String> {
        for &index in list {
            let quad = self
                .quads
                .get_mut(index)
                .ok_or_else(|| format!("回填错误: 四元组编号 {index} 不存在"))?;
            match quad {
                Quad::Jump {
                    target: jump_target,
                }
                | Quad::JumpIf {
                    target: jump_target,
                    ..
                } => {
                    *jump_target = JumpTarget::Target(target);
                }
                Quad::Binary { .. } | Quad::Assign { .. } => {
                    return Err(format!("回填错误: 四元组编号 {index} 不是跳转指令"));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_temp_starts_from_t1() {
        let mut codegen = CodeGen::new();

        assert_eq!(codegen.new_temp(), Operand::name("t1"));
        assert_eq!(codegen.new_temp(), Operand::name("t2"));
    }

    #[test]
    fn test_emit_returns_quad_index() {
        let mut codegen = CodeGen::new();

        let index = codegen.emit(Quad::Binary {
            op: "+".into(),
            left: Operand::name("a"),
            right: Operand::name("b"),
            dest: Operand::name("t1"),
        });

        assert_eq!(index, 0);
        assert_eq!(codegen.next_quad(), 1);
        assert_eq!(codegen.quads[0].to_string(), "(+, a, b, t1)");
    }

    #[test]
    fn test_empty_operand_and_attrs() {
        assert_eq!(Operand::Empty.to_string(), "_");

        let expr = ExprAttr {
            place: Operand::name("t1"),
        };
        let bool_attr = BoolAttr {
            tc: vec![0],
            fc: vec![1],
        };

        assert_eq!(expr.place, Operand::name("t1"));
        assert_eq!(bool_attr.tc, vec![0]);
        assert_eq!(bool_attr.fc, vec![1]);
    }

    #[test]
    fn test_merge_and_backpatch_jump_targets() {
        let mut codegen = CodeGen::new();
        let tc = codegen.emit(Quad::JumpIf {
            relop: "<".into(),
            left: Operand::name("a"),
            right: Operand::name("b"),
            target: JumpTarget::Pending,
        });
        let fc = codegen.emit(Quad::Jump {
            target: JumpTarget::Pending,
        });

        let list = CodeGen::merge(CodeGen::makelist(tc), CodeGen::makelist(fc));
        codegen.backpatch(&list, 4).unwrap();

        assert_eq!(codegen.quads[0].to_string(), "(j<, a, b, 4)");
        assert_eq!(codegen.quads[1].to_string(), "(j, _, _, 4)");
    }

    #[test]
    fn test_backpatch_rejects_non_jump_quad() {
        let mut codegen = CodeGen::new();
        let index = codegen.emit(Quad::Assign {
            src: Operand::name("t1"),
            dest: Operand::name("a"),
        });

        let err = codegen.backpatch(&[index], 3).unwrap_err();
        assert!(err.contains("不是跳转指令"));
    }

    #[test]
    fn test_format_quads() {
        let mut codegen = CodeGen::new();
        codegen.emit(Quad::Binary {
            op: "*".into(),
            left: Operand::name("b"),
            right: Operand::name("c"),
            dest: Operand::name("t1"),
        });
        codegen.emit(Quad::Assign {
            src: Operand::name("t1"),
            dest: Operand::name("a"),
        });

        assert_eq!(
            format_quads(&codegen),
            "Quadruples:\n0: (*, b, c, t1)\n1: (=, t1, _, a)\n"
        );
    }

    #[test]
    fn test_format_bool_result() {
        let mut codegen = CodeGen::new();
        let tc = codegen.emit(Quad::JumpIf {
            relop: "<".into(),
            left: Operand::name("a"),
            right: Operand::name("b"),
            target: JumpTarget::Pending,
        });
        let fc = codegen.emit(Quad::Jump {
            target: JumpTarget::Pending,
        });
        let attr = BoolAttr {
            tc: vec![tc],
            fc: vec![fc],
        };

        assert_eq!(
            format_bool_result(&codegen, &attr),
            "Quadruples:\n0: (j<, a, b, _)\n1: (j, _, _, _)\n\nTC = [0]\nFC = [1]\n"
        );
    }
}
