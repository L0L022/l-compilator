mod gen;

use crate::gen_three_address_code::gen::{Data, Gen};
use crate::symbol_table::SymbolTable;
use crate::three_address_code::ThreeAddressCode;

pub trait GenThreeAddressCode {
    fn gen_three_address_code(
        &self,
        symbol_table: &SymbolTable,
        current_table: usize,
    ) -> ThreeAddressCode;
}

impl<T: Gen<()>> GenThreeAddressCode for T {
    fn gen_three_address_code(
        &self,
        symbol_table: &SymbolTable,
        current_table: usize,
    ) -> ThreeAddressCode {
        let mut d = Data::new(symbol_table, current_table);
        self.gen(&mut d);
        d.into()
    }
}
