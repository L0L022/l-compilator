use crate::semantic_analyser::analyse::{Data, Error};
use crate::symbol_table::SymbolTable;
use failure::Fallible;
use std::cell::RefCell;
use std::rc::Rc;

mod analyse;

pub trait Analyse {
    fn analyse(&self) -> Fallible<Rc<RefCell<SymbolTable>>>;
}

impl<T: analyse::Analyse> Analyse for T {
    fn analyse(&self) -> Fallible<Rc<RefCell<SymbolTable>>> {
        let table = Rc::new(RefCell::new(SymbolTable::new()));
        let mut d = Data::new(&table);

        self.analyse(&mut d);

        if d.errors.is_empty() {
            Ok(table)
        } else {
            Err(Error {
                diagnostics: d.errors,
            }
            .into())
        }
    }
}
