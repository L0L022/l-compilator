use crate::ast::*;
use crate::symbol_table::Scope;
use crate::symbol_table::Symbol;
use crate::symbol_table::SymbolKind;
use crate::symbol_table::SymbolTable;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Fail)]
#[fail(display = "semantic(s) error occured: {:?}", diagnostics)]
pub struct Error {
    pub diagnostics: Vec<diagnostic::Diagnostic>,
}

mod diagnostic {
    #[derive(Debug)]
    pub enum Diagnostic {
        Error(Error),
        Warning(Warning),
    }

    #[derive(Debug)]
    pub enum Error {
        AlreadyDeclared,
        Undeclared,
        VectorWithoutIndice,
        ScalarWithIndice,
        TypeConversion,
        InvalidFunctionArguments,
        MainUndeclared,
    }

    #[derive(Debug)]
    pub enum Warning {
        VariableShadowing,
    }
}

pub struct Data {
    pub table: Rc<RefCell<SymbolTable>>,
    pub errors: Vec<diagnostic::Diagnostic>,
    pub scope: Scope,
    pub address: u32,
}

pub trait Analyse {
    fn analyse(&self, d: &mut Data);
}

impl<T: Analyse> Analyse for [T] {
    fn analyse(&self, d: &mut Data) {
        for i in self {
            i.analyse(d);
        }
    }
}

impl Analyse for Program {
    fn analyse(&self, d: &mut Data) {
        self.0.analyse(d);
    }
}

impl Analyse for Statement {
    fn analyse(&self, d: &mut Data) {
        use Statement::*;

        match self {
            DclVariable(v) => match v {
                Variable::Scalar(s) => {
                    let (_, id) = s;
                    d.table.borrow_mut().symbols.push(Symbol {
                        id: id.to_string(),
                        address: 0,
                        kind: SymbolKind::Scalar { scope: d.scope },
                    });
                }
                Variable::Vector(v) => {
                    let (_, size, id) = v;
                    d.table.borrow_mut().symbols.push(Symbol {
                        id: id.to_string(),
                        address: 0,
                        kind: SymbolKind::Vector {
                            scope: d.scope,
                            size: *size,
                        },
                    });
                }
            },
            DclFunction(id, args, vars, instructions) => {
                let table = Rc::new(RefCell::new(SymbolTable::with_parent(&d.table)));
                d.table.borrow_mut().symbols.push(Symbol {
                    id: id.to_string(),
                    address: 0,
                    kind: SymbolKind::Function {
                        nb_arguments: args.len() as i32,
                        symbol_table: table.clone(),
                    },
                });
                d.table = table.clone();
                d.scope = Scope::Argument;

                for (_, id) in args {
                    d.table.borrow_mut().symbols.push(Symbol {
                        id: id.clone(),
                        address: 0,
                        kind: SymbolKind::Scalar {
                            scope: Scope::Argument,
                        },
                    });
                }
                d.scope = Scope::Local;
                for (_, id) in vars {
                    d.table.borrow_mut().symbols.push(Symbol {
                        id: id.clone(),
                        address: 0,
                        kind: SymbolKind::Scalar {
                            scope: Scope::Local,
                        },
                    });
                }

                //instructions.analyse(d);

                let p = if let Some(p) = &d.table.borrow().parent {
                    p.upgrade()
                } else {
                    None
                };

                if let Some(p) = p {
                    d.table = p.clone();
                }
            }
        }
    }
}
/*
impl Analyse for [Instruction] {
    fn analyse(&self, d: &mut Data) {
        for i in self {
            i.analyse(d);
        }
    }
}

impl Analyse for Instruction {
    fn analyse(&self, d: &mut Data) {
        use Instruction::*;

        // match self {
        //     _ => d.errors.push(diagnostic::Error::AlreadyDeclared),
        // }
    }
}
*/
