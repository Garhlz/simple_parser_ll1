# TODO

## simple_codegen

* [x] 移除实验五不再需要的 LL(1) 展示入口、FIRST/FOLLOW 和预测分析表代码。
* [x] 新增 `src/codegen.rs`，实现四元组、临时变量和回填工具。
* [x] 新增 `src/sd_parser.rs`，实现算术赋值语句的语法制导翻译 parser。
* [x] 接入 `main.rs` 的 `assign` 模式，输出算术赋值语句四元组。
* [x] 扩展 `src/sd_parser.rs`，实现关系表达式跳转四元组和 `TC/FC` 链。
* [x] 接入 `main.rs` 的 `bool` 模式，输出关系表达式四元组和链信息。
* [x] 扩展 `src/sd_parser.rs`，实现 `and`、`or`、`not` 短路翻译。
* [x] 为算术赋值、括号、布尔关系、`and`、`or`、`not` 补充测试。
* [x] 支持布尔表达式括号分组，如 `(a < b or c > d) and e != f`。
* [x] 支持语句列表和代码块，接入 `main.rs` 的 `program` 模式。
* [x] 选做：支持 `if` / `if-else` 条件语句回填。
* [x] 扩展：支持 `while` 循环语句回填。
* [x] 移除早期只生成语法树的 `rd_parser.rs`，保留 `sd_parser.rs` 作为 codegen 主实现。
* [x] 支持布尔字面量 `true` / `false`，以及带括号算术表达式的关系判断。
* [x] 将命令行错误路径改为非零退出，并切换到兼容性更稳的 Rust 2021 edition。
* [x] 引入统一错误类型，结构化区分词法、语法、回填和命令行错误。
* [x] `assign` 模式兼容实验样例中的无分号单条赋值输入。
