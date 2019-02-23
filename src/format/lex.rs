use crate::lexer::Lexer;
use crate::token::Token;
use failure::Error;

impl<'input> Lexer<'input> {
    pub fn into_lex(self) -> Result<String, Error> {
        let mut lex = String::new();
        let input = self.source();

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

impl Token {
    fn lex_name(&self) -> &'static str {
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

    fn lex_value(&self) -> String {
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
