use crate::symbol_table::Scope;
use std::cell::RefCell;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Constant(i32);

impl Constant {
    pub fn new<T: Into<i32>>(c: T) -> Self {
        Constant(c.into())
    }

    pub fn constant(&self) -> i32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label(Rc<String>);

impl Label {
    pub fn new(label: String) -> Self {
        Label(Rc::new(label))
    }

    pub fn label(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Temp(u32, Rc<RefCell<i32>>);

impl Hash for Temp {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Temp {
    pub fn new<T: Into<u32>>(t: T) -> Self {
        Temp(t.into(), Rc::new(RefCell::new(-1)))
    }

    pub fn temp(&self) -> u32 {
        self.0
    }

    pub fn last_use(&self) -> i32 {
        *self.1.borrow()
    }

    pub fn set_last_use(&self, last_use: i32) {
        *self.1.borrow_mut() = last_use;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variable {
    id: Rc<String>,
    indice: Option<CT>,
    address: usize,
    scope: Scope,
}

impl Variable {
    pub fn new(id: String, indice: Option<CT>, address: usize, scope: Scope) -> Self {
        Variable {
            id: Rc::new(id),
            indice,
            address,
            scope,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn indice(&self) -> &Option<CT> {
        &self.indice
    }

    pub fn address(&self) -> usize {
        self.address
    }

    pub fn scope(&self) -> Scope {
        self.scope
    }
}

#[derive(Debug, Clone)]
pub enum CTV {
    C(Constant),
    T(Temp),
    V(Variable),
}

impl CTV {
    pub fn is_constant(&self) -> bool {
        match self {
            CTV::C(_) => true,
            _ => false,
        }
    }

    pub fn is_temp(&self) -> bool {
        match self {
            CTV::T(_) => true,
            _ => false,
        }
    }

    pub fn is_variable(&self) -> bool {
        match self {
            CTV::V(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TV {
    T(Temp),
    V(Variable),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CT {
    C(Constant),
    T(Temp),
}

impl From<Constant> for CTV {
    fn from(c: Constant) -> Self {
        CTV::C(c)
    }
}

impl From<Temp> for CTV {
    fn from(t: Temp) -> Self {
        CTV::T(t)
    }
}

impl From<Variable> for CTV {
    fn from(v: Variable) -> Self {
        CTV::V(v)
    }
}

impl From<TV> for CTV {
    fn from(tv: TV) -> Self {
        match tv {
            TV::T(t) => CTV::T(t),
            TV::V(v) => CTV::V(v),
        }
    }
}

impl From<CT> for CTV {
    fn from(ct: CT) -> Self {
        match ct {
            CT::C(c) => CTV::C(c),
            CT::T(t) => CTV::T(t),
        }
    }
}

impl From<Temp> for TV {
    fn from(t: Temp) -> Self {
        TV::T(t)
    }
}

impl From<Variable> for TV {
    fn from(v: Variable) -> Self {
        TV::V(v)
    }
}

impl From<Constant> for CT {
    fn from(c: Constant) -> Self {
        CT::C(c)
    }
}

impl From<Temp> for CT {
    fn from(t: Temp) -> Self {
        CT::T(t)
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub label: Option<Label>,
    pub kind: InstructionKind,
    pub comment: Option<String>,
}

#[derive(Debug)]
pub enum InstructionKind {
    Arithmetic {
        operator: ArithmeticOperator,
        left: CTV,
        right: CTV,
        result: TV,
    },
    Affectation {
        value: CTV,
        result: TV,
    },
    Allocation {
        variable: Option<Variable>,
        size: Constant,
    },
    ReadFunction {
        result: TV,
    },
    WriteFunction {
        value: CTV,
    },
    FunctionCall {
        function: Label,
        result: TV,
    },
    FunctionBegin,
    FunctionEnd,
    FunctionPushArg {
        arg: CTV,
    },
    FunctionReturn {
        value: CTV,
    },
    Jump {
        label: Label,
    },
    JumpIf {
        condition: JumpIfCondition,
        left: CTV,
        right: CTV,
        label: Label,
    },
    NOP,
}

#[derive(Debug)]
pub enum ArithmeticOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum JumpIfCondition {
    Less,
    LessOrEqual,
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
}

#[derive(Debug)]
pub struct ThreeAddressCode {
    pub instructions: Vec<Instruction>,
}
