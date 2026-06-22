# simple_parser_ll1

编译原理实验项目，当前分支 `simple_codegen` 面向实验五：语法制导翻译与中间代码生成。

## 当前目标

在已有词法分析和递归下降 / Pratt 表达式分析基础上，实现一个小型中间代码生成程序，支持：

* 算术表达式与赋值语句四元组生成；
* 布尔表达式跳转四元组生成；
* `TC` / `FC` 链与回填机制；
* 条件语句四元组生成作为选做扩展。

详细要求见 [docs/requirement.md](docs/requirement.md)，当前待办见 [docs/TODO.md](docs/TODO.md)。

## 当前状态

已保留并整理：

* `src/lexer.rs`：tokenizer，支持实验所需 token；
* `src/symbol.rs`：终结符定义；
* `src/rd_parser.rs`：递归下降 / Pratt 分析实现，作为后续 `sd_parser.rs` 的参考。

已移除旧实验中不再需要的 LL(1) 展示流程，包括 FIRST/FOLLOW、预测分析表和表驱动 parser。

## 运行

当前 `main.rs` 仅保留 tokenizer 预览入口，后续会接入中间代码生成：

```bash
cargo run -- "a = b + c * e / g;"
```

示例输出：

```text
id = id + id * id / id ; #
```

## 测试

```bash
cargo test
```

## 后续模块

计划新增：

* `src/codegen.rs`：四元组、临时变量、`makelist`、`merge`、`backpatch`；
* `src/sd_parser.rs`：语法制导翻译 parser；
* `main.rs`：命令行入口，输出赋值语句和布尔表达式四元组。
