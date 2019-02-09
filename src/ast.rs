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
pub type Vector = (Type, Number, Id);

#[derive(Debug, Copy, Clone)]
pub enum Type {
    Integer,
}

pub type Id = String;
pub type Number = i32;

pub type Instructions = Vec<Instruction>;

#[derive(Debug, Clone)]
pub enum Instruction {
    Affectation(LeftValue, Expression),
    Eval(Expression),
    Return(Expression),
    If(Expression, Instructions, Instructions),
    While(Expression, Instructions),
    WriteFunction(Expression),
    NOP,
}

pub type Expressions = Vec<Expression>;

#[derive(Debug, Clone)]
pub enum Expression {
    Value(Number),
    LeftValue(LeftValue),
    CallFunction(Id, Expressions),
    ReadFunction,
    UnaryOperation(UnaryOperator, Box<Expression>),
    BinaryOperation(BinaryOperator, Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum LeftValue {
    Variable(Id),
    VariableAt(Id, Box<Expression>),
}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOperator {
    // Boolean
    Not,
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
