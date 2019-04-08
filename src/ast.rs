use std::fmt;

#[derive(Debug)]
pub struct Program(pub Vec<Statement>);

#[derive(Debug, Clone)]
pub enum Statement {
    DclVariable(Variable),
    DclFunction(Id, Vec<Scalar>, Vec<Scalar>, Instructions),
}

#[derive(Debug, Clone)]
pub enum Variable {
    Scalar(Scalar),
    Vector(Vector),
}

pub type Scalar = (Type, Id);
pub type Vector = (Type, u32, Id);

#[derive(Debug, Copy, Clone)]
pub enum Type {
    Integer,
}

impl Type {
    pub fn size(self) -> usize {
        use std::mem::size_of;
        use Type::*;

        match self {
            Integer => size_of::<Number>(),
        }
    }
}

pub type Id = String;
pub type Number = i32;

pub type Instructions = Vec<Instruction>;

#[derive(Debug, Clone)]
pub enum Instruction {
    Affectation(LeftValue, Expression),
    CallFunction(CallFunction),
    Return(Expression),
    If(Expression, Instructions, Instructions),
    While(Expression, Instructions),
    For(Box<Instruction>, Expression, Box<Instruction>, Instructions),
    WriteFunction(Expression),
    NOP,
}

pub type Expressions = Vec<Expression>;

#[derive(Debug, Clone)]
pub enum Expression {
    Value(Number),
    LeftValue(LeftValue),
    CallFunction(CallFunction),
    ReadFunction,
    UnaryOperation(UnaryOperator, Box<Expression>),
    BinaryOperation(BinaryOperator, Box<Expression>, Box<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Expression::*;

        match self {
            Value(n) => write!(f, "{}", n),
            LeftValue(lv) => write!(f, "{}", lv),
            CallFunction(cf) => write!(f, "{}", cf),
            ReadFunction => write!(f, "lire()"),
            UnaryOperation(o, e) => write!(f, "{}({})", o, e),
            BinaryOperation(o, left, right) => write!(f, "({} {} {})", left, o, right),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LeftValue {
    Variable(Id),
    VariableAt(Id, Box<Expression>),
}

impl fmt::Display for LeftValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use LeftValue::*;

        match self {
            Variable(id) => write!(f, "{}", id),
            VariableAt(id, indice) => write!(f, "{}[{}]", id, indice),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CallFunction(pub Id, pub Expressions);

impl fmt::Display for CallFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(", self.0)?;
        for (i, arg) in self.1.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", arg)?;
        }
        write!(f, ")")
    }
}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOperator {
    // Boolean
    Not,
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use UnaryOperator::*;

        let o = match self {
            Not => "!",
        };

        write!(f, "{}", o)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BinaryOperator {
    // Arithmetic
    Addidion,
    Subtraction,
    Multiplication,
    Division,

    // Boolean
    And,
    Or,
    Equal,
    LessThan,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BinaryOperator::*;

        let o = match self {
            Addidion => "+",
            Subtraction => "-",
            Multiplication => "*",
            Division => "/",
            And => "&",
            Or => "|",
            Equal => "==",
            LessThan => "<",
        };

        write!(f, "{}", o)
    }
}
