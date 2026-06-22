# TODO

## simple_codegen

* [x] 移除实验五不再需要的 LL(1) 展示入口、FIRST/FOLLOW 和预测分析表代码。
* [ ] 新增 `src/codegen.rs`，实现四元组、临时变量和回填工具。
* [ ] 新增 `src/sd_parser.rs`，实现语法制导翻译 parser。
* [ ] 接入 `main.rs` 命令行入口，输出赋值语句和布尔表达式四元组。
* [ ] 为算术赋值、括号、布尔关系、`and`、`or`、`not` 补充测试。
