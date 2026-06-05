use crate::grammar::{Grammar, Production};
use crate::symbol::{NonTerminal, Symbol, Terminal};
use std::collections::{HashMap, HashSet};

pub fn build_parse_table(
    grammar: &Grammar,
) -> Result<HashMap<(NonTerminal, Terminal), Production>, String> {
    let mut parse_table: HashMap<(NonTerminal, Terminal), Production> = HashMap::new();

    let first_set_map = get_first_set(grammar);
    let follow_set_map = get_follow_set(grammar, &first_set_map);

    // 对于每条产生式
    for prod in &grammar.productions {
        // 求出 rhs 整体的first set
        let rhs_first = get_rhs_first(prod, &first_set_map);

        for term in rhs_first.iter().filter_map(|s| {
            if let Symbol::Terminal(term) = s {
                // 断言rhs_first中的终结符，非空
                Some(*term)
            } else {
                None
            }
        }) {
            let key = (prod.lhs, term);
            // 要求，如果LL1冲突需要报错
            if let Some(existing) = parse_table.get(&key) {
                return Err(format!(
                    "LL(1) 冲突: 表项 ({}, {}) 同时对应 `{}` 和 `{}`",
                    key.0, key.1, existing, prod
                ));
            }
            parse_table.insert(key, prod.clone());
        }

        // 如果右侧整体的first集合中有epsilon
        if rhs_first.contains(&Symbol::Epsilon) {
            let lhs_follow = follow_set_map.get(&prod.lhs).unwrap();
            for term in lhs_follow.iter().filter_map(|s| {
                if let Symbol::Terminal(term) = s {
                    Some(*term)
                } else {
                    None
                }
            }) {
                let key = (prod.lhs, term);
                if let Some(existing) = parse_table.get(&key) {
                    return Err(format!(
                        "LL(1) 冲突: 表项 ({}, {}) 同时对应 `{}` 和 `{}`",
                        key.0, key.1, existing, prod
                    ));
                }
                parse_table.insert(key, prod.clone());
            }
        }
    }
    Ok(parse_table)
}

/// 求出rhs整体的first集合
pub fn get_rhs_first(
    prod: &Production,
    first_set_map: &HashMap<NonTerminal, HashSet<Symbol>>,
) -> HashSet<Symbol> {
    let mut rhs_first = HashSet::new();
    for symbol in &prod.rhs {
        match symbol {
            Symbol::Terminal(_) | Symbol::Epsilon => {
                rhs_first.insert(symbol.clone());
                break;
            }
            Symbol::NonTerminal(nt) => {
                let cur_first = first_set_map.get(nt).unwrap();
                rhs_first.extend(
                    cur_first
                        .iter()
                        .filter(|s| !matches!(s, Symbol::Epsilon))
                        .cloned(),
                );
                // 可推出空，可以继续迭代
                if cur_first.contains(&Symbol::Epsilon) {
                    continue;
                }
                // 否则结束
                break;
            }
        }
    }
    // 如果rhs 都是可为空的非终结符， 整体first集插入epsilon
    let rhs_all_nullable = prod.rhs.iter().all(|s| matches!(s, Symbol::NonTerminal(nt) if first_set_map.get(nt).unwrap().contains(&Symbol::Epsilon)));
    if rhs_all_nullable {
        rhs_first.insert(Symbol::Epsilon);
    }
    rhs_first
}

pub fn get_first_set(grammar: &Grammar) -> HashMap<NonTerminal, HashSet<Symbol>> {
    let mut first_set_map: HashMap<NonTerminal, HashSet<Symbol>> = HashMap::new();

    // 初始化
    for prod in &grammar.productions {
        first_set_map.entry(prod.lhs).or_default();
    }
    loop {
        let mut changed = false;
        for prod in &grammar.productions {
            let mut lhs_first = first_set_map.get(&prod.lhs).cloned().unwrap();
            let len_prev = lhs_first.len();
            // 右侧是空的，直接写入
            if prod.rhs.is_empty() {
                lhs_first.insert(Symbol::Epsilon);
            } else {
                for i in 0..prod.rhs.len() {
                    match &prod.rhs[i] {
                        Symbol::NonTerminal(nt) => {
                            let cur_first = first_set_map.get(nt).unwrap();
                            lhs_first.extend(
                                cur_first
                                    .iter()
                                    .filter(|s| !matches!(s, Symbol::Epsilon))
                                    .cloned(),
                            );
                            // 可为空，则继续看下一个符号
                            if cur_first.contains(&Symbol::Epsilon) {
                                continue;
                            }
                            break;
                        }
                        whole @ Symbol::Terminal(_) => {
                            lhs_first.insert(whole.clone());
                            break;
                        }
                        Symbol::Epsilon => unreachable!(),
                    }
                }
                let rhs_all_nullable = prod.rhs.iter().all(|s| {
                    matches!(s, Symbol::NonTerminal(nt)
                    if first_set_map.get(nt).unwrap().contains(&Symbol::Epsilon))
                });
                if rhs_all_nullable {
                    lhs_first.insert(Symbol::Epsilon);
                }
            }

            if len_prev != lhs_first.len() {
                changed = true;
            }

            first_set_map.insert(prod.lhs, lhs_first);
        }
        if !changed {
            break;
        }
    }
    first_set_map
}

/// 获取所有非终结符的follow集合
pub fn get_follow_set(
    grammar: &Grammar,
    first_set: &HashMap<NonTerminal, HashSet<Symbol>>,
) -> HashMap<NonTerminal, HashSet<Symbol>> {
    let mut follow_set = HashMap::new();

    // 初始化
    for prod in &grammar.productions {
        follow_set.entry(prod.lhs).or_default();
        // 确保每个右部出现的非终结符有 entry
        for symbol in &prod.rhs {
            if let Symbol::NonTerminal(nt) = symbol {
                follow_set.entry(*nt).or_default();
            }
        }
    }

    // 开始符号的follow集中添加结束符号
    let start_follow: &mut HashSet<Symbol> = follow_set.get_mut(&grammar.start).unwrap();
    start_follow.insert(Symbol::Terminal(Terminal::End));

    let nullable_set = Grammar::nullable_nonterminal(grammar);

    loop {
        let mut changed = false;

        for prod in &grammar.productions {
            let lhs_follow = follow_set.get(&prod.lhs).unwrap().clone();
            for i in (0..prod.rhs.len()).rev() {
                if let Symbol::NonTerminal(nt) = prod.rhs[i] {
                    let nt_follow = follow_set.get_mut(&nt).unwrap();
                    let len_prev = nt_follow.len();
                    // 记录当前rhs中的非终的follow集的大小
                    if i == prod.rhs.len() - 1 {
                        // 末尾，添加lhs的follow集合
                        nt_follow.extend(lhs_follow.iter().cloned());
                    } else {
                        for j in i + 1..prod.rhs.len() {
                            match &prod.rhs[j] {
                                Symbol::NonTerminal(nt_nxt) => {
                                    let nxt_first = first_set.get(nt_nxt).unwrap();
                                    nt_follow.extend(
                                        nxt_first
                                            .iter()
                                            .filter(|s| !matches!(s, Symbol::Epsilon))
                                            .cloned(),
                                    );
                                    if nxt_first.contains(&Symbol::Epsilon) {
                                        continue;
                                    }
                                    break;
                                }
                                Symbol::Terminal(_) => {
                                    nt_follow.insert(prod.rhs[j].clone());
                                    break;
                                }
                                Symbol::Epsilon => unreachable!(),
                            }
                        }
                    }
                    // 只需用高阶函数扫一遍即可，没有必要在循环中维护这个量
                    let suffix_all_nullable = prod.rhs[i + 1..].iter().all(
                        |s| matches!(s, Symbol::NonTerminal(nt1) if nullable_set.contains(nt1)),
                    );

                    if suffix_all_nullable {
                        nt_follow.extend(lhs_follow.iter().cloned());
                    }

                    if len_prev != nt_follow.len() {
                        changed = true;
                    }
                }
            }
        }
        // 不动点迭代模式，迭代到没有变化为止
        if !changed {
            break;
        }
    }

    follow_set
}

fn format_symbol_set(set: &HashSet<Symbol>) -> String {
    let mut items: Vec<String> = set.iter().map(ToString::to_string).collect();
    items.sort();
    format!("{{ {} }}", items.join(", "))
}

pub fn format_first_sets(grammar: &Grammar) -> String {
    let first = get_first_set(grammar);
    let mut lines = Vec::new();
    for &nt in NonTerminal::all() {
        if let Some(set) = first.get(&nt) {
            lines.push(format!("FIRST({}) = {}", nt, format_symbol_set(set)));
        }
    }
    lines.join("\n")
}

pub fn format_follow_sets(grammar: &Grammar) -> String {
    let first = get_first_set(grammar);
    let follow = get_follow_set(grammar, &first);
    let mut lines = Vec::new();
    for &nt in NonTerminal::all() {
        if let Some(set) = follow.get(&nt) {
            lines.push(format!("FOLLOW({}) = {}", nt, format_symbol_set(set)));
        }
    }
    lines.join("\n")
}

pub fn format_parse_table(grammar: &Grammar) -> Result<String, String> {
    let table = build_parse_table(grammar)?;
    let mut lines = Vec::new();
    lines.push("非终结符 | 向前看符号 | 产生式".to_string());
    lines.push("-".repeat(48));
    for &nt in NonTerminal::all() {
        for &term in Terminal::all() {
            if let Some(prod) = table.get(&(nt, term)) {
                lines.push(format!("{:<12} | {:<12} | {}", nt, term, prod));
            }
        }
    }
    Ok(lines.join("\n"))
}
