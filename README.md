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
* `src/codegen.rs`：类型安全的四元组表示、临时变量和回填工具。
* `src/sd_parser.rs`：支持算术赋值语句、语句列表、代码块、`if` / `if-else` / `while` 控制流、关系表达式、布尔括号分组和 `and` / `or` / `not` 短路翻译。

已移除旧实验中不再需要的 LL(1) 展示流程，包括 FIRST/FOLLOW、预测分析表和表驱动 parser。

## 运行

生成算术赋值语句四元组：

```bash
cargo run -- assign "a = b + c * e / g;"
```

示例输出：

```text
Quadruples:
0: (*, c, e, t1)
1: (/, t1, g, t2)
2: (+, b, t2, t3)
3: (=, t3, _, a)
```

生成语句列表或代码块的四元组：

```bash
cargo run -- program "{ a = b + c; x = a * d; }"
```

示例输出：

```text
Quadruples:
0: (+, b, c, t1)
1: (=, t1, _, a)
2: (*, a, d, t2)
3: (=, t2, _, x)
```

查看 tokenizer 输出：

```bash
cargo run -- tokens "a = b + c * e / g;"
```

生成布尔表达式跳转四元组：

```bash
cargo run -- bool "a < b"
```

示例输出：

```text
Quadruples:
0: (j<, a, b, _)
1: (j, _, _, _)

TC = [0]
FC = [1]
```

短路布尔表达式示例：

```bash
cargo run -- bool "a < b or c > d and e != f"
```

示例输出：

```text
Quadruples:
0: (j<, a, b, _)
1: (j, _, _, 2)
2: (j>, c, d, 4)
3: (j, _, _, _)
4: (j!=, e, f, _)
5: (j, _, _, _)

TC = [0, 4]
FC = [3, 5]
```

布尔括号分组示例：

```bash
cargo run -- bool "(a < b or c > d) and e != f"
```

示例输出：

```text
Quadruples:
0: (j<, a, b, 4)
1: (j, _, _, 2)
2: (j>, c, d, 4)
3: (j, _, _, _)
4: (j!=, e, f, _)
5: (j, _, _, _)

TC = [4]
FC = [3, 5]
```

控制流语句示例：

```bash
cargo run -- program "if (a < b) { x = y + z; } else { x = y - z; }"
```

示例输出：

```text
Quadruples:
0: (j<, a, b, 2)
1: (j, _, _, 5)
2: (+, y, z, t1)
3: (=, t1, _, x)
4: (j, _, _, 7)
5: (-, y, z, t2)
6: (=, t2, _, x)
```

## 测试

```bash
cargo test
```

## 后续模块

当前实验五必做功能与选做控制流已覆盖。后续可继续扩展声明语句、作用域或更多语句类型。
