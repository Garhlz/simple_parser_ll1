mod lexer;
mod symbol;

use lexer::{format_tokens, tokenize};

fn main() {
    let input = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    if input.is_empty() {
        println!("simple_codegen: 中间代码生成入口待实现。");
        println!("当前保留 tokenizer，可运行: cargo run -- \"a = b + c * e / g;\"");
        return;
    }

    match tokenize(&input) {
        Ok(tokens) => println!("{}", format_tokens(&tokens)),
        Err(err) => eprintln!("{err}"),
    }
}
