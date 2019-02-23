mod opt;

use crate::{format::asynt::Asynt, lexer::Lexer, parser::Parser};
use codespan::{ByteIndex, CodeMap, Span};
use codespan_reporting::{emit, termcolor::StandardStream, Diagnostic, Label, Severity};
use failure::{Error, ResultExt};
use opt::Opt;
use std::fmt::Write;
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

        if opt.lex {
            Self::print_lex(&content)?;
        }

        if opt.ast {
            if let Err(error) = Self::print_ast(&content) {
                match error.downcast::<crate::parser::ParseError>() {
                    Ok(parse_error) => {
                        use lalrpop_util::ParseError::*;

                        let parse_error = parse_error.error;
                        match parse_error {
                            UnrecognizedToken { token, expected } => {
                                let error = Diagnostic::new(
                                    Severity::Error,
                                    "An unexpected token was observed",
                                );

                                let (error, span) = if let Some(token) = token {
                                    let (start, .., end) = token;
                                    let span = Span::new(
                                        ByteIndex(start as u32 + 1),
                                        ByteIndex(end as u32 + 1),
                                    );

                                    (
                                        error.with_label(
                                            Label::new_primary(span)
                                                .with_message("unrecognized token"),
                                        ),
                                        span,
                                    )
                                } else {
                                    let end = file_map.span().end().to_usize() as u32;
                                    let span = Span::new(ByteIndex(end - 1), ByteIndex(end));

                                    (
                                        error.with_label(
                                            Label::new_primary(span)
                                                .with_message("unrecognized EOF"),
                                        ),
                                        span,
                                    )
                                };

                                let error = if !expected.is_empty() {
                                    let mut message = String::new();
                                    for (i, e) in expected.iter().enumerate() {
                                        let sep = match i {
                                            0 => "expected one of",
                                            _ if i < expected.len() - 1 => ",",
                                            // Last expected message to be written
                                            _ => " or",
                                        };
                                        write!(&mut message, "{} {}", sep, e)?;
                                    }
                                    error.with_label(
                                        Label::new_secondary(span).with_message(message),
                                    )
                                } else {
                                    error
                                };

                                diagnostics.push(error);
                            }
                            ExtraToken { token } => {
                                let (start, .., end) = token;
                                let span = Span::new(
                                    ByteIndex(start as u32 + 1),
                                    ByteIndex(end as u32 + 1),
                                );

                                let error = Diagnostic::new(
                                    Severity::Error,
                                    "An unexpected token was observed",
                                )
                                .with_label(Label::new_primary(span).with_message("extra token"));

                                diagnostics.push(error);
                            }
                            _ => {
                                return Err(crate::parser::ParseError { error: parse_error }.into());
                            }
                        }
                    }
                    Err(error) => return Err(error),
                }
            }
        }

        if !diagnostics.is_empty() {
            let writer = StandardStream::stderr(codespan_reporting::termcolor::ColorChoice::Auto);
            for diagnostic in &diagnostics {
                emit(&mut writer.lock(), &code_map, &diagnostic).unwrap();
                println!();
            }
        }

        Ok(())
    }

    fn print_lex(content: &str) -> Result<(), Error> {
        Lexer::new(&content).into_lex(&mut std::io::stdout().lock())
    }

    fn print_ast(content: &str) -> Result<(), Error> {
        let l = Lexer::new(&content);
        let p = Parser::new();

        p.parse(l)?.to_asynt(&mut std::io::stdout().lock(), 0)?;

        Ok(())
    }
}
