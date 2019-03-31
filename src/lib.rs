extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate lalrpop_util;
extern crate codespan;
extern crate codespan_reporting;
extern crate structopt;

mod app;
mod assembler;
mod ast;
mod format;
mod gen_three_address_code;
mod lexer;
mod parser;
mod semantic_analyser;
mod symbol_table;
mod three_address_code;
mod token;

pub use app::App;
