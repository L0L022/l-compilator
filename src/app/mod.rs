mod opt;

use crate::{format::asynt::Asynt, lexer::Lexer, parser::Parser};
use failure::{Error, ResultExt};
use opt::Opt;
use std::fs::read_to_string;
use structopt::StructOpt;

pub struct App;

impl App {
    pub fn run() -> Result<(), Error> {
        let opt = Opt::from_args();

        let content = read_to_string(&opt.source_file)
            .with_context(|_| format!("could not read file {:?}", opt.source_file))?;

        if opt.lex {
            Self::print_lex(&content)?;
        }

        if opt.ast {
            Self::print_ast(&content)?;
        }

        Ok(())
    }

    fn print_lex(content: &str) -> Result<(), Error> {
        print!("{}", Lexer::new(&content).into_lex()?);

        Ok(())
    }

    fn print_ast(content: &str) -> Result<(), Error> {
        let l = Lexer::new(&content);
        let p = Parser::new();

        print!("{}", p.parse(l)?.to_asynt(0));

        Ok(())
    }
}
