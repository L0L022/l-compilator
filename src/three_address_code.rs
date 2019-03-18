struct Constant(i32);
struct Label(String);
struct TempVar(u32);
struct Variable(String);

enum CTV {
    C(Constant),
    T(TempVar),
    V(Variable),
}

enum TV {
    T(TempVar),
    V(Variable),
}

enum Instruction {
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
        variable: Variable,
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
        goto: Label,
    },
}

enum ArithmeticOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

enum JumpIfCondition {
    Less,
    LessOrEqual,
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
}
