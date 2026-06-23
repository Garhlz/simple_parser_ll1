# requirement.md

## 项目目标

本项目用于完成编译原理实验五：语法制导翻译与中间代码生成。

在已有词法分析和语法分析代码基础上，实现一个小型中间代码生成程序。程序应支持简单赋值语句、算术表达式和布尔表达式，并输出对应四元组序列。条件语句作为选做扩展。

## 已有基础

已有相关仓库：

* `simple_parser_ll1`
* `simple_parser_lr`

本实验建议优先基于 `simple_parser_ll1` 修改，复用：

* `lexer.rs`
* `symbol.rs`
* `sd_parser.rs` 中的递归下降表达式分析与语法制导翻译结构

不要从零重写 tokenizer。现有 tokenizer 已支持实验所需的大部分 token：

* 标识符、数字
* `+ - * /`
* `= == != < <= > >=`
* `and or not`
* `if else while`
* 括号、花括号、分号

## 必做功能

### 1. 算术表达式与赋值语句四元组生成

支持输入：

```text
a = b + c * e / g;
```

输出示例：

```text
0: (*, c, e, t1)
1: (/, t1, g, t2)
2: (+, b, t2, t3)
3: (=, t3, _, a)
```

要求：

* 正确处理运算符优先级；
* 支持括号；
* 支持变量和数字；
* 为每个中间结果生成临时变量，如 `t1`、`t2`、`t3`；
* 赋值语句最后生成 `=` 四元组。

### 2. 布尔表达式四元组生成

支持输入：

```text
a < b
a < b and c > d
a < b or c > d
not a < b
(a < b or c > d) and e != f
```

输出内容应包括：

* 条件跳转四元组；
* 无条件跳转四元组；
* 最终 `TC` 真链；
* 最终 `FC` 假链。

例如：

```text
a < b
```

可生成：

```text
0: (j<, a, b, _)
1: (j, _, _, _)

TC = [0]
FC = [1]
```

其中 `_` 表示目标地址暂未回填。

## 选做功能

支持条件语句：

```text
if (a < b) {
    x = y + z;
}
```

或：

```text
if (a < b) {
    x = y + z;
} else {
    x = y - z;
}
```

要求：

* 条件表达式使用布尔表达式的 `TC/FC` 链；
* then 分支和 else 分支生成赋值语句四元组；
* 使用回填机制连接控制流。

扩展支持循环语句：

```text
while (a < b) {
    a = a + 1;
}
```

要求：

* 条件表达式使用布尔表达式的 `TC/FC` 链；
* 循环体生成赋值语句四元组；
* 循环体结束后生成回到条件判断处的无条件跳转。

## 建议代码结构

建议新增模块：

```text
src/
├── main.rs
├── lexer.rs          # 复用
├── symbol.rs         # 复用
├── codegen.rs        # 四元组、临时变量、回填
├── sd_parser.rs      # 语法制导翻译 parser
```

当前实现以 `sd_parser.rs` 为主，不再保留早期只生成语法树的 `rd_parser.rs`。
原实验中的 `grammar.rs`、`first_follow.rs`、`parser.rs` 属于 LL(1) 文法展示、FIRST/FOLLOW 和预测分析表流程，当前 codegen 实验不再保留。

## 核心数据结构

普通操作数：

```rust
pub enum Operand {
    Name(String),
    Empty,
}
```

跳转目标：

```rust
pub enum JumpTarget {
    Pending,
    Target(usize),
}
```

四元组内部使用类型安全表示，输出时仍格式化为 `(op, arg1, arg2, result)`：

```rust
pub enum Quad {
    Binary { op: String, left: Operand, right: Operand, dest: Operand },
    Assign { src: Operand, dest: Operand },
    Jump { target: JumpTarget },
    JumpIf { relop: String, left: Operand, right: Operand, target: JumpTarget },
}
```

表达式属性：

```rust
pub struct ExprAttr {
    pub place: Operand,
}
```

布尔表达式属性：

```rust
pub struct BoolAttr {
    pub tc: Vec<usize>,
    pub fc: Vec<usize>,
}
```

代码生成器：

```rust
pub struct CodeGen {
    pub quads: Vec<Quad>,
    temp_count: usize,
}
```

需要提供：

```rust
new_temp()
emit()
makelist()
merge()
backpatch()
```

## 修改思路

### 算术表达式

原 `parse_expr()` / `parse_expr_bp()` 主要返回语法树节点；改造后应返回：

```rust
ExprAttr { place: ... }
```

识别二元运算：

```text
left op right
```

生成：

```text
(op, left.place, right.place, temp)
```

并返回：

```rust
ExprAttr { place: temp }
```

### 赋值语句

赋值语句格式：

```text
id = Expr ;
```

处理流程：

1. 读取左侧变量名；
2. 解析右侧表达式，得到 `ExprAttr.place`；
3. 生成赋值四元组：

```text
(=, expr.place, _, id)
```

### 布尔表达式

布尔表达式不返回 `place`，而是返回：

```rust
BoolAttr {
    tc: Vec<usize>,
    fc: Vec<usize>,
}
```

关系表达式：

```text
E relop E
```

生成两条四元组：

```text
(j<, a, b, _)
(j, _, _, _)
```

其中：

```text
TC = [条件跳转四元组编号]
FC = [无条件跳转四元组编号]
```

### and

```text
B1 and B2
```

语义规则：

```text
backpatch(B1.TC, B2.begin)
B.TC = B2.TC
B.FC = merge(B1.FC, B2.FC)
```

### or

```text
B1 or B2
```

语义规则：

```text
backpatch(B1.FC, B2.begin)
B.TC = merge(B1.TC, B2.TC)
B.FC = B2.FC
```

### not

```text
not B1
```

语义规则：

```text
B.TC = B1.FC
B.FC = B1.TC
```

## 命令行行为建议

程序可支持两种模式：

```bash
cargo run -- assign "a = b + c * e / g;"
cargo run -- bool "a < b and c > d"
cargo run -- program "{ a = b + c; x = a * d; }"
```

也可读取一行输入，并根据内容判断是赋值语句还是布尔表达式。

输出格式应清晰展示：

```text
Quadruples:
0: (*, c, e, t1)
1: (/, t1, g, t2)
2: (+, b, t2, t3)
3: (=, t3, _, a)
```

布尔表达式需额外展示：

```text
TC = [...]
FC = [...]
```

## 测试用例

### 算术赋值

```text
a = b + c * e / g;
```

期望：

```text
(*, c, e, t1)
(/, t1, g, t2)
(+, b, t2, t3)
(=, t3, _, a)
```

### 括号

```text
a = (b + c) * d;
```

期望：

```text
(+, b, c, t1)
(*, t1, d, t2)
(=, t2, _, a)
```

### 语句列表和代码块

```text
{ a = b + c; x = a * d; }
```

期望：

```text
(+, b, c, t1)
(=, t1, _, a)
(*, a, d, t2)
(=, t2, _, x)
```

### 布尔关系

```text
a < b
```

期望生成条件跳转四元组，并输出 `TC` 和 `FC`。

### 布尔 and

```text
a < b and c > d
```

要求体现短路求值：

* `a < b` 为真时继续判断 `c > d`；
* `a < b` 为假时直接进入假链。

### 布尔 or

```text
a < b or c > d
```

要求体现短路求值：

* `a < b` 为真时直接进入真链；
* `a < b` 为假时继续判断 `c > d`。

### 布尔 not

```text
not a < b
```

要求交换内部布尔表达式的真链和假链。

### 布尔优先级

```text
a < b or c > d and e != f
```

要求按 `not` 高于 `and`、`and` 高于 `or` 的优先级生成短路跳转四元组。

### 布尔括号

```text
(a < b or c > d) and e != f
```

要求括号内布尔表达式先作为整体翻译，再与外层布尔运算组合。

### if

```text
if (a < b) { x = y + z; }
```

期望：

```text
(j<, a, b, 2)
(j, _, _, 4)
(+, y, z, t1)
(=, t1, _, x)
```

### if-else

```text
if (a < b) { x = y + z; } else { x = y - z; }
```

期望：

```text
(j<, a, b, 2)
(j, _, _, 5)
(+, y, z, t1)
(=, t1, _, x)
(j, _, _, 7)
(-, y, z, t2)
(=, t2, _, x)
```

### while

```text
while (a < b) { a = a + 1; }
```

期望：

```text
(j<, a, b, 2)
(j, _, _, 5)
(+, a, 1, t1)
(=, t1, _, a)
(j, _, _, 0)
```

## 验收标准

完成后应满足：

1. 能正确复用 tokenizer；
2. 能解析并生成算术赋值语句四元组；
3. 能生成临时变量；
4. 能正确处理算术运算优先级和括号；
5. 能生成布尔表达式跳转四元组；
6. 能输出最终 `TC` 和 `FC` 链；
7. 能体现 `makelist`、`merge`、`backpatch` 的设计；
8. 代码结构清晰，便于在实验报告中说明文法、语义规则和实现过程。

## 注意事项

* 本实验重点是中间代码生成，不再继续展示 FIRST/FOLLOW 或分析表。
* 不要求生成汇编或机器码。
* 四元组格式为：

```text
(op, arg1, arg2, result)
```

* 算术表达式使用 `place` 属性传递结果位置。
* 布尔表达式使用 `TC` 和 `FC` 链处理跳转目标未确定的问题。
* 选做条件语句本质上是结合布尔表达式回填机制与赋值语句四元组生成。
