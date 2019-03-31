struct Instruction {
    label: Option<Label>,
    kind: InstructionKind,
    comment: Option<String>,
}

enum InstructionKind {
    Jump,
    Cmp,
    Call,
    Ret,
    Mov,
    Push,
    Pop,
    Sub,
    Add,
}

enum Register {
    EAX,
    EBX,
    ECX,
    EDX,
    EIP,
    ESP,
    EBP,
}

struct Label(String);
struct Constant(i32);

struct Pointer {
    label: Label,
    offset: i32,
}

enum UnaryOperand {
    R(Register),
    P(Pointer),
}

enum BinaryOperand {
    RR(RR),
    RP(RP),
    PR(PR),
}

type RR = (Register, Register);
type RP = (Register, Pointer);
type PR = (Pointer, Register);
