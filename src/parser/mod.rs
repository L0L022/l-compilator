lalrpop_mod!(
    #[allow(unused_imports)]
    #[allow(clippy::all)]
    grammar,
    "/parser/grammar.rs"
);

use crate::ast::Program;
use crate::lexer::Location;
use crate::lexer::Spanned;
use crate::token::Token;
use failure::Error;
use grammar::ProgramParser;

pub struct Parser {
    parser: ProgramParser,
}

#[derive(Debug, Fail)]
#[fail(display = "parse error occured")]
pub struct ParseError {
    #[cause]
    pub error: lalrpop_util::ParseError<Location, Token, Error>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            parser: ProgramParser::new(),
        }
    }

    pub fn parse<Tokens: IntoIterator<Item = Spanned<Token, Location, Error>>>(
        self,
        tokens: Tokens,
    ) -> Result<Program, Error> {
        match self.parser.parse(tokens) {
            Ok(v) => Ok(v),
            Err(error) => Err(ParseError { error }.into()),
        }
    }
}
