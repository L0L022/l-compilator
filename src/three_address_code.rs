#[derive(Debug, Clone)]
pub struct Constant(pub i32);

#[derive(Debug, Clone)]
pub struct Label(pub String);

#[derive(Debug, Clone)]
pub struct Temp(pub u32);

#[derive(Debug, Clone)]
pub struct Variable {
    pub id: String,
    pub indice: Option<CT>,
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

#[derive(Debug, Clone)]
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

pub struct Instruction {
    pub label: Option<Label>,
    pub kind: InstructionKind,
    pub comment: Option<String>,
}

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
}

pub enum ArithmeticOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

pub enum JumpIfCondition {
    Less,
    LessOrEqual,
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
}
