use crate::ast;
use crate::three_address_code::*;

pub trait Gen<T> {
    fn gen(&self, d: &mut Data) -> T;
}

pub struct Data {
    label_count: u32,
    temp_count: u32,
    instructions: Vec<Instruction>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            label_count: 0,
            temp_count: 0,
            instructions: Vec::new(),
        }
    }

    fn new_label(&mut self) -> Label {
        let l = Label::new(format!("e{}", self.label_count));
        self.label_count += 1;
        l
    }

    fn new_temp(&mut self) -> Temp {
        let t = Temp::new(self.temp_count);
        self.temp_count += 1;
        t
    }

    fn add_instr(&mut self, instr: Instruction) {
        use InstructionKind::*;

        let last_use = self.instructions.len() as i32;

        let ct_set_last_use = |ct: &CT| {
            if let CT::T(t) = ct {
                t.set_last_use(last_use);
            }
        };
        let v_set_last_use = |v: &Variable| {
            if let Some(indice) = v.indice() {
                ct_set_last_use(indice);
            }
        };
        let tv_set_last_use = |tv: &TV, is_right: bool| match tv {
            TV::T(t) => {
                if is_right {
                    t.set_last_use(last_use)
                }
            }
            TV::V(v) => v_set_last_use(v),
        };
        let ctv_set_last_use = |ctv: &CTV, is_right: bool| match ctv {
            CTV::T(t) => {
                if is_right {
                    t.set_last_use(last_use)
                }
            }
            CTV::V(v) => v_set_last_use(v),
            _ => {}
        };

        match &instr.kind {
            Arithmetic {
                operator: _,
                left,
                right,
                result,
            } => {
                ctv_set_last_use(left, true);
                ctv_set_last_use(right, true);
                tv_set_last_use(result, false);
            }
            Affectation { value, result } => {
                ctv_set_last_use(value, true);
                tv_set_last_use(result, false);
            }
            Allocation { .. } => {}
            ReadFunction { result } => {
                tv_set_last_use(result, false);
            }
            WriteFunction { value } => {
                ctv_set_last_use(value, true);
            }
            FunctionCall {
                function: _,
                result,
            } => {
                tv_set_last_use(result, false);
            }
            FunctionBegin => {}
            FunctionEnd => {}
            FunctionPushArg { arg } => {
                ctv_set_last_use(arg, true);
            }
            FunctionReturn { value } => {
                ctv_set_last_use(value, true);
            }
            Jump { .. } => {}
            JumpIf {
                condition: _,
                left,
                right,
                label: _,
            } => {
                ctv_set_last_use(left, true);
                ctv_set_last_use(right, true);
            }
            NOP => {}
        }

        self.instructions.push(instr);
    }
}

impl Into<ThreeAddressCode> for Data {
    fn into(self) -> ThreeAddressCode {
        ThreeAddressCode {
            instructions: self.instructions,
        }
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
            DclFunction(id, args, vars, instructions) => {
                d.add_instr(Instruction {
                    label: Some(Label::new(format!("f{}", id))),
                    kind: InstructionKind::FunctionBegin,
                    comment: Some(format!("début fonction {}", id)),
                });

                vars.gen(d);
                instructions.gen(d);

                d.add_instr(Instruction {
                    label: None,
                    kind: InstructionKind::FunctionEnd,
                    comment: Some(format!("fin fonction {}", id)),
                });
            }
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

        d.add_instr(Instruction {
            label: None,
            kind: InstructionKind::Allocation {
                variable: Some(Variable::new(format!("v{}", id), None)),
                size: Constant::new(t.size() as i32),
            },
            comment: None,
        });
    }
}

impl Gen<()> for ast::Vector {
    fn gen(&self, d: &mut Data) -> () {
        let (t, size, id) = self;

        d.add_instr(Instruction {
            label: None,
            kind: InstructionKind::Allocation {
                variable: Some(Variable::new(format!("v{}", id), None)),
                size: Constant::new(t.size() as i32 * (*size) as i32),
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
                d.add_instr(Instruction {
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
                d.add_instr(Instruction {
                    label: None,
                    kind: InstructionKind::FunctionReturn { value },
                    comment: Some(format!("retourne {}", e)),
                });
                d.add_instr(Instruction {
                    label: None,
                    kind: InstructionKind::FunctionEnd,
                    comment: None,
                });
            }
            If(e, i1, i2) => {
                let l_else = d.new_label();
                let l_end = d.new_label();

                let left = e.gen(d);
                d.add_instr(Instruction {
                    label: None,
                    kind: InstructionKind::JumpIf {
                        condition: JumpIfCondition::Equal,
                        left,
                        right: Constant::new(false).into(),
                        label: l_else.clone(),
                    },
                    comment: Some(format!("si {}", e)),
                });
                i1.gen(d);
                d.add_instr(Instruction {
                    label: None,
                    kind: InstructionKind::Jump {
                        label: l_end.clone(),
                    },
                    comment: None,
                });
                d.add_instr(Instruction {
                    label: Some(l_else),
                    kind: InstructionKind::NOP,
                    comment: Some("sinon".to_owned()),
                });
                i2.gen(d);
                d.add_instr(Instruction {
                    label: Some(l_end),
                    kind: InstructionKind::NOP,
                    comment: Some("fin si".to_owned()),
                });
            }
            While(e, i) => {
                let l_begin = d.new_label();
                let l_end = d.new_label();

                d.add_instr(Instruction {
                    label: Some(l_begin.clone()),
                    kind: InstructionKind::NOP,
                    comment: Some(format!("tantque {}", e)),
                });
                let left = e.gen(d);
                d.add_instr(Instruction {
                    label: None,
                    kind: InstructionKind::JumpIf {
                        condition: JumpIfCondition::Equal,
                        left,
                        right: Constant::new(false).into(),
                        label: l_end.clone(),
                    },
                    comment: Some("sort tantque".to_owned()),
                });
                i.gen(d);
                d.add_instr(Instruction {
                    label: None,
                    kind: InstructionKind::Jump { label: l_begin },
                    comment: None,
                });
                d.add_instr(Instruction {
                    label: Some(l_end),
                    kind: InstructionKind::NOP,
                    comment: Some("fin tantque".to_owned()),
                });
            }
            WriteFunction(e) => {
                let value = e.gen(d);
                d.add_instr(Instruction {
                    label: None,
                    kind: InstructionKind::WriteFunction { value },
                    comment: None,
                });
            }
            NOP => {
                d.add_instr(Instruction {
                    label: None,
                    kind: InstructionKind::NOP,
                    comment: None,
                });
            }
        }
    }
}

impl Gen<CTV> for ast::Expression {
    fn gen(&self, d: &mut Data) -> CTV {
        use ast::Expression::*;

        match self {
            Value(v) => Constant::new(*v).into(),
            LeftValue(lv) => lv.gen(d).into(),
            CallFunction(c) => c.gen(d),
            ReadFunction => {
                let result = d.new_temp();
                d.add_instr(Instruction {
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

                match op {
                    Not => {
                        let l_end = d.new_label();

                        let left = e.gen(d);
                        let result = d.new_temp();
                        d.add_instr(Instruction {
                            label: None,
                            kind: InstructionKind::Affectation {
                                value: Constant::new(false).into(),
                                result: result.clone().into(),
                            },
                            comment: Some(format!("début {}", self)),
                        });
                        d.add_instr(Instruction {
                            label: None,
                            kind: InstructionKind::JumpIf {
                                condition: JumpIfCondition::Equal,
                                left,
                                right: Constant::new(false).into(),
                                label: l_end.clone(),
                            },
                            comment: None,
                        });
                        d.add_instr(Instruction {
                            label: None,
                            kind: InstructionKind::Affectation {
                                value: Constant::new(true).into(),
                                result: result.clone().into(),
                            },
                            comment: None,
                        });
                        d.add_instr(Instruction {
                            label: Some(l_end),
                            kind: InstructionKind::NOP,
                            comment: Some(format!("fin {}", self)),
                        });

                        result.into()
                    }
                }
            }
            BinaryOperation(op, e1, e2) => {
                use ast::BinaryOperator::*;

                match op {
                    Addidion | Subtraction | Multiplication | Division => {
                        let operator = match op {
                            Addidion => ArithmeticOperator::Addition,
                            Subtraction => ArithmeticOperator::Subtraction,
                            Multiplication => ArithmeticOperator::Multiplication,
                            Division => ArithmeticOperator::Division,
                            _ => unreachable!(),
                        };

                        let left = e1.gen(d);
                        let right = e2.gen(d);
                        let result = d.new_temp();
                        d.add_instr(Instruction {
                            label: None,
                            kind: InstructionKind::Arithmetic {
                                operator,
                                left,
                                right,
                                result: result.clone().into(),
                            },
                            comment: Some(format!("{}", self)),
                        });

                        result.into()
                    }
                    And => {
                        let l_end = d.new_label();

                        let left = e1.gen(d);
                        let result = d.new_temp();
                        d.add_instr(Instruction {
                            label: None,
                            kind: InstructionKind::Affectation {
                                value: Constant::new(false).into(),
                                result: result.clone().into(),
                            },
                            comment: Some(format!("début {}", self)),
                        });
                        d.add_instr(Instruction {
                            label: None,
                            kind: InstructionKind::JumpIf {
                                condition: JumpIfCondition::Equal,
                                left,
                                right: Constant::new(false).into(),
                                label: l_end.clone(),
                            },
                            comment: None,
                        });
                        let left = e2.gen(d);
                        d.add_instr(Instruction {
                            label: None,
                            kind: InstructionKind::JumpIf {
                                condition: JumpIfCondition::Equal,
                                left,
                                right: Constant::new(false).into(),
                                label: l_end.clone(),
                            },
                            comment: None,
                        });
                        d.add_instr(Instruction {
                            label: None,
                            kind: InstructionKind::Affectation {
                                value: Constant::new(true).into(),
                                result: result.clone().into(),
                            },
                            comment: None,
                        });
                        d.add_instr(Instruction {
                            label: Some(l_end),
                            kind: InstructionKind::NOP,
                            comment: Some(format!("fin {}", self)),
                        });

                        result.into()
                    }
                    Or => {
                        let l_end = d.new_label();

                        let left = e1.gen(d);
                        let result = d.new_temp();
                        d.add_instr(Instruction {
                            label: None,
                            kind: InstructionKind::Affectation {
                                value: Constant::new(true).into(),
                                result: result.clone().into(),
                            },
                            comment: Some(format!("début {}", self)),
                        });
                        d.add_instr(Instruction {
                            label: None,
                            kind: InstructionKind::JumpIf {
                                condition: JumpIfCondition::Equal,
                                left,
                                right: Constant::new(true).into(),
                                label: l_end.clone(),
                            },
                            comment: None,
                        });
                        let left = e2.gen(d);
                        d.add_instr(Instruction {
                            label: None,
                            kind: InstructionKind::JumpIf {
                                condition: JumpIfCondition::Equal,
                                left,
                                right: Constant::new(true).into(),
                                label: l_end.clone(),
                            },
                            comment: None,
                        });
                        d.add_instr(Instruction {
                            label: None,
                            kind: InstructionKind::Affectation {
                                value: Constant::new(false).into(),
                                result: result.clone().into(),
                            },
                            comment: None,
                        });
                        d.add_instr(Instruction {
                            label: Some(l_end),
                            kind: InstructionKind::NOP,
                            comment: Some(format!("fin {}", self)),
                        });

                        result.into()
                    }
                    Equal | LessThan => {
                        let condition = match op {
                            Equal => JumpIfCondition::Equal,
                            LessThan => JumpIfCondition::Less,
                            _ => unreachable!(),
                        };
                        let l_end = d.new_label();

                        let left = e1.gen(d);
                        let right = e2.gen(d);
                        let result = d.new_temp();
                        d.add_instr(Instruction {
                            label: None,
                            kind: InstructionKind::Affectation {
                                value: Constant::new(true).into(),
                                result: result.clone().into(),
                            },
                            comment: Some(format!("début {}", self)),
                        });
                        d.add_instr(Instruction {
                            label: None,
                            kind: InstructionKind::JumpIf {
                                condition,
                                left,
                                right,
                                label: l_end.clone(),
                            },
                            comment: None,
                        });
                        d.add_instr(Instruction {
                            label: None,
                            kind: InstructionKind::Affectation {
                                value: Constant::new(false).into(),
                                result: result.clone().into(),
                            },
                            comment: None,
                        });
                        d.add_instr(Instruction {
                            label: Some(l_end),
                            kind: InstructionKind::NOP,
                            comment: Some(format!("fin {}", self)),
                        });

                        result.into()
                    }
                }
            }
        }
    }
}

impl Gen<Variable> for ast::LeftValue {
    fn gen(&self, d: &mut Data) -> Variable {
        match self {
            ast::LeftValue::Variable(id) => Variable::new(format!("v{}", id), None),
            ast::LeftValue::VariableAt(id, indice) => {
                let indice = indice.gen(d);

                let indice = match indice {
                    CTV::C(c) => CT::C(c),
                    CTV::T(t) => CT::T(t),
                    CTV::V(v) => {
                        let result = d.new_temp();
                        d.add_instr(Instruction {
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

                Variable::new(format!("v{}", id), Some(indice))
            }
        }
    }
}

impl Gen<CTV> for ast::CallFunction {
    fn gen(&self, d: &mut Data) -> CTV {
        let (id, arguments) = (&self.0, &self.1);

        d.add_instr(Instruction {
            label: None,
            kind: InstructionKind::Allocation {
                variable: None,
                size: Constant::new(1),
            },
            comment: Some(format!("début appel {}", id)),
        });

        for arg in arguments {
            let arg = arg.gen(d);
            d.add_instr(Instruction {
                label: None,
                kind: InstructionKind::FunctionPushArg { arg },
                comment: None,
            });
        }

        let result = d.new_temp();

        d.add_instr(Instruction {
            label: None,
            kind: InstructionKind::FunctionCall {
                function: Label::new(format!("f{}", id)),
                result: result.clone().into(),
            },
            comment: Some(format!("fin appel {}", id)),
        });

        result.into()
    }
}
