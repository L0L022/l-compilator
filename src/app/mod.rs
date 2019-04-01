mod as_diagnostic;
mod opt;

use crate::c_code;
use crate::format::three_a::ThreeA;
use crate::gen_three_address_code::GenThreeAddressCode;
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

            if opt.three_address_code {
                Self::print_three_a(&content)?;
            }

            if opt.nasm {
                Self::print_nasm(&content)?;
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
            .as_table(&mut std::io::stdout().lock())?;

        Ok(())
    }

    fn print_three_a(content: &str) -> Fallible<()> {
        let l = Lexer::new(&content);
        let p = Parser::new();

        let ast = p.parse(l)?;
        let symbol_table = ast.analyse()?;
        ast.gen_three_address_code(&symbol_table, 0)
            .three_a(&mut std::io::stdout().lock())?;

        Ok(())
    }

    fn print_nasm(content: &str) -> Fallible<()> {
        let l = Lexer::new(&content);
        let p = Parser::new();

        let ast = p.parse(l)?;
        let symbol_table = ast.analyse()?;
        c_code::print_nasm(&ast.gen_three_address_code(&symbol_table, 0));

        Ok(())
    }
}
