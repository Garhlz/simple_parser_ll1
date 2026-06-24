mod codegen;
mod error;
mod lexer;
mod sd_parser;
mod symbol;

use codegen::{format_bool_result, format_quads};
use error::{CompileError, CompileResult};
use lexer::{format_tokens, tokenize};
use sd_parser::{parse_assignment, parse_bool, parse_program};

fn main() {
    match run_cli(std::env::args().skip(1)) {
        Ok(output) => print!("{output}"),
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}

fn run_cli<I>(args: I) -> CompileResult<String>
where
    I: IntoIterator<Item = String>,
{
    let mut args = args.into_iter();
    let Some(mode) = args.next() else {
        return Err(CompileError::cli(usage()));
    };
    let input = args.collect::<Vec<_>>().join(" ");

    if input.is_empty() {
        return Err(CompileError::cli(format!(
            "错误: 缺少输入源码\n{}",
            usage()
        )));
    }

    match mode.as_str() {
        "program" => render_program(&input),
        "assign" => render_assign(&input),
        "bool" => render_bool(&input),
        "tokens" => render_tokens(&input),
        _ => Err(CompileError::cli(format!(
            "错误: 未知模式 `{mode}`\n{}",
            usage()
        ))),
    }
}

fn render_tokens(input: &str) -> CompileResult<String> {
    tokenize(input).map(|tokens| format!("{}\n", format_tokens(&tokens)))
}

fn render_assign(input: &str) -> CompileResult<String> {
    tokenize(input)
        .and_then(|tokens| parse_assignment(&tokens))
        .map(|codegen| format_quads(&codegen))
}

fn render_program(input: &str) -> CompileResult<String> {
    tokenize(input)
        .and_then(|tokens| parse_program(&tokens))
        .map(|codegen| format_quads(&codegen))
}

fn render_bool(input: &str) -> CompileResult<String> {
    tokenize(input).and_then(|tokens| {
        parse_bool(&tokens).map(|output| format_bool_result(&output.codegen, &output.attr))
    })
}

fn usage() -> String {
    [
        "用法:",
        "  cargo run -- program \"a = b + c; x = a * d;\"",
        "  cargo run -- assign \"a = b + c * e / g\"",
        "  cargo run -- bool \"a < b\"",
        "  cargo run -- tokens \"a = b + c * e / g;\"",
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_program_mode() {
        assert_eq!(
            render_program("a = b + c; x = a * d;").unwrap(),
            concat!(
                "Quadruples:\n",
                "0: (+, b, c, t1)\n",
                "1: (=, t1, _, a)\n",
                "2: (*, a, d, t2)\n",
                "3: (=, t2, _, x)\n",
            )
        );
    }

    #[test]
    fn test_render_assign_mode() {
        assert_eq!(
            render_assign("a = b + c * e / g").unwrap(),
            concat!(
                "Quadruples:\n",
                "0: (*, c, e, t1)\n",
                "1: (/, t1, g, t2)\n",
                "2: (+, b, t2, t3)\n",
                "3: (=, t3, _, a)\n",
            )
        );
    }

    #[test]
    fn test_render_bool_mode() {
        assert_eq!(
            render_bool("a < b").unwrap(),
            concat!(
                "Quadruples:\n",
                "0: (j<, a, b, _)\n",
                "1: (j, _, _, _)\n",
                "\n",
                "TC = [0]\n",
                "FC = [1]\n",
            )
        );
    }

    #[test]
    fn test_render_tokens_mode() {
        assert_eq!(render_tokens("a = b + c;").unwrap(), "id = id + id ; #\n");
    }

    #[test]
    fn test_run_cli_dispatches_program() {
        let output = run_cli([
            "program".to_string(),
            "if (a < b) { x = y + z; }".to_string(),
        ])
        .unwrap();

        assert!(output.contains("0: (j<, a, b, 2)"));
        assert!(output.contains("3: (=, t1, _, x)"));
    }

    #[test]
    fn test_run_cli_reports_missing_input() {
        let err = run_cli(["program".to_string()]).unwrap_err();

        assert!(err.to_string().contains("缺少输入源码"));
        assert!(err.to_string().contains("cargo run -- program"));
    }

    #[test]
    fn test_run_cli_reports_unknown_mode() {
        let err = run_cli(["expr".to_string(), "a = b;".to_string()]).unwrap_err();

        assert!(err.to_string().contains("未知模式 `expr`"));
        assert!(err.to_string().contains("cargo run -- program"));
    }
}
