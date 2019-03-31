use crate::three_address_code::*;
use std::io;
use std::io::Write;

pub trait ThreeA {
    fn three_a(&self, f: &mut dyn Write) -> io::Result<()>;
}

impl ThreeA for ThreeAddressCode {
    fn three_a(&self, f: &mut dyn Write) -> io::Result<()> {
        for (i, instr) in self.instructions.iter().enumerate() {
            write!(f, "{:04}", i)?;
            instr.three_a(f)?;
            writeln!(f)?;
        }

        Ok(())
    }
}

impl ThreeA for Constant {
    fn three_a(&self, f: &mut dyn Write) -> io::Result<()> {
        write!(f, "{}", self.constant())
    }
}

impl ThreeA for Label {
    fn three_a(&self, f: &mut dyn Write) -> io::Result<()> {
        write!(f, "{}", self.label())
    }
}

impl ThreeA for Temp {
    fn three_a(&self, f: &mut dyn Write) -> io::Result<()> {
        write!(f, "t{}", self.temp())
    }
}

impl ThreeA for Variable {
    fn three_a(&self, f: &mut dyn Write) -> io::Result<()> {
        write!(f, "{}", self.id())?;

        if let Some(indice) = self.indice() {
            write!(f, "[")?;
            indice.three_a(f)?;
            write!(f, "]")?;
        }

        Ok(())
    }
}

impl ThreeA for CTV {
    fn three_a(&self, f: &mut dyn Write) -> io::Result<()> {
        match self {
            CTV::C(c) => c.three_a(f),
            CTV::T(t) => t.three_a(f),
            CTV::V(v) => v.three_a(f),
        }
    }
}

impl ThreeA for TV {
    fn three_a(&self, f: &mut dyn Write) -> io::Result<()> {
        match self {
            TV::T(t) => t.three_a(f),
            TV::V(v) => v.three_a(f),
        }
    }
}

impl ThreeA for CT {
    fn three_a(&self, f: &mut dyn Write) -> io::Result<()> {
        match self {
            CT::C(c) => c.three_a(f),
            CT::T(t) => t.three_a(f),
        }
    }
}

impl ThreeA for Instruction {
    fn three_a(&self, f: &mut dyn Write) -> io::Result<()> {
        match &self.label {
            Some(label) => write!(f, " >{:8}", label.label())?,
            None => write!(f, "{:10}", "")?,
        }
        write!(f, " : ")?;

        const LINE_LENGTH: usize = 50;
        let mut instr = Vec::with_capacity(LINE_LENGTH);
        self.kind.three_a(&mut instr)?;
        write!(f, "{:1$}", String::from_utf8_lossy(&instr), LINE_LENGTH)?;

        if let Some(comment) = &self.comment {
            write!(f, "; {}", comment)?;
        }

        Ok(())
    }
}

impl ThreeA for InstructionKind {
    fn three_a(&self, f: &mut dyn Write) -> io::Result<()> {
        use InstructionKind::*;

        match self {
            Arithmetic {
                operator,
                left,
                right,
                result,
            } => {
                let operator = match operator {
                    ArithmeticOperator::Addition => "+",
                    ArithmeticOperator::Subtraction => "-",
                    ArithmeticOperator::Multiplication => "*",
                    ArithmeticOperator::Division => "/",
                };

                result.three_a(f)?;
                write!(f, " = ")?;
                left.three_a(f)?;
                write!(f, " {} ", operator)?;
                right.three_a(f)?;
            }
            Affectation { value, result } => {
                result.three_a(f)?;
                write!(f, " = ")?;
                value.three_a(f)?;
            }
            Allocation { variable, size } => {
                write!(f, "alloc ")?;
                size.three_a(f)?;
                if let Some(variable) = variable {
                    write!(f, " ")?;
                    variable.three_a(f)?;
                }
            }
            ReadFunction { result } => {
                result.three_a(f)?;
                write!(f, " = read")?;
            }
            WriteFunction { value } => {
                write!(f, "write ")?;
                value.three_a(f)?;
            }
            FunctionCall { function, result } => {
                result.three_a(f)?;
                write!(f, " = ")?;
                function.three_a(f)?;
            }
            FunctionBegin => {
                write!(f, "fbegin")?;
            }
            FunctionEnd => {
                write!(f, "fend")?;
            }
            FunctionPushArg { arg } => {
                write!(f, "param ")?;
                arg.three_a(f)?;
            }
            FunctionReturn { value } => {
                write!(f, "ret ")?;
                value.three_a(f)?;
            }
            Jump { label } => {
                write!(f, "goto ")?;
                label.three_a(f)?;
            }
            JumpIf {
                condition,
                left,
                right,
                label,
            } => {
                let condition = match condition {
                    JumpIfCondition::Less => "<",
                    JumpIfCondition::LessOrEqual => "<=",
                    JumpIfCondition::Equal => "==",
                    JumpIfCondition::NotEqual => "!=",
                    JumpIfCondition::Greater => ">",
                    JumpIfCondition::GreaterOrEqual => ">=",
                };

                write!(f, "if ")?;
                left.three_a(f)?;
                write!(f, " {} ", condition)?;
                right.three_a(f)?;
                write!(f, " goto ")?;
                label.three_a(f)?;
            }
            NOP => {}
        }

        Ok(())
    }
}
