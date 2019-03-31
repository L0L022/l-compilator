mod gen;

use crate::gen_three_address_code::gen::{Data, Gen};
use crate::three_address_code::ThreeAddressCode;

pub trait GenThreeAddressCode {
    fn gen_three_address_code(&self) -> ThreeAddressCode;
}

impl<T: Gen<()>> GenThreeAddressCode for T {
    fn gen_three_address_code(&self) -> ThreeAddressCode {
        let mut d = Data::new();
        self.gen(&mut d);
        d.into()
    }
}
