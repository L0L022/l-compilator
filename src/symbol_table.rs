use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::rc::Weak;

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

pub struct Symbol {
    pub id: String,
    pub address: i32,
    pub kind: SymbolKind,
}

pub enum SymbolKind {
    Scalar {
        scope: Scope,
    },
    Vector {
        scope: Scope,
        size: i32,
    },
    Function {
        nb_arguments: i32,
        symbol_table: Rc<RefCell<SymbolTable>>,
    },
}

pub enum Scope {
    Global,
    Local,
    Argument,
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Scope::*;

        match self {
            Global => write!(f, "GLOBALE"),
            Local => write!(f, "LOCALE"),
            Argument => write!(f, "ARGUMENT"),
        }
    }
}

impl fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "------------------------------------------")?;
        writeln!(f, "base = {}", self.base)?;
        writeln!(f, "sommet = {}", self.sommet)?;
        for (i, e) in self.tab.iter().enumerate() {
            write!(f, "{} {}", i, e)?;
        }
        writeln!(f, "------------------------------------------")
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use SymbolKind::*;

        let (scope, kind, additional) = match self.kind {
            Scalar { scope } => (scope, "ENTIER", 1),
            Vector { scope, size } => (scope, "TABLEAU", size),
            Function { nb_arguments } => (Scope::Global, "FONCTION", nb_arguments),
        };

        writeln!(
            f,
            "{} {} {} {} {}",
            self.id, scope, kind, self.address, additional
        )
    }
}
