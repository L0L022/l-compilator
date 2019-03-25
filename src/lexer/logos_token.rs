use crate::lexer::UndefinedBehavior;
use crate::token::Token;
use failure::Error;
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
pub enum LogosToken {
    #[end]
    End,

    #[error]
    Error,

    #[regex = "[0-9]+"]
    Number,

    #[regex = "[a-zA-Z_$][a-zA-Z_$0-9]*"]
    Id,

    #[regex = "#.*"]
    Comment,

    #[token = ","]
    Comma,

    #[token = ";"]
    Semicolon,

    // Types
    #[token = "entier"]
    IntegerType,

    // Predefined functions
    #[token = "lire"]
    ReadFunction,

    #[token = "ecrire"]
    WriteFunction,

    // Instructions
    #[token = "retour"]
    Return,

    #[token = "si"]
    If,

    #[token = "alors"]
    Then,

    #[token = "sinon"]
    Else,

    #[token = "tantque"]
    While,

    #[token = "faire"]
    Do,

    // Brackets
    #[token = "("]
    OpenParenthesis,

    #[token = ")"]
    CloseParenthesis,

    #[token = "{"]
    OpenCurlyBracket,

    #[token = "}"]
    CloseCurlyBracket,

    #[token = "["]
    OpenSquareBracket,

    #[token = "]"]
    CloseSquareBracket,

    // Operators
    #[token = "+"]
    Addition,

    #[token = "-"]
    Subtraction,

    #[token = "*"]
    Multiplication,

    #[token = "/"]
    Division,

    #[token = "<"]
    LessThan,

    #[token = "="]
    Equal,

    #[token = "&"]
    And,

    #[token = "|"]
    Or,

    #[token = "!"]
    Not,
}

impl LogosToken {
    pub fn to_token(self, token: &str) -> Result<Token, Error> {
        use LogosToken::*;
        use Token as T;

        let token = match self {
            End => unreachable!(),
            Error => return Err(UndefinedBehavior {}.into()),
            Number => T::Number(token.parse()?),
            Id => T::Id(token.to_string()),
            Comment => unreachable!(),
            Comma => T::Comma,
            Semicolon => T::Semicolon,
            IntegerType => T::IntegerType,
            ReadFunction => T::ReadFunction,
            WriteFunction => T::WriteFunction,
            Return => T::Return,
            If => T::If,
            Then => T::Then,
            Else => T::Else,
            While => T::While,
            Do => T::Do,
            OpenParenthesis => T::OpenParenthesis,
            CloseParenthesis => T::CloseParenthesis,
            OpenCurlyBracket => T::OpenCurlyBracket,
            CloseCurlyBracket => T::CloseCurlyBracket,
            OpenSquareBracket => T::OpenSquareBracket,
            CloseSquareBracket => T::CloseSquareBracket,
            Addition => T::Addition,
            Subtraction => T::Subtraction,
            Multiplication => T::Multiplication,
            Division => T::Division,
            LessThan => T::LessThan,
            Equal => T::Equal,
            And => T::And,
            Or => T::Or,
            Not => T::Not,
        };

        Ok(token)
    }
}
