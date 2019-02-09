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

impl Token {
    pub fn lex_name(&self) -> &'static str {
        use Token::*;

        match self {
            Number(_) => "nombre",
            Id(_) => "identificateur",
            IntegerType | ReadFunction | WriteFunction | Return | If | Then | Else | While | Do => {
                "mot_clef"
            }
            Comma | Semicolon | OpenParenthesis | CloseParenthesis | OpenCurlyBracket
            | CloseCurlyBracket | OpenSquareBracket | CloseSquareBracket | Addition
            | Subtraction | Multiplication | Division | LessThan | Equal | And | Or | Not => {
                "symbole"
            }
        }
    }

    pub fn lex_value(&self) -> String {
        use Token::*;

        if let Number(n) = self {
            return format!("{}", n);
        }

        match self {
            Number(_) => unreachable!(),
            Id(id) => id,
            Comma => "VIRGULE",
            Semicolon => "POINT_VIRGULE",

            // Types
            IntegerType => "entier",

            // Predefined functions
            ReadFunction => "lire",
            WriteFunction => "ecrire",

            // Instructions
            Return => "retour",
            If => "si",
            Then => "alors",
            Else => "sinon",
            While => "tantque",
            Do => "faire",

            // Brackets
            OpenParenthesis => "PARENTHESE_OUVRANTE",
            CloseParenthesis => "PARENTHESE_FERMANTE",
            OpenCurlyBracket => "ACCOLADE_OUVRANTE",
            CloseCurlyBracket => "ACCOLADE_FERMANTE",
            OpenSquareBracket => "CROCHET_OUVRANT",
            CloseSquareBracket => "CROCHET_FERMANT",

            // Operators
            Addition => "PLUS",
            Subtraction => "MOINS",
            Multiplication => "FOIS",
            Division => "DIVISE",
            LessThan => "INFERIEUR",
            Equal => "EGAL",
            And => "ET",
            Or => "OU",
            Not => "NON",
        }
        .to_owned()
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
