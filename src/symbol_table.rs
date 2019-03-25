use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
pub struct SymbolTable {
    pub parent: Option<Weak<RefCell<SymbolTable>>>,
    pub symbols: Vec<Symbol>,
}

use std::cell::Ref;

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            parent: None,
            symbols: Vec::new(),
        }
    }

    pub fn with_parent(parent: &Rc<RefCell<SymbolTable>>) -> Self {
        Self {
            parent: Some(Rc::downgrade(parent)),
            symbols: Vec::new(),
        }
    }

    // pub fn iter<'a>(&'a self) -> Box<Iterator<Item = &'a Symbol> + 'a> {
    //     let it = self.symbols.iter();
    //
    //     if let Some(parent) = &self.parent {
    //         if let Some(parent) = parent.upgrade() {
    //             let it2 = parent.borrow().iter();
    //
    //             return Box::new(it2.chain(it));
    //         }
    //     }
    //
    //     Box::new(it)
    // }
}

#[derive(Debug)]
pub struct Symbol {
    pub id: String,
    pub address: u32,
    pub kind: SymbolKind,
}

#[derive(Debug)]
pub enum SymbolKind {
    Scalar {
        scope: Scope,
    },
    Vector {
        scope: Scope,
        size: u32,
    },
    Function {
        nb_arguments: u32,
        symbol_table: Rc<RefCell<SymbolTable>>,
    },
}

#[derive(Debug, Copy, Clone)]
pub enum Scope {
    Global,
    Local,
    Argument,
}
