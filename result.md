
=== 原始文法 ===

文法: Simple 原文法（含直接左递归）
开始符号: Program

  (1)  Program → StmtList
  (2)  StmtList → StmtList Statement
  (3)  StmtList → Statement
  (4)  Statement → SimpleStmt ;
  (5)  Statement → IfStmt
  (6)  Statement → WhileStmt
  (7)  Statement → Block
  (8)  Block → { StmtList }
  (9)  Block → { }
  (10)  SimpleStmt → DeclStmt
  (11)  SimpleStmt → AssignStmt
  (12)  DeclStmt → let Variable DeclInit
  (13)  DeclInit → = Expr
  (14)  DeclInit → ε
  (15)  AssignStmt → Variable = Expr
  (16)  IfStmt → if ( BoolExpr ) Block ElsePart
  (17)  ElsePart → else ElseBody
  (18)  ElsePart → ε
  (19)  ElseBody → Block
  (20)  ElseBody → IfStmt
  (21)  WhileStmt → while ( BoolExpr ) Block
  (22)  Expr → Expr + Term
  (23)  Expr → Expr - Term
  (24)  Expr → Term
  (25)  Term → Term * Factor
  (26)  Term → Term / Factor
  (27)  Term → Factor
  (28)  Factor → ( Expr )
  (29)  Factor → Variable
  (30)  Factor → num
  (31)  BoolExpr → BoolExpr or BoolAnd
  (32)  BoolExpr → BoolAnd
  (33)  BoolAnd → BoolAnd and BoolNot
  (34)  BoolAnd → BoolNot
  (35)  BoolNot → not BoolNot
  (36)  BoolNot → BoolAtom
  (37)  BoolAtom → ( BoolExpr )
  (38)  BoolAtom → true
  (39)  BoolAtom → false
  (40)  BoolAtom → Relation
  (41)  Relation → Variable RelOp Expr
  (42)  Relation → num RelOp Expr
  (43)  RelOp → ==
  (44)  RelOp → !=
  (45)  RelOp → <
  (46)  RelOp → <=
  (47)  RelOp → >
  (48)  RelOp → >=
  (49)  Variable → id



=== LL(1) 文法 ===

文法: LL(1) 改造后文法
开始符号: Program

  (1)  Program → StmtList
  (2)  StmtList → Statement StmtListTail
  (3)  StmtListTail → Statement StmtListTail
  (4)  StmtListTail → ε
  (5)  Statement → SimpleStmt ;
  (6)  Statement → IfStmt
  (7)  Statement → WhileStmt
  (8)  Statement → Block
  (9)  Block → { BlockBody }
  (10)  BlockBody → StmtList
  (11)  BlockBody → ε
  (12)  SimpleStmt → DeclStmt
  (13)  SimpleStmt → AssignStmt
  (14)  DeclStmt → let Variable DeclInit
  (15)  DeclInit → = Expr
  (16)  DeclInit → ε
  (17)  AssignStmt → Variable = Expr
  (18)  IfStmt → if ( BoolExpr ) Block ElsePart
  (19)  ElsePart → else ElseBody
  (20)  ElsePart → ε
  (21)  ElseBody → Block
  (22)  ElseBody → IfStmt
  (23)  WhileStmt → while ( BoolExpr ) Block
  (24)  Expr → Term ExprTail
  (25)  ExprTail → + Term ExprTail
  (26)  ExprTail → - Term ExprTail
  (27)  ExprTail → ε
  (28)  Term → Factor TermTail
  (29)  TermTail → * Factor TermTail
  (30)  TermTail → / Factor TermTail
  (31)  TermTail → ε
  (32)  Factor → ( Expr )
  (33)  Factor → Variable
  (34)  Factor → num
  (35)  BoolExpr → BoolAnd BoolExprTail
  (36)  BoolExprTail → or BoolAnd BoolExprTail
  (37)  BoolExprTail → ε
  (38)  BoolAnd → BoolNot BoolAndTail
  (39)  BoolAndTail → and BoolNot BoolAndTail
  (40)  BoolAndTail → ε
  (41)  BoolNot → not BoolNot
  (42)  BoolNot → BoolAtom
  (43)  BoolAtom → ( BoolExpr )
  (44)  BoolAtom → true
  (45)  BoolAtom → false
  (46)  BoolAtom → Relation
  (47)  Relation → Variable RelOp Expr
  (48)  Relation → num RelOp Expr
  (49)  RelOp → ==
  (50)  RelOp → !=
  (51)  RelOp → <
  (52)  RelOp → <=
  (53)  RelOp → >
  (54)  RelOp → >=
  (55)  Variable → id



=== FIRST 集 ===

FIRST(Program) = { id, if, let, while, { }
FIRST(StmtList) = { id, if, let, while, { }
FIRST(StmtListTail) = { id, if, let, while, {, ε }
FIRST(Block) = { { }
FIRST(BlockBody) = { id, if, let, while, {, ε }
FIRST(Statement) = { id, if, let, while, { }
FIRST(SimpleStmt) = { id, let }
FIRST(IfStmt) = { if }
FIRST(WhileStmt) = { while }
FIRST(ElsePart) = { else, ε }
FIRST(ElseBody) = { if, { }
FIRST(DeclStmt) = { let }
FIRST(DeclInit) = { =, ε }
FIRST(AssignStmt) = { id }
FIRST(Expr) = { (, id, num }
FIRST(ExprTail) = { +, -, ε }
FIRST(Term) = { (, id, num }
FIRST(TermTail) = { *, /, ε }
FIRST(Factor) = { (, id, num }
FIRST(BoolExpr) = { (, false, id, not, num, true }
FIRST(BoolExprTail) = { or, ε }
FIRST(BoolAnd) = { (, false, id, not, num, true }
FIRST(BoolAndTail) = { and, ε }
FIRST(BoolNot) = { (, false, id, not, num, true }
FIRST(BoolAtom) = { (, false, id, num, true }
FIRST(Relation) = { id, num }
FIRST(RelOp) = { !=, <, <=, ==, >, >= }
FIRST(Variable) = { id }


=== FOLLOW 集 ===

FOLLOW(Program) = { # }
FOLLOW(StmtList) = { #, } }
FOLLOW(StmtListTail) = { #, } }
FOLLOW(Block) = { #, else, id, if, let, while, {, } }
FOLLOW(BlockBody) = { } }
FOLLOW(Statement) = { #, id, if, let, while, {, } }
FOLLOW(SimpleStmt) = { ; }
FOLLOW(IfStmt) = { #, id, if, let, while, {, } }
FOLLOW(WhileStmt) = { #, id, if, let, while, {, } }
FOLLOW(ElsePart) = { #, id, if, let, while, {, } }
FOLLOW(ElseBody) = { #, id, if, let, while, {, } }
FOLLOW(DeclStmt) = { ; }
FOLLOW(DeclInit) = { ; }
FOLLOW(AssignStmt) = { ; }
FOLLOW(Expr) = { ), ;, and, or }
FOLLOW(ExprTail) = { ), ;, and, or }
FOLLOW(Term) = { ), +, -, ;, and, or }
FOLLOW(TermTail) = { ), +, -, ;, and, or }
FOLLOW(Factor) = { ), *, +, -, /, ;, and, or }
FOLLOW(BoolExpr) = { ) }
FOLLOW(BoolExprTail) = { ) }
FOLLOW(BoolAnd) = { ), or }
FOLLOW(BoolAndTail) = { ), or }
FOLLOW(BoolNot) = { ), and, or }
FOLLOW(BoolAtom) = { ), and, or }
FOLLOW(Relation) = { ), and, or }
FOLLOW(RelOp) = { (, id, num }
FOLLOW(Variable) = { !=, ), *, +, -, /, ;, <, <=, =, ==, >, >=, and, or }


=== LL(1) 分析表 ===

非终结符 | 向前看符号 | 产生式
------------------------------------------------
Program | id | Program → StmtList
Program | let | Program → StmtList
Program | if | Program → StmtList
Program | while | Program → StmtList
Program | { | Program → StmtList
StmtList | id | StmtList → Statement StmtListTail
StmtList | let | StmtList → Statement StmtListTail
StmtList | if | StmtList → Statement StmtListTail
StmtList | while | StmtList → Statement StmtListTail
StmtList | { | StmtList → Statement StmtListTail
StmtListTail | id | StmtListTail → Statement StmtListTail
StmtListTail | let | StmtListTail → Statement StmtListTail
StmtListTail | if | StmtListTail → Statement StmtListTail
StmtListTail | while | StmtListTail → Statement StmtListTail
StmtListTail | { | StmtListTail → Statement StmtListTail
StmtListTail | } | StmtListTail → ε
StmtListTail | # | StmtListTail → ε
Block | { | Block → { BlockBody }
BlockBody | id | BlockBody → StmtList
BlockBody | let | BlockBody → StmtList
BlockBody | if | BlockBody → StmtList
BlockBody | while | BlockBody → StmtList
BlockBody | { | BlockBody → StmtList
BlockBody | } | BlockBody → ε
Statement | id | Statement → SimpleStmt ;
Statement | let | Statement → SimpleStmt ;
Statement | if | Statement → IfStmt
Statement | while | Statement → WhileStmt
Statement | { | Statement → Block
SimpleStmt | id | SimpleStmt → AssignStmt
SimpleStmt | let | SimpleStmt → DeclStmt
IfStmt | if | IfStmt → if ( BoolExpr ) Block ElsePart
WhileStmt | while | WhileStmt → while ( BoolExpr ) Block
ElsePart | id | ElsePart → ε
ElsePart | let | ElsePart → ε
ElsePart | if | ElsePart → ε
ElsePart | else | ElsePart → else ElseBody
ElsePart | while | ElsePart → ε
ElsePart | { | ElsePart → ε
ElsePart | } | ElsePart → ε
ElsePart | # | ElsePart → ε
ElseBody | if | ElseBody → IfStmt
ElseBody | { | ElseBody → Block
DeclStmt | let | DeclStmt → let Variable DeclInit
DeclInit | = | DeclInit → = Expr
DeclInit | ; | DeclInit → ε
AssignStmt | id | AssignStmt → Variable = Expr
Expr | id | Expr → Term ExprTail
Expr | num | Expr → Term ExprTail
Expr | ( | Expr → Term ExprTail
ExprTail | and | ExprTail → ε
ExprTail | or | ExprTail → ε
ExprTail | + | ExprTail → + Term ExprTail
ExprTail | - | ExprTail → - Term ExprTail
ExprTail | ) | ExprTail → ε
ExprTail | ; | ExprTail → ε
Term | id | Term → Factor TermTail
Term | num | Term → Factor TermTail
Term | ( | Term → Factor TermTail
TermTail | and | TermTail → ε
TermTail | or | TermTail → ε
TermTail | + | TermTail → ε
TermTail | - | TermTail → ε
TermTail | * | TermTail → * Factor TermTail
TermTail | / | TermTail → / Factor TermTail
TermTail | ) | TermTail → ε
TermTail | ; | TermTail → ε
Factor | id | Factor → Variable
Factor | num | Factor → num
Factor | ( | Factor → ( Expr )
BoolExpr | id | BoolExpr → BoolAnd BoolExprTail
BoolExpr | num | BoolExpr → BoolAnd BoolExprTail
BoolExpr | true | BoolExpr → BoolAnd BoolExprTail
BoolExpr | false | BoolExpr → BoolAnd BoolExprTail
BoolExpr | not | BoolExpr → BoolAnd BoolExprTail
BoolExpr | ( | BoolExpr → BoolAnd BoolExprTail
BoolExprTail | or | BoolExprTail → or BoolAnd BoolExprTail
BoolExprTail | ) | BoolExprTail → ε
BoolAnd | id | BoolAnd → BoolNot BoolAndTail
BoolAnd | num | BoolAnd → BoolNot BoolAndTail
BoolAnd | true | BoolAnd → BoolNot BoolAndTail
BoolAnd | false | BoolAnd → BoolNot BoolAndTail
BoolAnd | not | BoolAnd → BoolNot BoolAndTail
BoolAnd | ( | BoolAnd → BoolNot BoolAndTail
BoolAndTail | and | BoolAndTail → and BoolNot BoolAndTail
BoolAndTail | or | BoolAndTail → ε
BoolAndTail | ) | BoolAndTail → ε
BoolNot | id | BoolNot → BoolAtom
BoolNot | num | BoolNot → BoolAtom
BoolNot | true | BoolNot → BoolAtom
BoolNot | false | BoolNot → BoolAtom
BoolNot | not | BoolNot → not BoolNot
BoolNot | ( | BoolNot → BoolAtom
BoolAtom | id | BoolAtom → Relation
BoolAtom | num | BoolAtom → Relation
BoolAtom | true | BoolAtom → true
BoolAtom | false | BoolAtom → false
BoolAtom | ( | BoolAtom → ( BoolExpr )
Relation | id | Relation → Variable RelOp Expr
Relation | num | Relation → num RelOp Expr
RelOp | == | RelOp → ==
RelOp | != | RelOp → !=
RelOp | < | RelOp → <
RelOp | <= | RelOp → <=
RelOp | > | RelOp → >
RelOp | >= | RelOp → >=
Variable | id | Variable → id


=== 词法分析 ===

[样例 1] let x = 1;
Token: let id = num ; #

[样例 2] x = 1 - 2 / 3;
Token: id = num - num / num ; #

[样例 3] if ( not false and x >= 1 ) { } else if ( x == 10 or false ) { let z; }
Token: if ( not false and id >= num ) { } else if ( id == num or false ) { let id ; } #

[样例 4] while ( x > 0 ) { x = x - 1; }
Token: while ( id > num ) { id = id - num ; } #

[样例 5] { let a = 1; let b; }
Token: { let id = num ; let id ; } #

[样例 6] { { } while ( x > 0 ) { if ( x == 1 ) { } } }
Token: { { } while ( id > num ) { if ( id == num ) { } } } #


=== LL(1) 预测分析 ===

[样例 1] let x = 1;
分析结果: 接受
语法树:
Program
  StmtList
    Statement
      SimpleStmt
        DeclStmt
          let
          Variable
            id
          DeclInit
            =
            Expr
              Term
                Factor
                  num
                TermTail
                  ε
              ExprTail
                ε
      ;
    StmtListTail
      ε

[样例 2] x = 1 - 2 / 3;
分析结果: 接受
语法树:
Program
  StmtList
    Statement
      SimpleStmt
        AssignStmt
          Variable
            id
          =
          Expr
            Term
              Factor
                num
              TermTail
                ε
            ExprTail
              -
              Term
                Factor
                  num
                TermTail
                  /
                  Factor
                    num
                  TermTail
                    ε
              ExprTail
                ε
      ;
    StmtListTail
      ε

[样例 3] if ( not false and x >= 1 ) { } else if ( x == 10 or false ) { let z; }
分析结果: 接受
语法树:
Program
  StmtList
    Statement
      IfStmt
        if
        (
        BoolExpr
          BoolAnd
            BoolNot
              not
              BoolNot
                BoolAtom
                  false
            BoolAndTail
              and
              BoolNot
                BoolAtom
                  Relation
                    Variable
                      id
                    RelOp
                      >=
                    Expr
                      Term
                        Factor
                          num
                        TermTail
                          ε
                      ExprTail
                        ε
              BoolAndTail
                ε
          BoolExprTail
            ε
        )
        Block
          {
          BlockBody
            ε
          }
        ElsePart
          else
          ElseBody
            IfStmt
              if
              (
              BoolExpr
                BoolAnd
                  BoolNot
                    BoolAtom
                      Relation
                        Variable
                          id
                        RelOp
                          ==
                        Expr
                          Term
                            Factor
                              num
                            TermTail
                              ε
                          ExprTail
                            ε
                  BoolAndTail
                    ε
                BoolExprTail
                  or
                  BoolAnd
                    BoolNot
                      BoolAtom
                        false
                    BoolAndTail
                      ε
                  BoolExprTail
                    ε
              )
              Block
                {
                BlockBody
                  StmtList
                    Statement
                      SimpleStmt
                        DeclStmt
                          let
                          Variable
                            id
                          DeclInit
                            ε
                      ;
                    StmtListTail
                      ε
                }
              ElsePart
                ε
    StmtListTail
      ε

[样例 4] while ( x > 0 ) { x = x - 1; }
分析结果: 接受
语法树:
Program
  StmtList
    Statement
      WhileStmt
        while
        (
        BoolExpr
          BoolAnd
            BoolNot
              BoolAtom
                Relation
                  Variable
                    id
                  RelOp
                    >
                  Expr
                    Term
                      Factor
                        num
                      TermTail
                        ε
                    ExprTail
                      ε
            BoolAndTail
              ε
          BoolExprTail
            ε
        )
        Block
          {
          BlockBody
            StmtList
              Statement
                SimpleStmt
                  AssignStmt
                    Variable
                      id
                    =
                    Expr
                      Term
                        Factor
                          Variable
                            id
                        TermTail
                          ε
                      ExprTail
                        -
                        Term
                          Factor
                            num
                          TermTail
                            ε
                        ExprTail
                          ε
                ;
              StmtListTail
                ε
          }
    StmtListTail
      ε

[样例 5] { let a = 1; let b; }
分析结果: 接受
语法树:
Program
  StmtList
    Statement
      Block
        {
        BlockBody
          StmtList
            Statement
              SimpleStmt
                DeclStmt
                  let
                  Variable
                    id
                  DeclInit
                    =
                    Expr
                      Term
                        Factor
                          num
                        TermTail
                          ε
                      ExprTail
                        ε
              ;
            StmtListTail
              Statement
                SimpleStmt
                  DeclStmt
                    let
                    Variable
                      id
                    DeclInit
                      ε
                ;
              StmtListTail
                ε
        }
    StmtListTail
      ε

[样例 6] { { } while ( x > 0 ) { if ( x == 1 ) { } } }
分析结果: 接受
语法树:
Program
  StmtList
    Statement
      Block
        {
        BlockBody
          StmtList
            Statement
              Block
                {
                BlockBody
                  ε
                }
            StmtListTail
              Statement
                WhileStmt
                  while
                  (
                  BoolExpr
                    BoolAnd
                      BoolNot
                        BoolAtom
                          Relation
                            Variable
                              id
                            RelOp
                              >
                            Expr
                              Term
                                Factor
                                  num
                                TermTail
                                  ε
                              ExprTail
                                ε
                      BoolAndTail
                        ε
                    BoolExprTail
                      ε
                  )
                  Block
                    {
                    BlockBody
                      StmtList
                        Statement
                          IfStmt
                            if
                            (
                            BoolExpr
                              BoolAnd
                                BoolNot
                                  BoolAtom
                                    Relation
                                      Variable
                                        id
                                      RelOp
                                        ==
                                      Expr
                                        Term
                                          Factor
                                            num
                                          TermTail
                                            ε
                                        ExprTail
                                          ε
                                BoolAndTail
                                  ε
                              BoolExprTail
                                ε
                            )
                            Block
                              {
                              BlockBody
                                ε
                              }
                            ElsePart
                              ε
                        StmtListTail
                          ε
                    }
              StmtListTail
                ε
        }
    StmtListTail
      ε


=== Pratt 递归下降分析 ===

[样例 1] let x = 1;
分析结果: 接受
调用轨迹:
  enter parse_program
  enter parse_stmt_list
  enter parse_statement
  enter parse_decl_stmt
  match let
  enter parse_variable
  match id
  match =
  enter parse_expr
  match num
  match ;
语法树:
Program
  StmtList
    Statement
      DeclStmt
        let
        Variable
          id
        =
        Expr
          Number
            num
      ;

[样例 2] x = 1 - 2 / 3;
分析结果: 接受
调用轨迹:
  enter parse_program
  enter parse_stmt_list
  enter parse_statement
  enter parse_assign_stmt
  enter parse_variable
  match id
  match =
  enter parse_expr
  match num
  match -
  match num
  match /
  match num
  match ;
语法树:
Program
  StmtList
    Statement
      AssignStmt
        Variable
          id
        =
        Expr
          BinaryExpr
            Number
              num
            -
            BinaryExpr
              Number
                num
              /
              Number
                num
      ;

[样例 3] if ( not false and x >= 1 ) { } else if ( x == 10 or false ) { let z; }
分析结果: 接受
调用轨迹:
  enter parse_program
  enter parse_stmt_list
  enter parse_statement
  enter parse_if_stmt
  match if
  match (
  enter parse_bool_expr
  match not
  match false
  match and
  enter parse_variable
  match id
  match >=
  enter parse_expr
  match num
  match )
  enter parse_block
  match {
  match }
  match else
  enter parse_if_stmt
  match if
  match (
  enter parse_bool_expr
  enter parse_variable
  match id
  match ==
  enter parse_expr
  match num
  match or
  match false
  match )
  enter parse_block
  match {
  enter parse_stmt_list
  enter parse_statement
  enter parse_decl_stmt
  match let
  enter parse_variable
  match id
  match ;
  match }
语法树:
Program
  StmtList
    Statement
      IfStmt
        if
        (
        BoolExpr
          LogicalExpr
            NotExpr
              not
              BoolLiteral
                false
            and
            Relation
              Variable
                id
              >=
              Expr
                Number
                  num
        )
        Block
          {
          ε
          }
        else
        IfStmt
          if
          (
          BoolExpr
            LogicalExpr
              Relation
                Variable
                  id
                ==
                Expr
                  Number
                    num
              or
              BoolLiteral
                false
          )
          Block
            {
            StmtList
              Statement
                DeclStmt
                  let
                  Variable
                    id
                  ε
                ;
            }
          ε

[样例 4] while ( x > 0 ) { x = x - 1; }
分析结果: 接受
调用轨迹:
  enter parse_program
  enter parse_stmt_list
  enter parse_statement
  enter parse_while_stmt
  match while
  match (
  enter parse_bool_expr
  enter parse_variable
  match id
  match >
  enter parse_expr
  match num
  match )
  enter parse_block
  match {
  enter parse_stmt_list
  enter parse_statement
  enter parse_assign_stmt
  enter parse_variable
  match id
  match =
  enter parse_expr
  enter parse_variable
  match id
  match -
  match num
  match ;
  match }
语法树:
Program
  StmtList
    Statement
      WhileStmt
        while
        (
        BoolExpr
          Relation
            Variable
              id
            >
            Expr
              Number
                num
        )
        Block
          {
          StmtList
            Statement
              AssignStmt
                Variable
                  id
                =
                Expr
                  BinaryExpr
                    Variable
                      id
                    -
                    Number
                      num
              ;
          }

[样例 5] { let a = 1; let b; }
分析结果: 接受
调用轨迹:
  enter parse_program
  enter parse_stmt_list
  enter parse_statement
  enter parse_block
  match {
  enter parse_stmt_list
  enter parse_statement
  enter parse_decl_stmt
  match let
  enter parse_variable
  match id
  match =
  enter parse_expr
  match num
  match ;
  enter parse_statement
  enter parse_decl_stmt
  match let
  enter parse_variable
  match id
  match ;
  match }
语法树:
Program
  StmtList
    Statement
      Block
        {
        StmtList
          Statement
            DeclStmt
              let
              Variable
                id
              =
              Expr
                Number
                  num
            ;
          Statement
            DeclStmt
              let
              Variable
                id
              ε
            ;
        }

[样例 6] { { } while ( x > 0 ) { if ( x == 1 ) { } } }
分析结果: 接受
调用轨迹:
  enter parse_program
  enter parse_stmt_list
  enter parse_statement
  enter parse_block
  match {
  enter parse_stmt_list
  enter parse_statement
  enter parse_block
  match {
  match }
  enter parse_statement
  enter parse_while_stmt
  match while
  match (
  enter parse_bool_expr
  enter parse_variable
  match id
  match >
  enter parse_expr
  match num
  match )
  enter parse_block
  match {
  enter parse_stmt_list
  enter parse_statement
  enter parse_if_stmt
  match if
  match (
  enter parse_bool_expr
  enter parse_variable
  match id
  match ==
  enter parse_expr
  match num
  match )
  enter parse_block
  match {
  match }
  match }
  match }
语法树:
Program
  StmtList
    Statement
      Block
        {
        StmtList
          Statement
            Block
              {
              ε
              }
          Statement
            WhileStmt
              while
              (
              BoolExpr
                Relation
                  Variable
                    id
                  >
                  Expr
                    Number
                      num
              )
              Block
                {
                StmtList
                  Statement
                    IfStmt
                      if
                      (
                      BoolExpr
                        Relation
                          Variable
                            id
                          ==
                          Expr
                            Number
                              num
                      )
                      Block
                        {
                        ε
                        }
                      ε
                }
        }
