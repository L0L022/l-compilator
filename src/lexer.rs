use failure::Error;
use logos::Logos;

pub struct Lexer<'input> {
    lexer: logos::Lexer<LogosToken, &'input str>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer { lexer: LogosToken::lexer(input) }
    }

    pub fn into_lex(self) -> Result<String, Error> {
        let mut lex = String::new();
        let input = self.lexer.source;

        for spanned in self {
            let (begin, token, end) = spanned?;
            let line = format!(
                "{}\t{}\t{}\n",
                &input[begin..end],
                token.lex_name(),
                token.lex_value()
            );
            lex.push_str(&line);
        }

        Ok(lex)
    }
}

#[derive(Debug, Fail)]
#[fail(display = "lexical error occured at [{:?}] with this token: {}", range, token)]
pub struct LexicalError {
    token: String,
    range: std::ops::Range<usize>,
}

#[derive(Debug, Fail)]
#[fail(display = "undefined behavior")]
pub struct UndefinedBehavior;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            use LogosToken::*;

            let logos_token = self.lexer.token;

            return match logos_token {
                End => None,
                Comment => {
                    self.lexer.advance();
                    continue;
                }
                _ => {
                    let range = self.lexer.range();

                    let token = match logos_token.to_token(self.lexer.slice()) {
                        Ok(v) => v,
                        Err(e) => {
                            let e = e.context(LexicalError {
                                token: self.lexer.slice().to_string(),
                                range,
                            });
                            return Some(Err(e.into()));
                        }
                    };

                    self.lexer.advance();
                    Some(Ok((range.start, token, range.end)))
                }
            };
        }
    }
}

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
            Comma | Semicolon | OpenParenthesis | CloseParenthesis | OpenCurlyBracket |
            CloseCurlyBracket | OpenSquareBracket | CloseSquareBracket | Addition |
            Subtraction | Multiplication | Division | LessThan | Equal | And | Or | Not => {
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
        }.to_owned()
    }
}

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
enum LogosToken {
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
    fn to_token(self, token: &str) -> Result<Token, Error> {
        use LogosToken::*;
        use Token as T;

        let token = match self {
            End => unreachable!(),
            Error => return Err(UndefinedBehavior {}.into()),
            Number => T::Number(token.parse::<i32>()?),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn affect_err() {
        test("affect-err");
    }

    #[test]
    fn affect() {
        test("affect");
    }

    #[test]
    fn boucle() {
        test("boucle");
    }

    #[test]
    fn expression() {
        test("expression");
    }

    #[test]
    fn max() {
        test("max");
    }

    #[test]
    fn tri() {
        test("tri");
    }

    fn test(filename: &str) {
        let l_file = std::fs::read_to_string(format!("tests/resources/{}.l", filename)).unwrap();
        let lex_file = std::fs::read_to_string(format!("tests/resources/{}.lex", filename))
            .unwrap();

        let generated_lex = Lexer::new(&l_file).into_lex().unwrap();

        assert_eq!(lex_file, generated_lex);
    }
}
