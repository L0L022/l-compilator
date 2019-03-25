use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
pub struct SymbolTable {
    pub parent: Option<Weak<RefCell<SymbolTable>>>,
    pub symbols: Vec<Symbol>,
}

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
