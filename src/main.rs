mod codegen;
mod lexer;
mod sd_parser;
mod symbol;

use codegen::{format_bool_result, format_quads};
use lexer::{format_tokens, tokenize};
use sd_parser::{parse_assignment, parse_bool, parse_program};

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(mode) = args.next() else {
        print_usage();
        return;
    };
    let input = args.collect::<Vec<_>>().join(" ");

    if input.is_empty() {
        eprintln!("错误: 缺少输入源码");
        print_usage();
        return;
    }

    match mode.as_str() {
        "assign" => run_assign(&input),
        "program" => run_program(&input),
        "bool" => run_bool(&input),
        "tokens" => run_tokens(&input),
        _ => {
            eprintln!("错误: 未知模式 `{mode}`");
            print_usage();
        }
    }
}

fn run_tokens(input: &str) {
    match tokenize(input) {
        Ok(tokens) => println!("{}", format_tokens(&tokens)),
        Err(err) => eprintln!("{err}"),
    }
}

fn run_assign(input: &str) {
    match tokenize(input).and_then(|tokens| parse_assignment(&tokens)) {
        Ok(codegen) => print!("{}", format_quads(&codegen)),
        Err(err) => eprintln!("{err}"),
    }
}

fn run_program(input: &str) {
    match tokenize(input).and_then(|tokens| parse_program(&tokens)) {
        Ok(codegen) => print!("{}", format_quads(&codegen)),
        Err(err) => eprintln!("{err}"),
    }
}

fn run_bool(input: &str) {
    match tokenize(input).and_then(|tokens| parse_bool(&tokens)) {
        Ok(output) => print!("{}", format_bool_result(&output.codegen, &output.attr)),
        Err(err) => eprintln!("{err}"),
    }
}

fn print_usage() {
    println!("用法:");
    println!("  cargo run -- assign \"a = b + c * e / g;\"");
    println!("  cargo run -- program \"a = b + c; x = a * d;\"");
    println!("  cargo run -- bool \"a < b\"");
    println!("  cargo run -- tokens \"a = b + c * e / g;\"");
}
