use crate::first_follow::build_parse_table;
use crate::grammar::Grammar;
use crate::lexer::Token;
use crate::symbol::{Symbol, Terminal};

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: Symbol,
    pub children: Vec<usize>,
}

pub fn parse(tokens: &[Token], grammar: &Grammar) -> Result<Vec<Node>, String> {
    if tokens.is_empty() {
        return Err("输入 token 序列为空".into());
    }

    let parse_table = build_parse_table(grammar)?;

    let mut nodes: Vec<Option<Node>> = vec![None];
    let mut stack: Vec<(Symbol, usize)> = vec![
        (Symbol::Terminal(Terminal::End), 0),
        (Symbol::NonTerminal(grammar.start), 0),
    ];
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
                let current = tokens[pos].kind;
                if term != current {
                    return Err(format!(
                        "语法错误: 期待终结符 `{}`, 实际读到 `{}` (offset {})",
                        term, current, tokens[pos].offset
                    ));
                }

                if term == Terminal::End {
                    break;
                }

                nodes[index] = Some(Node {
                    kind: Symbol::Terminal(term),
                    children: vec![],
                });
                pos += 1;
            }
            Symbol::NonTerminal(nt) => {
                let lookahead = tokens[pos].kind;
                let prod = parse_table.get(&(nt, lookahead)).ok_or_else(|| {
                    format!(
                        "语法错误: 非终结符 `{}` 在向前看符号 `{}` 下无可用产生式 (offset {})",
                        nt, lookahead, tokens[pos].offset
                    )
                })?;

                if prod.rhs.is_empty() {
                    nodes.push(Some(Node {
                        kind: Symbol::Epsilon,
                        children: vec![],
                    }));
                    let epsilon_id = nodes.len() - 1;
                    nodes[index] = Some(Node {
                        kind: Symbol::NonTerminal(nt),
                        children: vec![epsilon_id],
                    });
                } else {
                    let mut child_ids = Vec::with_capacity(prod.rhs.len());
                    for _ in &prod.rhs {
                        nodes.push(None);
                        child_ids.push(nodes.len() - 1);
                    }

                    nodes[index] = Some(Node {
                        kind: Symbol::NonTerminal(nt),
                        children: child_ids.clone(),
                    });

                    for (rhs_symbol, child_id) in
                        prod.rhs.iter().cloned().rev().zip(child_ids.into_iter().rev())
                    {
                        stack.push((rhs_symbol, child_id));
                    }
                }
            }
            Symbol::Epsilon => {
                nodes[index] = Some(Node {
                    kind: Symbol::Epsilon,
                    children: vec![],
                });
            }
        }
    }

    if pos != tokens.len() - 1 {
        return Err(format!(
            "语法错误: 输入未完全消费，当前位置仍为 `{}` (offset {})",
            tokens[pos].kind, tokens[pos].offset
        ));
    }

    nodes
        .into_iter()
        .enumerate()
        .map(|(index, node)| {
            node.ok_or_else(|| format!("语法树节点 {} 未被正确填充", index))
        })
        .collect()
}

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
    fn test_parse_error() {
        let tokens = tokenize("let x = ;").expect("tokenize should succeed");
        let err = parse(&tokens, &Grammar::ll1()).expect_err("parse should fail");
        assert!(err.contains("语法错误"));
    }
}
