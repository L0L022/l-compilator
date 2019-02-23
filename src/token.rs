use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i32),
    Id(String),
    Comma,
    Semicolon,

    // Types
    IntegerType,

    // Predefined functions
    ReadFunction,
    WriteFunction,

    // Instructions
    Return,
    If,
    Then,
    Else,
    While,
    Do,

    // Brackets
    OpenParenthesis,
    CloseParenthesis,
    OpenCurlyBracket,
    CloseCurlyBracket,
    OpenSquareBracket,
    CloseSquareBracket,

    // Operators
    Addition,
    Subtraction,
    Multiplication,
    Division,
    LessThan,
    Equal,
    And,
    Or,
    Not,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
