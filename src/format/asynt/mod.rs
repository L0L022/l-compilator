pub mod asynt;
#[cfg(test)]
mod tests;

use std::io::{Result, Write};

pub trait Asynt {
    fn to_asynt(&self, f: &mut dyn Write, indent: usize) -> Result<()>;
}

impl<T: asynt::Asynt> Asynt for T {
    fn to_asynt(&self, f: &mut dyn Write, indent: usize) -> Result<()> {
        self.to_asynt(f, indent)
    }
}
