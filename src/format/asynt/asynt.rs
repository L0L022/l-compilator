use crate::ast::*;
use std::io::{Result, Write};

pub trait Asynt {
    fn to_asynt(&self, f: &mut dyn Write, indent: usize) -> Result<()> {
        if self.hide() {
            return Ok(());
        }

        if !self.with_tag() {
            return self.content(f, indent);
        }

        let spaces = " ".repeat(indent);
        let name = self.name();

        write!(f, "{}<{}>", spaces, name)?;
        if !self.one_line() {
            writeln!(f)?;
        }
        self.content(f, indent + 2)?;
        if !self.one_line() {
            write!(f, "{}", spaces)?;
        }
        writeln!(f, "</{}>", name)
    }
    fn name(&self) -> &'static str {
        ""
    }
    fn one_line(&self) -> bool {
        false
    }
    fn hide(&self) -> bool {
        false
    }
    fn with_tag(&self) -> bool {
        true
    }
    fn content(&self, f: &mut dyn Write, indent: usize) -> Result<()>;
}

impl Asynt for Program {
    fn name(&self) -> &'static str {
        "prog"
    }
    fn content(&self, f: &mut dyn Write, indent: usize) -> Result<()> {
        use Statement::*;

        let var = self
            .0
            .iter()
            .filter(|s| if let DclVariable(..) = s { true } else { false });

        let func = self
            .0
            .iter()
            .filter(|s| if let DclFunction(..) = s { true } else { false });

        (&var).to_asynt(f, indent)?;
        (&func).to_asynt(f, indent)
    }
}

impl<'a, 'b: 'a, I: Iterator<Item = &'b Statement> + Clone> Asynt for &'a I {
    fn name(&self) -> &'static str {
        "l_dec"
    }

    fn content(&self, f: &mut dyn Write, indent: usize) -> Result<()> {
        let mut it: I = (*self).clone();

        match it.next() {
            None => Ok(()),
            Some(s) => {
                s.to_asynt(f, indent)?;
                (&it).to_asynt(f, indent)
            }
        }
    }
}

impl Asynt for Statement {
    fn name(&self) -> &'static str {
        use Statement::*;

        match self {
            DclFunction(..) => "foncDec",
            _ => unreachable!(),
        }
    }

    fn with_tag(&self) -> bool {
        use Statement::*;

        match self {
            DclVariable(..) => false,
            _ => true,
        }
    }

    fn content(&self, f: &mut dyn Write, indent: usize) -> Result<()> {
        use Statement::*;

        match self {
            DclVariable(v) => v.to_asynt(f, indent),
            DclFunction(id, p, v, i) => {
                let spaces = " ".repeat(indent);
                writeln!(f, "{}{}", spaces, id)?;

                p.to_asynt(f, indent)?;
                v.to_asynt(f, indent)?;
                i.to_asynt(f, indent)
            }
        }
    }
}

impl Asynt for [Scalar] {
    fn name(&self) -> &'static str {
        "l_dec"
    }

    fn content(&self, f: &mut dyn Write, indent: usize) -> Result<()> {
        match self.len() {
            0 => Ok(()),
            _ => {
                self[0].to_asynt(f, indent)?;
                self[1..].to_asynt(f, indent)
            }
        }
    }
}

impl Asynt for Variable {
    fn with_tag(&self) -> bool {
        false
    }

    fn content(&self, f: &mut dyn Write, indent: usize) -> Result<()> {
        use Variable::*;

        match self {
            Scalar(s) => s.to_asynt(f, indent),
            Vector(v) => v.to_asynt(f, indent),
        }
    }
}

impl Asynt for Scalar {
    fn name(&self) -> &'static str {
        "varDec"
    }

    fn one_line(&self) -> bool {
        true
    }

    fn content(&self, f: &mut dyn Write, _indent: usize) -> Result<()> {
        write!(f, "{}", self.1)
    }
}

impl Asynt for Vector {
    fn name(&self) -> &'static str {
        "tabDec"
    }

    fn one_line(&self) -> bool {
        true
    }

    fn content(&self, f: &mut dyn Write, _indent: usize) -> Result<()> {
        write!(f, "{}[{}]", self.2, self.1)
    }
}

impl Asynt for [Instruction] {
    fn name(&self) -> &'static str {
        "l_instr"
    }

    fn hide(&self) -> bool {
        self.is_empty()
    }

    fn content(&self, f: &mut dyn Write, indent: usize) -> Result<()> {
        match self.len() {
            0 => Ok(()),
            1 => self[0].to_asynt(f, indent),
            _ => {
                self[0].to_asynt(f, indent)?;
                self[1..].to_asynt(f, indent)
            }
        }
    }
}

impl Asynt for Instruction {
    fn name(&self) -> &'static str {
        use Instruction::*;

        match self {
            Affectation(..) => "instr_affect",
            CallFunction(..) => "instr_appel",
            Return(..) => "instr_retour",
            If(..) => "instr_si",
            While(..) => "instr_tantque",
            WriteFunction(..) => "instr_ecrire",
            NOP => unreachable!(),
        }
    }

    fn hide(&self) -> bool {
        use Instruction::*;

        match self {
            NOP => true,
            _ => false,
        }
    }

    fn content(&self, f: &mut dyn Write, indent: usize) -> Result<()> {
        use Instruction::*;

        match self {
            Affectation(lv, e) => {
                lv.to_asynt(f, indent)?;
                e.to_asynt(f, indent)
            }
            CallFunction(e) => e.to_asynt(f, indent),
            Return(e) => e.to_asynt(f, indent),
            If(e, i1, i2) => {
                e.to_asynt(f, indent)?;
                i1.to_asynt(f, indent)?;
                i2.to_asynt(f, indent)
            }
            While(e, i) => {
                e.to_asynt(f, indent)?;
                i.to_asynt(f, indent)
            }
            WriteFunction(e) => e.to_asynt(f, indent),
            NOP => unreachable!(),
        }
    }
}

impl Asynt for LeftValue {
    fn name(&self) -> &'static str {
        use LeftValue::*;

        match self {
            Variable(..) => "var_simple",
            VariableAt(..) => "var_indicee",
        }
    }

    fn one_line(&self) -> bool {
        use LeftValue::*;

        match self {
            Variable(..) => true,
            VariableAt(..) => false,
        }
    }

    fn content(&self, f: &mut dyn Write, indent: usize) -> Result<()> {
        use LeftValue::*;

        match self {
            Variable(id) => write!(f, "{}", id),
            VariableAt(id, e) => {
                let spaces = " ".repeat(indent);

                writeln!(f, "{}<var_base_tableau>{}</var_base_tableau>", spaces, id)?;
                e.to_asynt(f, indent)
            }
        }
    }
}

impl Asynt for CallFunction {
    fn name(&self) -> &'static str {
        "appel"
    }

    fn content(&self, f: &mut dyn Write, indent: usize) -> Result<()> {
        let spaces = " ".repeat(indent);
        let (id, args) = (&self.0, &self.1);

        writeln!(f, "{}{}", spaces, id)?;
        args.to_asynt(f, indent)
    }
}

impl Asynt for [Expression] {
    fn name(&self) -> &'static str {
        "l_exp"
    }

    fn content(&self, f: &mut dyn Write, indent: usize) -> Result<()> {
        match self.len() {
            0 => Ok(()),
            _ => {
                self[0].to_asynt(f, indent)?;
                self[1..].to_asynt(f, indent)
            }
        }
    }
}

impl Asynt for Expression {
    fn name(&self) -> &'static str {
        use Expression::*;

        match self {
            Value(..) => "intExp",
            LeftValue(..) => "varExp",
            CallFunction(..) => "appelExp",
            ReadFunction => "lireExp",
            UnaryOperation(..) => "opExp",
            BinaryOperation(..) => "opExp",
        }
    }

    fn one_line(&self) -> bool {
        use Expression::*;

        match self {
            Value(..) => true,
            _ => false,
        }
    }

    fn content(&self, f: &mut dyn Write, indent: usize) -> Result<()> {
        use Expression::*;

        match self {
            Value(v) => write!(f, "{}", v),
            LeftValue(lv) => lv.to_asynt(f, indent),
            CallFunction(cf) => cf.to_asynt(f, indent),
            ReadFunction => Ok(()),
            UnaryOperation(o, e) => {
                o.to_asynt(f, indent)?;
                e.to_asynt(f, indent)
            }
            BinaryOperation(o, e1, e2) => {
                o.to_asynt(f, indent)?;
                e1.to_asynt(f, indent)?;
                e2.to_asynt(f, indent)
            }
        }
    }
}

impl Asynt for UnaryOperator {
    fn with_tag(&self) -> bool {
        false
    }

    fn content(&self, f: &mut dyn Write, indent: usize) -> Result<()> {
        use UnaryOperator::*;

        let spaces = " ".repeat(indent);

        let op = match self {
            Not => "non",
        };

        writeln!(f, "{}{}", spaces, op)
    }
}

impl Asynt for BinaryOperator {
    fn with_tag(&self) -> bool {
        false
    }

    fn content(&self, f: &mut dyn Write, indent: usize) -> Result<()> {
        use BinaryOperator::*;

        let spaces = " ".repeat(indent);

        let op = match self {
            Addidion => "plus",
            Subtraction => "moins",
            Multiplication => "fois",
            Division => "divise",
            And => "et",
            Or => "ou",
            Equal => "egal",
            LessThan => "inf",
        };

        writeln!(f, "{}{}", spaces, op)
    }
}
