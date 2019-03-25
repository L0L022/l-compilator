#[derive(Debug)]
pub struct SymbolTable {
    pub tables: Vec<Table>,
}

#[derive(Debug)]
pub struct Table {
    pub parent: Option<usize>,
    pub symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            tables: vec![Table::new()],
        }
    }

    pub fn global(&self) -> &Table {
        &self.tables[0]
    }

    pub fn new_table(&mut self, parent: Option<usize>) -> usize {
        self.tables.push(Table {
            parent,
            symbols: Vec::new(),
        });

        self.tables.len() - 1
    }

    pub fn iter<'a>(&'a self, from: usize) -> Box<Iterator<Item = &'a Symbol> + 'a> {
        let it = self.tables[from].symbols.iter();

        if let Some(parent) = &self.tables[from].parent {
            let it2 = self.tables[*parent].symbols.iter();

            return Box::new(it2.chain(it));
        }

        Box::new(it)
    }
}

impl Table {
    pub fn new() -> Self {
        Self {
            parent: None,
            symbols: Vec::new(),
        }
    }

    pub fn with_parent(parent: usize) -> Self {
        Self {
            parent: Some(parent),
            symbols: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Symbol {
    pub id: String,
    pub address: usize,
    pub kind: SymbolKind,
}

impl Symbol {
    pub fn is_function(&self) -> bool {
        if let SymbolKind::Function { .. } = self.kind {
            return true;
        }

        false
    }
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
        nb_arguments: usize,
        symbol_table: usize,
    },
}

#[derive(Debug, Copy, Clone)]
pub enum Scope {
    Global,
    Local,
    Argument,
}
