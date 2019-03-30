use crate::ast;
use crate::three_address_code::*;

trait Gen<T> {
    fn gen(&self, d: &mut Data) -> T;
}

pub struct Data {
    pub label_count: u32,
    pub temp_count: u32,
    pub next_label: Option<Label>,
    pub instructions: Vec<Instruction>,
}

impl Data {
    fn new_temp(&mut self) -> Temp {
        let t = Temp(self.label_count);
        self.label_count += 1;
        t
    }
}

impl<T: Gen<()>> Gen<()> for [T] {
    fn gen(&self, d: &mut Data) {
        for i in self {
            i.gen(d);
        }
    }
}

impl Gen<()> for ast::Program {
    fn gen(&self, d: &mut Data) -> () {
        self.0.gen(d);
    }
}

impl Gen<()> for ast::Statement {
    fn gen(&self, d: &mut Data) -> () {
        use ast::Statement::*;

        match self {
            DclVariable(v) => v.gen(d),
            DclFunction(id, args, vars, instructions) => {}
        }
    }
}

impl Gen<()> for ast::Variable {
    fn gen(&self, d: &mut Data) -> () {
        use ast::Variable::*;

        match self {
            Scalar(s) => s.gen(d),
            Vector(v) => v.gen(d),
        }
    }
}

impl Gen<()> for ast::Scalar {
    fn gen(&self, d: &mut Data) -> () {
        let (t, id) = self;

        d.instructions.push(Instruction {
            label: None,
            kind: InstructionKind::Allocation {
                variable: Some(Variable {
                    id: id.clone(),
                    indice: None,
                }),
                size: Constant(t.size() as i32),
            },
            comment: None,
        });
    }
}

impl Gen<()> for ast::Vector {
    fn gen(&self, d: &mut Data) -> () {
        let (t, size, id) = self;

        d.instructions.push(Instruction {
            label: None,
            kind: InstructionKind::Allocation {
                variable: Some(Variable {
                    id: id.clone(),
                    indice: None,
                }),
                size: Constant(t.size() as i32 * (*size) as i32),
            },
            comment: None,
        });
    }
}

impl Gen<()> for ast::Instruction {
    fn gen(&self, d: &mut Data) -> () {
        use ast::Instruction::*;

        match self {
            Affectation(lv, e) => {
                let result = lv.gen(d).into();
                let value = e.gen(d);
                d.instructions.push(Instruction {
                    label: None,
                    kind: InstructionKind::Affectation { value, result },
                    comment: None,
                });
            }
            CallFunction(c) => {
                c.gen(d);
            }
            Return(e) => {
                let value = e.gen(d);
                d.instructions.push(Instruction {
                    label: None,
                    kind: InstructionKind::FunctionReturn { value },
                    comment: None,
                });
                d.instructions.push(Instruction {
                    label: None,
                    kind: InstructionKind::FunctionEnd,
                    comment: None,
                });
            }
            If(e, i1, i2) => {
                e.gen(d);
                i1.gen(d);
                i2.gen(d);
            }
            While(e, i) => {
                e.gen(d);
                i.gen(d);
            }
            WriteFunction(e) => {
                let value = e.gen(d);
                d.instructions.push(Instruction {
                    label: None,
                    kind: InstructionKind::WriteFunction { value },
                    comment: None,
                });
            }
            NOP => {}
        }
    }
}

impl Gen<CTV> for ast::Expression {
    fn gen(&self, d: &mut Data) -> CTV {
        use ast::Expression::*;

        match self {
            Value(v) => Constant(*v).into(),
            LeftValue(lv) => lv.gen(d).into(),
            CallFunction(c) => c.gen(d),
            ReadFunction => {
                let result = d.new_temp();
                d.instructions.push(Instruction {
                    label: None,
                    kind: InstructionKind::ReadFunction {
                        result: result.clone().into(),
                    },
                    comment: None,
                });
                result.into()
            }
            UnaryOperation(op, e) => {
                use ast::UnaryOperator::*;
                let result = d.new_temp();

                match op {
                    Not => {
                        e.gen(d);
                    }
                }

                result.into()
            }
            BinaryOperation(op, e1, e2) => {
                use ast::BinaryOperator::*;

                let result = d.new_temp();

                if op.is_arithmetic() {
                    let operator = match op {
                        Addidion => ArithmeticOperator::Addition,
                        Subtraction => ArithmeticOperator::Subtraction,
                        Multiplication => ArithmeticOperator::Multiplication,
                        Division => ArithmeticOperator::Division,
                        _ => unreachable!(),
                    };
                    let left = e1.gen(d);
                    let right = e2.gen(d);
                    d.instructions.push(Instruction {
                        label: None,
                        kind: InstructionKind::Arithmetic {
                            operator,
                            left,
                            right,
                            result: result.clone().into(),
                        },
                        comment: None,
                    });
                }

                result.into()
            }
        }
    }
}

impl Gen<Variable> for ast::LeftValue {
    fn gen(&self, d: &mut Data) -> Variable {
        match self {
            ast::LeftValue::Variable(id) => Variable {
                id: id.clone(),
                indice: None,
            },
            ast::LeftValue::VariableAt(id, indice) => {
                let indice = indice.gen(d);

                let indice = match indice {
                    CTV::C(c) => CT::C(c),
                    CTV::T(t) => CT::T(t),
                    CTV::V(v) => {
                        let result = d.new_temp();
                        d.instructions.push(Instruction {
                            label: None,
                            kind: InstructionKind::Affectation {
                                value: v.into(),
                                result: result.clone().into(),
                            },
                            comment: None,
                        });
                        CT::T(result)
                    }
                };

                Variable {
                    id: id.clone(),
                    indice: Some(indice),
                }
            }
        }
    }
}

impl Gen<CTV> for ast::CallFunction {
    fn gen(&self, d: &mut Data) -> CTV {
        let (id, arguments) = self;

        d.instructions.push(Instruction {
            label: None,
            kind: InstructionKind::Allocation {
                variable: None,
                size: Constant(1),
            },
            comment: Some("alloue place pour la valeur de retour".to_string()),
        });

        for arg in arguments {
            let arg = arg.gen(d);
            d.instructions.push(Instruction {
                label: None,
                kind: InstructionKind::FunctionPushArg { arg },
                comment: None,
            });
        }

        let result = d.new_temp();

        d.instructions.push(Instruction {
            label: None,
            kind: InstructionKind::FunctionCall {
                function: Label(format!("f{}", id)),
                result: result.clone().into(),
            },
            comment: None,
        });

        result.into()
    }
}
