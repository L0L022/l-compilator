lalrpop_mod!(grammar);

use crate::ast::Program;
use crate::lexer::Lexer;
use crate::lexer::Spanned;
use crate::lexer::Token;
use failure::Error;
use grammar::ProgramParser;

pub struct Parser {
    parser: ProgramParser,
}

#[derive(Debug, Fail)]
#[fail(display = "parse error occured")]
pub struct ParseError {
    #[cause]
    error: lalrpop_util::ParseError<usize, Token, Error>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            parser: ProgramParser::new(),
        }
    }

    pub fn parse<Tokens: IntoIterator<Item = Spanned<Token, usize, Error>>>(
        self,
        tokens: Tokens,
    ) -> Result<Program, Error> {
        match self.parser.parse(tokens) {
            Ok(v) => Ok(v),
            Err(error) => Err(ParseError { error }.into()),
        }
    }
}
