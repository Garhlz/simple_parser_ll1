mod first_follow;
mod grammar;
mod lexer;
mod parser;
mod rd_parser;
mod symbol;

use first_follow::{format_first_sets, format_follow_sets, format_parse_table};
use grammar::Grammar;
use lexer::{format_tokens, tokenize};
use parser::{format_tree, parse};
use rd_parser::{format_tree as format_rd_tree, parse as parse_rd};

fn print_section(title: &str) {
    println!("\n=== {title} ===\n");
}

fn print_subsection(index: usize, input: &str) {
    println!("[样例 {index}] {input}");
}

fn main() {
    let ll1 = Grammar::ll1();

    // ── 文法打印 ──
    print_section("原始文法");
    println!("{}\n", Grammar::original());
    print_section("LL(1) 文法");
    println!("{}\n", ll1);

    print_section("FIRST 集");
    println!("{}\n", format_first_sets(&ll1));

    print_section("FOLLOW 集");
    println!("{}\n", format_follow_sets(&ll1));

    print_section("LL(1) 分析表");
    match format_parse_table(&ll1) {
        Ok(table) => println!("{}\n", table),
        Err(e) => println!("构造分析表失败: {e}\n"),
    }

    // ── 词法分析测试 ──
    let test_inputs = [
        "let x = 1;",
        "x = 1 - 2 / 3;",
        "if ( x < 10 or false ) { let y = 1; } else if ( x == 10 ) { let z; }",
        "while ( x > 0 ) { x = x - 1; }",
        "{ let a = 1; let b; }",
    ];

    print_section("词法分析");
    for (index, input) in test_inputs.iter().enumerate() {
        print_subsection(index + 1, input);
        match tokenize(input) {
            Ok(tokens) => {
                println!("Token: {}\n", format_tokens(&tokens));
            }
            Err(e) => println!("错误: {e}\n"),
        }
    }

    print_section("LL(1) 预测分析");
    for (index, input) in test_inputs.iter().enumerate() {
        print_subsection(index + 1, input);
        match tokenize(input) {
            Ok(tokens) => match parse(&tokens, &ll1) {
                Ok(tree) => {
                    println!("分析结果: 接受");
                    println!("语法树:\n{}", format_tree(&tree));
                }
                Err(e) => println!("分析结果: 错误\n{e}\n"),
            },
            Err(e) => println!("词法错误: {e}\n"),
        }
    }

    print_section("Pratt 递归下降分析");
    for (index, input) in test_inputs.iter().enumerate() {
        print_subsection(index + 1, input);
        match tokenize(input) {
            Ok(tokens) => match parse_rd(&tokens) {
                Ok(output) => {
                    println!("分析结果: 接受");
                    println!("调用轨迹:");
                    for line in &output.trace {
                        println!("  {line}");
                    }
                    println!("语法树:\n{}", format_rd_tree(&output.nodes, output.root));
                }
                Err(e) => println!("分析结果: 错误\n{e}\n"),
            },
            Err(e) => println!("词法错误: {e}\n"),
        }
    }
}
