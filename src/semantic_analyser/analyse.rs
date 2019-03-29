use crate::ast::*;
use crate::symbol_table::Scope;
use crate::symbol_table::Symbol;
use crate::symbol_table::SymbolKind;
use crate::symbol_table::SymbolTable;
use crate::symbol_table::Table;

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

pub struct Data<'t> {
    pub symbol_table: &'t mut SymbolTable,
    pub current_table: usize,
    pub errors: Vec<diagnostic::Diagnostic>,
    pub scope: Scope,
    pub address: usize,
}

impl<'t> Data<'t> {
    pub fn new(symbol_table: &'t mut SymbolTable) -> Self {
        Self {
            symbol_table,
            current_table: 0,
            errors: Vec::new(),
            scope: Scope::Global,
            address: 0,
        }
    }

    pub fn table(&mut self) -> &mut Table {
        &mut self.symbol_table.tables[self.current_table]
    }

    fn already_declared_variable(&self, id: &str) -> bool {
        self.symbol_table.iter(self.current_table).any(|symbol| {
            if symbol.is_function() || symbol.id != id {
                return false;
            }

            let scope = match symbol.kind {
                SymbolKind::Scalar { scope, .. } => scope,
                SymbolKind::Vector { scope, .. } => scope,
                SymbolKind::Function { .. } => unreachable!(),
            };

            if self.scope == scope {
                return true;
            }

            if self.scope == Scope::Local && scope == Scope::Argument {
                return true;
            }

            false
        })
    }
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

        let main_exists = d
            .symbol_table
            .global()
            .symbols
            .iter()
            .any(|symbol| symbol.is_function() && symbol.id == "main");

        if !main_exists {
            d.errors.push(diagnostic::Diagnostic::Error(
                diagnostic::Error::MainUndeclared,
            ));
        }
    }
}

impl Analyse for Statement {
    fn analyse(&self, d: &mut Data) {
        use Statement::*;

        match self {
            DclVariable(v) => v.analyse(d),
            DclFunction(id, args, vars, instructions) => {
                let exists = d
                    .symbol_table
                    .iter(d.current_table)
                    .any(|symbol| symbol.is_function() && symbol.id == *id);

                if exists {
                    d.errors.push(diagnostic::Diagnostic::Error(
                        diagnostic::Error::AlreadyDeclared,
                    ));
                    return;
                }

                let table = d.symbol_table.new_table(Some(d.current_table));
                d.table().symbols.push(Symbol {
                    id: id.clone(),
                    address: 0,
                    kind: SymbolKind::Function {
                        nb_arguments: args.len(),
                        symbol_table: table,
                    },
                });
                d.current_table = table;

                d.scope = Scope::Argument;
                d.address = 0;
                args.analyse(d);

                d.scope = Scope::Local;
                d.address = 0;
                vars.analyse(d);

                instructions.analyse(d);

                if let Some(parent) = d.table().parent {
                    d.current_table = parent;
                }
            }
        }
    }
}

impl Analyse for Variable {
    fn analyse(&self, d: &mut Data) {
        use Variable::*;

        match self {
            Scalar(s) => s.analyse(d),
            Vector(v) => v.analyse(d),
        }
    }
}

impl Analyse for Scalar {
    fn analyse(&self, d: &mut Data) {
        let (t, id) = self;

        if d.already_declared(id) {
            d.errors.push(diagnostic::Diagnostic::Error(
                diagnostic::Error::AlreadyDeclared,
            ));
            return;
        }

        let s = Symbol {
            id: id.clone(),
            address: d.address,
            kind: SymbolKind::Scalar { scope: d.scope },
        };
        d.table().symbols.push(s);
        d.address += t.size();
    }
}

impl Analyse for Vector {
    fn analyse(&self, d: &mut Data) {
        let (t, size, id) = self;

        if d.already_declared(id) {
            d.errors.push(diagnostic::Diagnostic::Error(
                diagnostic::Error::AlreadyDeclared,
            ));
            return;
        }

        let s = Symbol {
            id: id.clone(),
            address: d.address,
            kind: SymbolKind::Vector {
                scope: d.scope,
                size: *size,
            },
        };
        d.table().symbols.push(s);
        d.address += t.size() * (*size) as usize;
    }
}

impl Analyse for Instruction {
    fn analyse(&self, d: &mut Data) {
        use Instruction::*;

        match self {
            Affectation(lv, e) => {
                lv.analyse(d);
                e.analyse(d);
            }
            CallFunction(c) => {
                c.analyse(d);
            }
            Return(e) => {
                e.analyse(d);
            }
            If(e, i1, i2) => {
                e.analyse(d);
                i1.analyse(d);
                i2.analyse(d);
            }
            While(e, i) => {
                e.analyse(d);
                i.analyse(d);
            }
            WriteFunction(e) => {
                e.analyse(d);
            }
            NOP => {}
        }
    }
}

impl Analyse for Expression {
    fn analyse(&self, d: &mut Data) {
        use Expression::*;

        match self {
            Value(_) => {}
            LeftValue(lv) => {
                lv.analyse(d);
            }
            CallFunction(c) => {
                c.analyse(d);
            }
            ReadFunction => {}
            UnaryOperation(_, e) => {
                e.analyse(d);
            }
            BinaryOperation(_, e1, e2) => {
                e1.analyse(d);
                e2.analyse(d);
            }
        }
    }
}

impl Analyse for LeftValue {
    fn analyse(&self, d: &mut Data) {
        use LeftValue::*;
        use SymbolKind::*;

        match self {
            Variable(id) => {
                let symbol = d
                    .symbol_table
                    .iter(d.current_table)
                    .find(|symbol| symbol.id == *id && !symbol.is_function());

                if let Some(symbol) = symbol {
                    match symbol.kind {
                        Scalar { .. } => {
                            return;
                        }
                        Vector { .. } => {
                            d.errors.push(diagnostic::Diagnostic::Error(
                                diagnostic::Error::VectorWithoutIndice,
                            ));
                            return;
                        }
                        Function { .. } => unreachable!(),
                    }
                }
            }
            VariableAt(id, _) => {
                let symbol = d
                    .symbol_table
                    .iter(d.current_table)
                    .find(|symbol| symbol.id == *id && !symbol.is_function());

                if let Some(symbol) = symbol {
                    match symbol.kind {
                        Scalar { .. } => {
                            d.errors.push(diagnostic::Diagnostic::Error(
                                diagnostic::Error::ScalarWithIndice,
                            ));
                            return;
                        }
                        Vector { .. } => {
                            return;
                        }
                        Function { .. } => unreachable!(),
                    }
                }
            }
        }

        d.errors
            .push(diagnostic::Diagnostic::Error(diagnostic::Error::Undeclared));
    }
}

impl Analyse for CallFunction {
    fn analyse(&self, d: &mut Data) {
        let (id, expressions) = self;

        let symbol = d
            .symbol_table
            .iter(d.current_table)
            .find(|symbol| symbol.id == *id && symbol.is_function());

        if let Some(symbol) = symbol {
            if let SymbolKind::Function { nb_arguments, .. } = symbol.kind {
                if nb_arguments != expressions.len() {
                    d.errors.push(diagnostic::Diagnostic::Error(
                        diagnostic::Error::InvalidFunctionArguments,
                    ));
                }
                return;
            }
        }

        d.errors
            .push(diagnostic::Diagnostic::Error(diagnostic::Error::Undeclared));
    }
}
