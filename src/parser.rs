use crate::first_follow::build_parse_table;
use crate::grammar::Grammar;
use crate::lexer::Token;
use crate::symbol::{Symbol, Terminal};

// 语法树节点：保存当前符号以及其子节点在数组中的索引
#[derive(Debug, Clone)]
pub struct Node {
    pub kind: Symbol,
    pub children: Vec<usize>,
}

/// 基于 LL(1) 分析表做表驱动预测分析，并同步构造语法树
pub fn parse(tokens: &[Token], grammar: &Grammar) -> Result<Vec<Node>, String> {
    if tokens.is_empty() {
        return Err("输入 token 序列为空".into());
    }

    // 如果文法不是 LL(1)，这里会在构造分析表时直接报冲突
    let parse_table = build_parse_table(grammar)?;

    // 最后返回的语法树节点集合，先给根预留一个槽位
    let mut result_nodes: Vec<Option<Node>> = vec![None];

    // 分析栈中保存还需要处理的文法符号，以及该符号对应的语法树节点索引
    let mut stack: Vec<(Symbol, usize)> = vec![
        (Symbol::Terminal(Terminal::End), 0),
        (Symbol::NonTerminal(grammar.start), 0),
    ];
    // token 数组的游标
    let mut pos = 0usize;

    loop {
        let Some((symbol, index)) = stack.pop() else {
            return Err("分析栈提前耗尽，未在结束符位置成功结束".into());
        };

        if pos >= tokens.len() {
            return Err("输入在分析过程中提前结束".into());
        }

        match symbol {
            Symbol::Terminal(term) => {
                // 栈顶是终结符时，必须和当前输入精确匹配
                let current = tokens[pos].kind;
                if term != current {
                    return Err(format!(
                        "语法错误: 期待终结符 `{}`, 实际读到 `{}` (offset {})",
                        term, current, tokens[pos].offset
                    ));
                }

                // 结束符匹配成功，说明整个分析过程结束
                if term == Terminal::End {
                    break;
                }

                // 终结符节点没有子节点，直接回填到预留位置
                result_nodes[index] = Some(Node {
                    kind: Symbol::Terminal(term),
                    children: vec![],
                });
                pos += 1;
            }
            Symbol::NonTerminal(nt) => {
                let lookahead = tokens[pos].kind;
                // 使用 (非终结符, 向前看符号) 查询分析表，确定展开用的产生式
                let prod = parse_table.get(&(nt, lookahead)).ok_or_else(|| {
                    format!(
                        "语法错误: 非终结符 `{}` 在向前看符号 `{}` 下无可用产生式 (offset {})",
                        nt, lookahead, tokens[pos].offset
                    )
                })?;

                // ε 产生式不再继续展开，只补一个 ε 子节点保留树结构
                if prod.rhs.is_empty() {
                    result_nodes.push(Some(Node {
                        kind: Symbol::Epsilon,
                        children: vec![],
                    }));
                    let epsilon_id = result_nodes.len() - 1;
                    result_nodes[index] = Some(Node {
                        kind: Symbol::NonTerminal(nt),
                        children: vec![epsilon_id],
                    });
                } else {
                    // 先为右部每个符号预留节点槽位
                    let mut child_ids = Vec::with_capacity(prod.rhs.len());
                    for _ in &prod.rhs {
                        result_nodes.push(None);
                        child_ids.push(result_nodes.len() - 1);
                    }

                    // 父节点就地回填，并记录所有子节点索引
                    result_nodes[index] = Some(Node {
                        kind: Symbol::NonTerminal(nt),
                        children: child_ids.clone(),
                    });

                    // 栈是后进先出，所以右部需要逆序入栈，才能保证展开顺序仍是从左到右
                    for (rhs_symbol, child_id) in prod
                        .rhs
                        .iter()
                        .cloned()
                        .rev()
                        .zip(child_ids.into_iter().rev())
                    {
                        stack.push((rhs_symbol, child_id));
                    }
                }
            }
            Symbol::Epsilon => {
                // 当前实现不会主动把 ε 压栈，这个分支主要用于完整性
                result_nodes[index] = Some(Node {
                    kind: Symbol::Epsilon,
                    children: vec![],
                });
            }
        }
    }

    // 正常结束时，真实输入应该已经全部消费完，只剩自动补到末尾的 End
    if pos != tokens.len() - 1 {
        return Err(format!(
            "语法错误: 输入未完全消费，当前位置仍为 `{}` (offset {})",
            tokens[pos].kind, tokens[pos].offset
        ));
    }

    // 所有槽位都必须被成功填充，否则说明构树过程存在内部错误
    result_nodes
        .into_iter()
        .enumerate()
        .map(|(index, node)| node.ok_or_else(|| format!("语法树节点 {} 未被正确填充", index)))
        .collect()
}

/// 使用缩进形式打印语法树，便于人工检查推导结构
pub fn format_tree(nodes: &[Node]) -> String {
    fn dfs(nodes: &[Node], node_id: usize, depth: usize, out: &mut String) {
        out.push_str(&"  ".repeat(depth));
        out.push_str(&nodes[node_id].kind.to_string());
        out.push('\n');
        for &child in &nodes[node_id].children {
            dfs(nodes, child, depth + 1, out);
        }
    }

    let mut out = String::new();
    dfs(nodes, 0, 0, &mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::{format_tree, parse};
    use crate::grammar::Grammar;
    use crate::lexer::tokenize;

    fn parse_ok(input: &str) -> String {
        let tokens = tokenize(input).expect("tokenize should succeed");
        let tree = parse(&tokens, &Grammar::ll1()).expect("parse should succeed");
        format_tree(&tree)
    }

    #[test]
    fn test_parse_let_stmt() {
        let tree = parse_ok("let x = 1;");
        assert!(tree.contains("DeclStmt"));
        assert!(tree.contains("let"));
    }

    #[test]
    fn test_parse_assign_stmt() {
        let tree = parse_ok("x = 1 - 2 / 3;");
        assert!(tree.contains("AssignStmt"));
        assert!(tree.contains("/"));
    }

    #[test]
    fn test_parse_if_else_if_stmt() {
        let tree = parse_ok("if ( x < 10 ) { let y = 1; } else if ( x == 10 ) { let z; }");
        assert!(tree.contains("IfStmt"));
        assert!(tree.contains("ElsePart"));
        assert!(tree.contains("=="));
    }

    #[test]
    fn test_parse_while_stmt() {
        let tree = parse_ok("while ( x > 0 ) { x = x - 1; }");
        assert!(tree.contains("WhileStmt"));
        assert!(tree.contains(">"));
    }

    #[test]
    fn test_parse_block_stmt() {
        let tree = parse_ok("{ let a = 1; let b; }");
        assert!(tree.contains("Block"));
        assert!(tree.contains("BlockBody"));
    }

    #[test]
    fn test_parse_empty_block() {
        let tree = parse_ok("{ }");
        assert!(tree.contains("Block"));
        assert!(tree.contains("ε"));
    }

    #[test]
    fn test_parse_nested_if_while_with_bool_precedence() {
        let tree =
            parse_ok("if ( not false and x >= 1 ) { while ( x > 0 ) { if ( x == 1 ) { } } }");
        assert!(tree.contains("IfStmt"));
        assert!(tree.contains("WhileStmt"));
        assert!(tree.contains("not"));
        assert!(tree.contains("and"));
        assert!(tree.contains(">="));
    }

    #[test]
    fn test_parse_error() {
        let tokens = tokenize("let x = ;").expect("tokenize should succeed");
        let err = parse(&tokens, &Grammar::ll1()).expect_err("parse should fail");
        assert!(err.contains("语法错误"));
    }

    #[test]
    fn test_parse_missing_block_error() {
        let tokens = tokenize("if ( x < 1 ) let y = 1;").expect("tokenize should succeed");
        let err = parse(&tokens, &Grammar::ll1()).expect_err("parse should fail");
        assert!(err.contains("语法错误"));
    }
}
