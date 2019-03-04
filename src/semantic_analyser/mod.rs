use crate::ast::*;
use crate::symbol_table::SymbolTable;
use failure::Fallible;

// #[derive(Debug, Fail)]
// #[fail(display = "semantic error occured at: {:?}", span)]
// struct Error {
//     pub span: Range<crate::lexer::Location>,
//     //#[cause]
//     //pub kind: ErrorKind,
// }

enum Error {
    AlreadyDeclared,
    Undeclared,
    VectorWithoutIndice,
    ScalarWithIndice,
    TypeConversion,
    InvalidFunctionArguments,
    MainUndeclared,
}

enum Warning {
    VariableShadowing,
}

struct Data {
    table: SymbolTable,
    errors: Vec<Error>,
}

impl SemanticAnalyser {
    fn analyse() -> Fallible<()> {
        Ok(())
    }
}

trait Analyse {
    fn analyse(&self, d: Data);
}

impl Analyse for Program {
    fn analyse(&self, d: Data) {
        self.analyse(d);
    }
}

impl Analyse for Statement {
    fn analyse(&self, d: Data) {
        use Statement::*;

        match self {
            DclVariable(..) => {}
            DclFunction(_, _, _, instructions) => {
                instructions.analyse(d);
            }
        }
    }
}

impl Analyse for [Instruction] {
    fn analyse(&self, d: Data) {
        for i in self {
            i.analyse(d);
        }
    }
}

impl Analyse for Instruction {
    fn analyse(&self, d: Data) {
        use Instruction::*;

        match self {
            _ => d.errors.push(Error::AlreadyDeclared),
        }
    }
}
