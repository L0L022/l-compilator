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
        let self_it = self.tables[from].symbols.iter().rev();

        if let Some(parent) = &self.tables[from].parent {
            let parent_it = self.iter(*parent);

            return Box::new(self_it.chain(parent_it));
        }

        Box::new(self_it)
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Scope {
    Global,
    Local,
    Argument,
}
