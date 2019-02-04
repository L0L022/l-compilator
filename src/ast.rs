pub type Programm = Vec<Statement>;

#[derive(Debug)]
pub enum Type {
	Integer
}

pub type Id = String;
pub type Number = i32;

pub type Scalar = (Type, Id);
pub type Vector = (Type, Number, Id);

#[derive(Debug)]
pub enum Variable {
	Scalar(Scalar),
	Vector(Vector),
}

#[derive(Debug)]
pub enum Statement {
	DclVariable(Vec<Variable>),
	DclFunction(Id, Vec<Scalar>, Vec<Scalar>, Vec<Instruction>),
}

#[derive(Debug)]
pub enum Instruction {
	Affectation(LeftValue, Expression),
	Eval(Expression),
	Return(Option<Expression>),
	If(Expression, Vec<Instruction>, Vec<Instruction>),
	While(Expression, Vec<Instruction>),
	ReadFunction(Vec<Expression>),
	WriteFunction(Vec<Expression>),
	NOP,
}

#[derive(Debug)]
pub enum LeftValue {
	Variable(Id),
	VariableAt(Id, Box<Expression>),
}

#[derive(Debug)]
pub enum Expression {
	Value(Number),
	LeftValue(LeftValue),
	CallFunction(Id, Vec<Expression>),
	UnaryOperation(UnaryOperator, Box<Expression>),
	BinaryOperation(BinaryOperator, Box<Expression>, Box<Expression>),
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
