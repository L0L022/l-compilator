#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use crate::symbol_table::Scope;
use crate::three_address_code::*;
use std::cmp::max;
use std::ffi::CString;
use std::ptr;

pub fn print_nasm(three_address_code: &ThreeAddressCode) {
    unsafe { assert!(code3a.liste.is_null()) }

    let mut instructions = Vec::with_capacity(three_address_code.instructions.len());

    for instr in &three_address_code.instructions {
        use InstructionKind::*;

        let (op_code, op_oper1, op_oper2, op_result): (
            u32,
            Option<operande>,
            Option<operande>,
            Option<operande>,
        ) = match &instr.kind {
            Arithmetic {
                operator,
                left,
                right,
                result,
            } => {
                let operator = match operator {
                    ArithmeticOperator::Addition => instrcode_arith_add,
                    ArithmeticOperator::Subtraction => instrcode_arith_sub,
                    ArithmeticOperator::Multiplication => instrcode_arith_mult,
                    ArithmeticOperator::Division => instrcode_arith_div,
                };
                (
                    operator,
                    Some(left.into()),
                    Some(right.into()),
                    Some(result.into()),
                )
            }
            Affectation { value, result } => (
                instrcode_assign,
                Some(value.into()),
                None,
                Some(result.into()),
            ),
            Allocation { variable, size } => (
                instrcode_alloc,
                Some(size.into()),
                match variable {
                    Some(v) => Some(v.into()),
                    None => None,
                },
                None,
            ),
            ReadFunction { result } => (instrcode_sys_read, None, None, Some(result.into())),
            WriteFunction { value } => (instrcode_sys_write, Some(value.into()), None, None),
            FunctionCall { function, result } => (
                instrcode_func_call,
                Some(function.into()),
                None,
                Some(result.into()),
            ),
            FunctionBegin => (instrcode_func_begin, None, None, None),
            FunctionEnd => (instrcode_func_end, None, None, None),
            FunctionPushArg { arg } => (instrcode_func_param, Some(arg.into()), None, None),
            FunctionReturn { value } => (instrcode_func_val_ret, Some(value.into()), None, None),
            Jump { label } => (instrcode_jump, Some(label.into()), None, None),
            JumpIf {
                condition,
                left,
                right,
                label,
            } => {
                let condition = match condition {
                    JumpIfCondition::Less => instrcode_jump_if_less,
                    JumpIfCondition::LessOrEqual => instrcode_jump_if_less_or_equal,
                    JumpIfCondition::Equal => instrcode_jump_if_equal,
                    JumpIfCondition::NotEqual => instrcode_jump_if_not_equal,
                    JumpIfCondition::Greater => instrcode_jump_if_greater,
                    JumpIfCondition::GreaterOrEqual => instrcode_jump_if_greater_or_equal,
                };
                (
                    condition,
                    Some(left.into()),
                    Some(right.into()),
                    Some(label.into()),
                )
            }
            NOP => continue,
        };

        let op_etiq = match &instr.label {
            Some(label) => CString::new(&label.label()[..])
                .unwrap_or_default()
                .into_raw(),
            None => ptr::null_mut(),
        };

        let op_oper1 = match op_oper1 {
            Some(o) => Box::into_raw(Box::new(o)),
            None => ptr::null_mut(),
        };

        let op_oper2 = match op_oper2 {
            Some(o) => Box::into_raw(Box::new(o)),
            None => ptr::null_mut(),
        };

        let op_result = match op_result {
            Some(o) => Box::into_raw(Box::new(o)),
            None => ptr::null_mut(),
        };

        let comment = match &instr.comment {
            Some(comment) => CString::new(&comment[..]).unwrap_or_default().into_raw(),
            None => ptr::null_mut(),
        };

        instructions.push(operation_3a {
            op_etiq,
            op_code,
            op_oper1,
            op_oper2,
            op_result,
            comment,
        });
    }

    unsafe {
        code3a_init();

        code3a.liste = instructions.as_mut_ptr();
        code3a.next = instructions.len() as i32;

        c3a2nasm_generer();

        code3a.liste = ptr::null_mut();
        code3a.next = 0;
    }

    for mut instr in instructions {
        // utiliser drop operande
        unsafe {
            if !instr.op_etiq.is_null() {
                CString::from_raw(instr.op_etiq);
                instr.op_etiq = ptr::null_mut();
            }
            if !instr.op_oper1.is_null() {
                Box::from_raw(instr.op_oper1);
                instr.op_oper1 = ptr::null_mut();
            }
            if !instr.op_oper2.is_null() {
                Box::from_raw(instr.op_oper2);
                instr.op_oper2 = ptr::null_mut();
            }
            if !instr.comment.is_null() {
                CString::from_raw(instr.comment);
                instr.comment = ptr::null_mut();
            }
        }
    }
}

impl From<&Variable> for operande {
    fn from(v: &Variable) -> Self {
        Self {
            oper_type: O_VARIABLE as i32,
            u: operande___bindgen_ty_1 {
                oper_var: operande___bindgen_ty_1__bindgen_ty_2 {
                    oper_nom: CString::new(&v.id()[..]).unwrap_or_default().into_raw(),
                    oper_portee: match v.scope() {
                        Scope::Global => P_VARIABLE_GLOBALE,
                        Scope::Local => P_VARIABLE_LOCALE,
                        Scope::Argument => P_ARGUMENT,
                    } as i32,
                    oper_adresse: v.address() as i32,
                    oper_indice: match v.indice() {
                        Some(indice) => Box::into_raw(Box::new(indice.into())),
                        None => ptr::null_mut(),
                    },
                },
            },
        }
    }
}

unsafe fn drop_operande(op: &mut operande) {
    if op.oper_type == O_VARIABLE as i32 {
        let nom = op.u.oper_var.oper_nom;
        if !nom.is_null() {
            CString::from_raw(nom);
            op.u.oper_var.oper_nom = ptr::null_mut();
        }

        let indice = op.u.oper_var.oper_indice;
        if !indice.is_null() {
            Box::from_raw(indice);
            op.u.oper_var.oper_indice = ptr::null_mut();
        }
    }

    if op.oper_type == O_ETIQUETTE as i32 {
        let nom = op.u.oper_nom;
        if !nom.is_null() {
            CString::from_raw(nom);
            op.u.oper_nom = ptr::null_mut();
        }
    }
}

impl From<&Constant> for operande {
    fn from(c: &Constant) -> Self {
        Self {
            oper_type: O_CONSTANTE as i32,
            u: operande___bindgen_ty_1 {
                oper_valeur: c.constant(),
            },
        }
    }
}

impl From<&Temp> for operande {
    fn from(t: &Temp) -> Self {
        unsafe {
            global_temp_counter = max(global_temp_counter, t.temp() as i32 + 1);
        }
        Self {
            oper_type: O_TEMPORAIRE as i32,
            u: operande___bindgen_ty_1 {
                oper_temp: operande___bindgen_ty_1__bindgen_ty_1 {
                    oper_tempnum: t.temp() as i32,
                    last_use: t.last_use(),
                    emplacement: 0,
                },
            },
        }
    }
}

impl From<&Label> for operande {
    fn from(l: &Label) -> Self {
        Self {
            oper_type: O_ETIQUETTE as i32,
            u: operande___bindgen_ty_1 {
                oper_nom: CString::new(&l.label()[..]).unwrap_or_default().into_raw(),
            },
        }
    }
}

impl From<&CTV> for operande {
    fn from(ctv: &CTV) -> Self {
        match ctv {
            CTV::C(c) => c.into(),
            CTV::T(t) => t.into(),
            CTV::V(v) => v.into(),
        }
    }
}

impl From<&TV> for operande {
    fn from(tv: &TV) -> Self {
        match tv {
            TV::T(t) => t.into(),
            TV::V(v) => v.into(),
        }
    }
}

impl From<&CT> for operande {
    fn from(ct: &CT) -> Self {
        match ct {
            CT::C(c) => c.into(),
            CT::T(t) => t.into(),
        }
    }
}
