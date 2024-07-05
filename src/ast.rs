
/*
outputAST:
code = "(a + 2) / b"
Program {
    body: [
        BinaryExpr {
            left: BinaryExpr {
                left: IdentifierLiteral {
                    value: "a"
                 },
                op: Addition,
                right: IntegerLiteral {
                    value: "2"
                }
            }
            op: Division,
            right: IdentifierLiteral {
                value: "b"
            }
        }
    ]
}
*/
#[derive(Debug)]
pub enum Operators {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus,
}
#[derive(Debug, Clone)]
pub enum BinaryExprType {
    IdentifierLiteral { value: String },
    IntegerLiteral { value: f64 },
    BinaryExpr { left: Box<BinaryExprType>, op: &'static Operators, right: Box<BinaryExprType>},
}

#[derive(Debug)]
pub enum Expr {
    IntegerLiteral { value: f64 },
    IdentifierLiteral { value: String },
    BinaryExpr { left: Box<BinaryExprType>, op: Operators, right: Box<BinaryExprType> }
}

#[derive(Debug)]
pub struct Program {
    pub body: Vec<Expr>,
}