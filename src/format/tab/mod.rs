pub mod tab;
#[cfg(test)]
mod tests;

use crate::symbol_table::SymbolTable;
use std::io;
use std::io::Write;

impl SymbolTable {
    pub fn as_table(&self, f: &mut dyn Write) -> io::Result<()> {
        tab::AsTab::as_table(self, f)
    }
}
