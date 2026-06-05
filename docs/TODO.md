# TODO：simple_parser_ll1 当前实现计划

## 当前状态

已完成：

* 文法符号与产生式结构
* 原始文法与 LL(1) 文法打印
* First / Follow 集计算
* LL(1) 分析表构造与冲突检测
* 基础词法分析器
* 语法树节点结构与表驱动分析雏形

当前文法特性：

* `let` 声明：`let x;`、`let x = expr;`
* 赋值：`x = expr;`
* 算术表达式：`+ - * /`
* 布尔条件：`or / and / not`
* 比较：`== != < <= > >=`
* 代码块：`{ ... }`
* 条件语句：`if (...) { ... } else if (...) { ... } else { ... }`
* 循环语句：`while (...) { ... }`
* 注释：`// ...`、`/* ... */`

## 当前实现方案

### 词法层

支持的 token：

```text
id num let true false if else while and or not
+ - * / = == != < <= > >=
( ) { } ; #
```

### 语法层

当前 LL(1) 文法以以下结构为核心：

```text
Program      -> StmtList
StmtList     -> Statement StmtListTail
StmtListTail -> Statement StmtListTail | ε

Statement    -> SimpleStmt ; | IfStmt | WhileStmt | Block
Block        -> { BlockBody }
BlockBody    -> StmtList | ε

SimpleStmt   -> DeclStmt | AssignStmt
DeclStmt     -> let Variable DeclInit
DeclInit     -> = Expr | ε
AssignStmt   -> Variable = Expr

IfStmt       -> if ( BoolExpr ) Block ElsePart
ElsePart     -> else ElseBody | ε
ElseBody     -> Block | IfStmt
WhileStmt    -> while ( BoolExpr ) Block
```

表达式层：

```text
Expr         -> Term ExprTail
ExprTail     -> + Term ExprTail | - Term ExprTail | ε
Term         -> Factor TermTail
TermTail     -> * Factor TermTail | / Factor TermTail | ε
Factor       -> ( Expr ) | Variable | num
```

布尔条件层：

```text
BoolExpr     -> BoolAnd BoolExprTail
BoolExprTail -> or BoolAnd BoolExprTail | ε
BoolAnd      -> BoolNot BoolAndTail
BoolAndTail  -> and BoolNot BoolAndTail | ε
BoolNot      -> not BoolNot | BoolAtom
BoolAtom     -> ( BoolExpr ) | true | false | Relation
Relation     -> Variable RelOp Expr | num RelOp Expr
RelOp        -> == | != | < | <= | > | >=
```

## 下一步

高优先级：

* 把 `parser()` 真正接入 `main.rs`
* 输出 First / Follow 集
* 输出 LL(1) 分析表
* 给 `parser()` 增加最小测试
* 实现语法树缩进打印

中优先级：

* 把 `first_follow.rs` 里的 parser 逻辑拆成独立模块
* 增加更明确的错误信息
* 实现递归下降版本，和表驱动版本对照

低优先级：

* 清理 warning
* 统一模块命名
* 增加更多非法输入测试

## 验收标准

本阶段至少需要达到：

* `cargo test` 通过
* `cargo run` 能打印当前原文法与 LL(1) 文法
* lexer 能正确识别当前语法样例与注释
* LL(1) 分析表构造不出现静默覆盖
* 语法分析器可以成功跑通至少一个声明、一个赋值、一个 `if-else if-else` 样例和一个 `while` 样例
