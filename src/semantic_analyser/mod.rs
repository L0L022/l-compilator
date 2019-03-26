use crate::semantic_analyser::analyse::{Data, Error};
use crate::symbol_table::SymbolTable;
use failure::Fallible;

mod analyse;

pub trait Analyse {
    fn analyse(&self) -> Fallible<SymbolTable>;
}

// TODO return warning

impl<T: analyse::Analyse> Analyse for T {
    fn analyse(&self) -> Fallible<SymbolTable> {
        let mut symbol_table = SymbolTable::new();
        let mut d = Data::new(&mut symbol_table);

        self.analyse(&mut d);

        if d.errors.is_empty() {
            Ok(symbol_table)
        } else {
            Err(Error {
                diagnostics: d.errors,
            }
            .into())
        }
    }
}
