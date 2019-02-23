mod logos_token;
#[cfg(test)]
mod tests;

use crate::token::Token;
use failure::Error;
use logos::Logos;
use logos_token::LogosToken;

pub type Location = usize;

pub struct Lexer<'input> {
    lexer: logos::Lexer<LogosToken, &'input str>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            lexer: LogosToken::lexer(input),
        }
    }

    pub fn source(&self) -> &'input str {
        self.lexer.source
    }
}

#[derive(Debug, Fail)]
#[fail(
    display = "lexical error occured at [{:?}] with this token: {}",
    range, token
)]
pub struct LexicalError {
    token: String,
    range: std::ops::Range<Location>,
}

#[derive(Debug, Fail)]
#[fail(display = "undefined behavior")]
pub struct UndefinedBehavior;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, Location, Error>;

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
