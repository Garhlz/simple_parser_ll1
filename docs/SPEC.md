# simple_parser_ll1 规格说明

## 1. 目标

本项目用于完成编译原理实验三：实现一个面向简化 Simple 语言的 LL(1) 语法分析器。

当前版本的目标是：

* 给出原始文法与 LL(1) 改造后文法
* 计算并输出 First / Follow 集
* 构造 LL(1) 预测分析表
* 基于分析表进行预测分析
* 在预测分析过程中同步构造语法树

递归下降分析仍然保留为后续目标，但当前实现主线已经明确为“词法分析 + First/Follow + 分析表 + 表驱动 LL(1) 分析 + 语法树”。

## 2. 当前语言子集

### 语句

支持：

* `let` 声明
* 赋值语句
* `if / else if / else`
* `while`
* 代码块 `{ ... }`

示例：

```text
let x = 1;
x = x - 2 / 3;

if ( x < 10 or false ) {
  let y = 1;
} else if ( x == 10 ) {
  let z;
} else {
  x = 0;
}

while ( x > 0 ) {
  x = x - 1;
}
```

### 表达式

算术表达式支持：

* `+ - * /`
* 括号
* 标识符与整数

布尔条件支持：

* `or / and / not`
* 比较运算 `== != < <= > >=`
* 布尔常量 `true / false`

## 3. 当前词法规则

支持的 token：

```text
id
num
let
true
false
if
else
while
and
or
not
+  -  *  /
=  ==  !=  <  <=  >  >=
(  )  {  }  ;
#
```

说明：

* `#` 为输入结束符，由程序自动补到 token 序列末尾
* 词法分析器支持单行注释 `// ...`
* 词法分析器支持多行注释 `/* ... */`
* 当前不支持字符串、数组、函数和 `var`

## 4. 当前文法

### 原始文法

```text
Program   → StmtList

StmtList  → StmtList Statement | Statement

Statement → SimpleStmt ; | IfStmt | WhileStmt | Block
Block     → { StmtList } | { }

SimpleStmt → DeclStmt | AssignStmt
DeclStmt   → let Variable DeclInit
DeclInit   → = Expr | ε
AssignStmt → Variable = Expr

IfStmt   → if ( BoolExpr ) Block ElsePart
ElsePart → else ElseBody | ε
ElseBody → Block | IfStmt
WhileStmt → while ( BoolExpr ) Block

Expr   → Expr + Term | Expr - Term | Term
Term   → Term * Factor | Term / Factor | Factor
Factor → ( Expr ) | Variable | num

BoolExpr → BoolExpr or BoolAnd | BoolAnd
BoolAnd  → BoolAnd and BoolNot | BoolNot
BoolNot  → not BoolNot | BoolAtom
BoolAtom → ( BoolExpr ) | true | false | Relation
Relation → Variable RelOp Expr | num RelOp Expr
RelOp    → == | != | < | <= | > | >=

Variable → id
```

### LL(1) 改造后文法

```text
Program      → StmtList

StmtList     → Statement StmtListTail
StmtListTail → Statement StmtListTail | ε

Statement    → SimpleStmt ; | IfStmt | WhileStmt | Block
Block        → { BlockBody }
BlockBody    → StmtList | ε

SimpleStmt   → DeclStmt | AssignStmt
DeclStmt     → let Variable DeclInit
DeclInit     → = Expr | ε
AssignStmt   → Variable = Expr

IfStmt       → if ( BoolExpr ) Block ElsePart
ElsePart     → else ElseBody | ε
ElseBody     → Block | IfStmt
WhileStmt    → while ( BoolExpr ) Block

Expr         → Term ExprTail
ExprTail     → + Term ExprTail | - Term ExprTail | ε
Term         → Factor TermTail
TermTail     → * Factor TermTail | / Factor TermTail | ε
Factor       → ( Expr ) | Variable | num

BoolExpr     → BoolAnd BoolExprTail
BoolExprTail → or BoolAnd BoolExprTail | ε
BoolAnd      → BoolNot BoolAndTail
BoolAndTail  → and BoolNot BoolAndTail | ε
BoolNot      → not BoolNot | BoolAtom
BoolAtom     → ( BoolExpr ) | true | false | Relation
Relation     → Variable RelOp Expr | num RelOp Expr
RelOp        → == | != | < | <= | > | >=

Variable     → id
```

## 5. 设计约束

当前这版文法有两个明确约束：

* 普通语句层不直接允许“裸布尔表达式语句”，避免 `id ...` 与赋值语句冲突
* `if` 的 then / else 主体统一使用 `Block`，并通过 `ElseBody -> Block | IfStmt` 支持 `else if`

这样可以避免 dangling-else 类问题，同时保持 LL(1) 分析较为直接。

## 6. 当前实现模块

```text
src/
  main.rs
  symbol.rs
  grammar.rs
  lexer.rs
  first_follow.rs
```

职责：

* `symbol.rs`：终结符、非终结符、统一符号类型
* `grammar.rs`：原始文法与 LL(1) 文法定义
* `lexer.rs`：词法分析
* `first_follow.rs`：First / Follow、分析表、表驱动分析和语法树节点结构
* `main.rs`：演示输出

## 7. 当前已验证内容

已验证：

* 词法分析测试通过
* 编译通过
* 当前文法与 token 示例可打印

仍需继续验证：

* 将 `parser()` 接入 `main.rs`
* 输出并人工检查 First / Follow 集
* 输出分析表
* 对声明、赋值、`if / else if / else`、`while`、代码块进行语法分析测试
* 打印并检查语法树结构
