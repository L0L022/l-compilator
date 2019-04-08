use crate::lexer::Lexer;
use crate::token::Token;
use failure::Fallible;
use std::io;
use std::io::Write;

impl<'input> Lexer<'input> {
    pub fn into_lex(self, f: &mut dyn Write) -> Fallible<()> {
        let input = self.source();

        for spanned in self {
            let (begin, token, end) = spanned?;
            write!(f, "{}\t", &input[begin..end])?;
            token.lex_name(f)?;
            write!(f, "\t")?;
            token.lex_value(f)?;
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Token {
    fn lex_name(&self, f: &mut dyn Write) -> io::Result<()> {
        use Token::*;

        let name = match self {
            Number(_) => "nombre",
            Id(_) => "identificateur",
            IntegerType | ReadFunction | WriteFunction | Return | If | Then | Else | While | Do
            | For => "mot_clef",
            Comma | Semicolon | OpenParenthesis | CloseParenthesis | OpenCurlyBracket
            | CloseCurlyBracket | OpenSquareBracket | CloseSquareBracket | Addition
            | Subtraction | Multiplication | Division | LessThan | Equal | And | Or | Not => {
                "symbole"
            }
        };

        write!(f, "{}", name)
    }

    fn lex_value(&self, f: &mut dyn Write) -> io::Result<()> {
        use Token::*;

        if let Number(n) = self {
            return write!(f, "{}", n);
        }

        let value = match self {
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
            For => "pour",

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
        };

        write!(f, "{}", value)
    }
}
