use crate::symbol_table::*;
use std::io;
use std::io::Write;

pub trait AsTab {
    fn as_table(&self, f: &mut dyn Write) -> io::Result<()>;
}

impl AsTab for SymbolTable {
    fn as_table(&self, f: &mut dyn Write) -> io::Result<()> {
        let global_functions = self
            .global()
            .symbols
            .iter()
            .enumerate()
            .filter_map(|(i, s)| {
                if let SymbolKind::Function { symbol_table, .. } = &s.kind {
                    Some((i, *symbol_table))
                } else {
                    None
                }
            });

        for (i, symbol_table) in global_functions {
            let i = i + 1;
            let symbol_table = &self.tables[symbol_table].symbols;

            writeln!(f, "------------------------------------------")?;
            writeln!(f, "base = {}", i)?;
            writeln!(f, "sommet = {}", i + symbol_table.len())?;

            let it = self
                .global()
                .symbols
                .iter()
                .take(i)
                .chain(symbol_table.iter())
                .enumerate();

            for (i, e) in it {
                write!(f, "{} ", i)?;
                e.as_table(f)?;
            }
            writeln!(f, "------------------------------------------")?;
        }

        Ok(())
    }
}

impl AsTab for Symbol {
    fn as_table(&self, f: &mut dyn Write) -> io::Result<()> {
        use SymbolKind::*;

        let (scope, kind, additional) = match self.kind {
            Scalar { scope } => (scope, "ENTIER", 1),
            Vector { scope, size } => (scope, "TABLEAU", size as usize),
            Function { nb_arguments, .. } => (Scope::Global, "FONCTION", nb_arguments),
        };

        write!(f, "{} ", self.id)?;
        scope.as_table(f)?;
        writeln!(f, " {} {} {}", kind, self.address, additional)
    }
}

impl AsTab for Scope {
    fn as_table(&self, f: &mut dyn Write) -> io::Result<()> {
        use Scope::*;

        match self {
            Global => write!(f, "GLOBALE"),
            Local => write!(f, "LOCALE"),
            Argument => write!(f, "ARGUMENT"),
        }
    }
}
