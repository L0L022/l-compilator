mod as_diagnostic;
mod opt;

use crate::format::tab::AsTab;
use crate::semantic_analyser::Analyse;
use crate::{format::asynt::Asynt, lexer::Lexer, parser::Parser};
use as_diagnostic::AsDiagnostic;
use codespan::CodeMap;
use codespan_reporting::{emit, termcolor::StandardStream};
use failure::{Error, Fallible, ResultExt};
use opt::Opt;
use structopt::StructOpt;

pub struct App;

impl App {
    pub fn run() -> Result<(), Error> {
        let opt = Opt::from_args();

        let mut code_map = CodeMap::new();
        let file_map = code_map
            .add_filemap_from_disk(&opt.source_file)
            .with_context(|_| format!("could not read file {:?}", opt.source_file))?;
        let content = file_map.src();
        let mut diagnostics = Vec::new();

        let res: &Fn() -> Fallible<()> = &|| {
            if opt.lex {
                Self::print_lex(&content)?;
            }

            if opt.ast {
                Self::print_ast(&content)?;
            }

            if opt.symbol_table {
                Self::print_tab(&content)?;
            }

            Ok(())
        };

        if let Err(error) = res() {
            let error = error.as_diagnostic(&file_map).ok_or(error)?;
            diagnostics.push(error);
        }

        if !diagnostics.is_empty() {
            let writer = StandardStream::stderr(codespan_reporting::termcolor::ColorChoice::Auto);
            for diagnostic in &diagnostics {
                emit(&mut writer.lock(), &code_map, &diagnostic).unwrap();
                println!();
            }
            std::process::exit(1);
        }

        Ok(())
    }

    fn print_lex(content: &str) -> Fallible<()> {
        Lexer::new(&content).into_lex(&mut std::io::stdout().lock())
    }

    fn print_ast(content: &str) -> Fallible<()> {
        let l = Lexer::new(&content);
        let p = Parser::new();

        p.parse(l)?.to_asynt(&mut std::io::stdout().lock(), 0)?;

        Ok(())
    }

    fn print_tab(content: &str) -> Fallible<()> {
        let l = Lexer::new(&content);
        let p = Parser::new();

        p.parse(l)?
            .analyse()?
            .borrow()
            .as_table(&mut std::io::stdout().lock())?;

        Ok(())
    }
}
