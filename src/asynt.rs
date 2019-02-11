use crate::ast::*;

pub trait Asynt {
    fn to_asynt(&self, indent: usize) -> String {
        if self.hide() {
            return String::new();
        }

        if !self.with_tag() {
            return self.content(indent);
        }

        let spaces = " ".repeat(indent);
        let name = self.name();

        if self.one_line() {
            format!(
                "{s}<{n}>{c}</{n}>\n",
                s = spaces,
                n = name,
                c = self.content(indent + 2)
            )
        } else {
            format!(
                "{s}<{n}>\n{c}{s}</{n}>\n",
                s = spaces,
                n = name,
                c = self.content(indent + 2)
            )
        }
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
    fn content(&self, indent: usize) -> String;
}

impl Asynt for Program {
    fn name(&self) -> &'static str {
        "prog"
    }
    fn content(&self, indent: usize) -> String {
        use Statement::*;

        let var: Vec<Statement> = self
            .0
            .iter()
            .filter(|s| if let DclVariable(..) = s { true } else { false })
            .cloned()
            .collect();

        let func: Vec<Statement> = self
            .0
            .iter()
            .filter(|s| if let DclFunction(..) = s { true } else { false })
            .cloned()
            .collect();

        format!("{}{}", var.to_asynt(indent), func.to_asynt(indent))
    }
}

impl Asynt for [Statement] {
    fn name(&self) -> &'static str {
        "l_dec"
    }

    fn hide(&self) -> bool {
        self.is_empty()
    }

    fn content(&self, indent: usize) -> String {
        match self.len() {
            0 => String::new(),
            1 => self[0].to_asynt(indent),
            _ => format!("{}{}", self[0].to_asynt(indent), self[1..].to_asynt(indent)),
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

    fn content(&self, indent: usize) -> String {
        use Statement::*;

        match self {
            DclVariable(v) => v.to_asynt(indent),
            DclFunction(_, p, v, i) => format!(
                "{}{}{}",
                p.to_asynt(indent),
                v.to_asynt(indent),
                i.to_asynt(indent)
            ),
        }
    }
}

impl Asynt for [Scalar] {
    fn name(&self) -> &'static str {
        "l_dec"
    }

    fn hide(&self) -> bool {
        self.is_empty()
    }

    fn content(&self, indent: usize) -> String {
        match self.len() {
            0 => String::new(),
            1 => self[0].to_asynt(indent),
            _ => format!("{}{}", self[0].to_asynt(indent), self[1..].to_asynt(indent)),
        }
    }
}

impl Asynt for Variable {
    fn with_tag(&self) -> bool {
        false
    }

    fn content(&self, indent: usize) -> String {
        use Variable::*;

        match self {
            Scalar(s) => s.to_asynt(indent),
            Vector(v) => v.to_asynt(indent),
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

    fn content(&self, _indent: usize) -> String {
        self.1.clone()
    }
}

impl Asynt for Vector {
    fn name(&self) -> &'static str {
        "tabDec"
    }

    fn one_line(&self) -> bool {
        true
    }

    fn content(&self, _indent: usize) -> String {
        format!("{}[{}]", self.2, self.1)
    }
}

impl Asynt for [Instruction] {
    fn name(&self) -> &'static str {
        "l_instr"
    }

    fn hide(&self) -> bool {
        self.is_empty()
    }

    fn content(&self, indent: usize) -> String {
        match self.len() {
            0 => String::new(),
            1 => self[0].to_asynt(indent),
            _ => format!("{}{}", self[0].to_asynt(indent), self[1..].to_asynt(indent)),
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
            NOP => "INSTRUCTION",
        }
    }

    fn content(&self, indent: usize) -> String {
        use Instruction::*;

        match self {
            Affectation(lv, e) => format!("{}{}", lv.to_asynt(indent), e.to_asynt(indent)),
            CallFunction(e) => e.to_asynt(indent),
            Return(e) => e.to_asynt(indent),
            If(e, i1, i2) => format!("{}{}{}", e.to_asynt(indent), i1.to_asynt(indent), i2.to_asynt(indent)),
            While(e, i) => format!("{}{}", e.to_asynt(indent), i.to_asynt(indent)),
            WriteFunction(e) => e.to_asynt(indent),
            _ => {
                let spaces = " ".repeat(indent);
                format!("{}{:#?}\n", spaces, self)
            }
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

    fn content(&self, _indent: usize) -> String {
        use LeftValue::*;

        match self {
            Variable(id) => id.clone(),
            VariableAt(..) => format!("{:?}", self),
        }
    }
}

impl Asynt for CallFunction {
    fn name(&self) -> &'static str {
        "appel"
    }

    fn content(&self, indent: usize) -> String {
        let spaces = " ".repeat(indent);
        let (id, args) = self;

        format!("{}{}\n{}", spaces, id, args.to_asynt(indent))
    }
}

impl Asynt for [Expression] {
    fn name(&self) -> &'static str {
        "l_exp"
    }

    fn hide(&self) -> bool {
        self.is_empty()
    }

    fn content(&self, indent: usize) -> String {
        match self.len() {
            0 => String::new(),
            1 => self[0].to_asynt(indent),
            _ => format!("{}{}", self[0].to_asynt(indent), self[1..].to_asynt(indent)),
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
            _ => "EXPRESSION",
        }
    }

    fn one_line(&self) -> bool {
        use Expression::*;

        match self {
            Value(..) => true,
            _ => false,
        }
    }

    fn with_tag(&self) -> bool {
        use Expression::*;

        match self {
            _ => true,
        }
    }

    fn content(&self, indent: usize) -> String {
        use Expression::*;

        match self {
            Value(v) => format!("{}", v),
            LeftValue(lv) => lv.to_asynt(indent),
            CallFunction(cf) => cf.to_asynt(indent),
            ReadFunction => String::new(),
            UnaryOperation(o, e) => format!("{}{}", o.to_asynt(indent), e.to_asynt(indent)),
            BinaryOperation(o, e1, e2) => format!("{}{}{}", o.to_asynt(indent), e1.to_asynt(indent), e2.to_asynt(indent)),
            _ => {
                let spaces = " ".repeat(indent);
                format!("{}{:#?}\n", spaces, self)
            }
        }
    }
}

impl Asynt for UnaryOperator {
    fn with_tag(&self) -> bool {
        false
    }

    fn content(&self, indent: usize) -> String {
        use UnaryOperator::*;

        let spaces = " ".repeat(indent);

        let op = match self {
            Not => "non",
        };

        format!("{}{}\n", spaces, op)
    }
}

impl Asynt for BinaryOperator {
    fn with_tag(&self) -> bool {
        false
    }

    fn content(&self, indent: usize) -> String {
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

        format!("{}{}\n", spaces, op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use std::fs::read_to_string;

    #[test]
    fn simple_test() {
        let instrs = vec![Instruction::NOP, Instruction::NOP, Instruction::NOP];
        print!("{}", instrs.to_asynt(0));
        assert!(false);
    }

    #[test]
    fn affect() {
        test("affect");
    }

    #[test]
    fn boucle() {
        test("boucle");
    }

    #[test]
    fn expression() {
        test("expression");
    }

    #[test]
    fn max() {
        test("max");
    }

    #[test]
    fn tri() {
        test("tri");
    }

    fn test(filename: &str) {
        let l_file = read_to_string(format!("tests/resources/{}.l", filename)).unwrap();
        let asynt_file = read_to_string(format!("tests/resources/{}.asynt", filename)).unwrap();

        let generated_asynt = Parser::new()
            .parse(Lexer::new(&l_file))
            .unwrap()
            .to_asynt(0);

        print!("{}", generated_asynt);

        assert_eq!(asynt_file, generated_asynt);
    }
}
