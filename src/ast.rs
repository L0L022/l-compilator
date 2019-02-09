pub type Program = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    DclVariable(Variable),
    DclFunction(Id, Vec<Scalar>, Vec<Scalar>, Instructions),
}

#[derive(Debug)]
pub enum Variable {
    Scalar(Scalar),
    Vector(Vector),
}

pub type Scalar = (Type, Id);
pub type Vector = (Type, Number, Id);

#[derive(Debug)]
pub enum Type {
    Integer,
}

pub type Id = String;
pub type Number = i32;

pub type Instructions = Vec<Instruction>;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Expression {
    Value(Number),
    LeftValue(LeftValue),
    CallFunction(Id, Expressions),
    ReadFunction,
    UnaryOperation(UnaryOperator, Box<Expression>),
    BinaryOperation(BinaryOperator, Box<Expression>, Box<Expression>),
}

#[derive(Debug)]
pub enum LeftValue {
    Variable(Id),
    VariableAt(Id, Box<Expression>),
}

#[derive(Debug)]
pub enum UnaryOperator {
    // Boolean
    Not,
}

#[derive(Debug)]
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
