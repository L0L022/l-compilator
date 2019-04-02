#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use crate::symbol_table::Scope;
use crate::symbol_table::SymbolTable;
use crate::three_address_code::*;
use std::cell::RefCell;
use std::cmp::max;
use std::collections::HashMap;
use std::ffi::c_void;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

thread_local!(static symbol_table_info: RefCell<Option<(*const SymbolTable, usize)>> = RefCell::new(None));
thread_local!(static allocs: RefCell<Vec<*mut c_void>> = RefCell::new(Vec::new()));
thread_local!(static labels: RefCell<HashMap<Label, *mut operande>> = RefCell::new(HashMap::new()));
thread_local!(static constants: RefCell<HashMap<Constant, *mut operande>> = RefCell::new(HashMap::new()));
thread_local!(static temps: RefCell<HashMap<Temp, *mut operande>> = RefCell::new(HashMap::new()));
thread_local!(static variables: RefCell<HashMap<Variable, *mut operande>> = RefCell::new(HashMap::new()));

pub fn print_nasm(
    three_address_code: &ThreeAddressCode,
    symbol_table: &SymbolTable,
    current_table: usize,
) {
    unsafe { assert!(code3a.liste.is_null()) }

    symbol_table_info.with(|sti| {
        *sti.borrow_mut() = Some((symbol_table, current_table));
    });

    let mut instructions = Vec::with_capacity(three_address_code.instructions.len());

    for instr in &three_address_code.instructions {
        use InstructionKind::*;

        let (op_code, op_oper1, op_oper2, op_result): (
            u32,
            Option<*mut operande>,
            Option<*mut operande>,
            Option<*mut operande>,
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
            NOP => (instrcode_nop, None, None, None),
        };

        let op_etiq = match &instr.label {
            Some(label) => CString::new(&label.label()[..])
                .unwrap_or_default()
                .into_raw(),
            None => ptr::null_mut(),
        };

        let op_oper1 = match op_oper1 {
            Some(o) => o,
            None => ptr::null_mut(),
        };

        let op_oper2 = match op_oper2 {
            Some(o) => o,
            None => ptr::null_mut(),
        };

        let op_result = match op_result {
            Some(o) => o,
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
        code3a.liste = instructions.as_mut_ptr();
        code3a.next = instructions.len() as i32;

        c3a2nasm_generer();

        code3a.liste = ptr::null_mut();
        code3a.next = 0;
    }

    for mut instr in instructions {
        unsafe {
            if !instr.op_etiq.is_null() {
                CString::from_raw(instr.op_etiq);
                instr.op_etiq = ptr::null_mut();
            }

            if !instr.comment.is_null() {
                CString::from_raw(instr.comment);
                instr.comment = ptr::null_mut();
            }
        }
    }

    symbol_table_info.with(|sti| {
        *sti.borrow_mut() = None;
    });

    allocs.with(|als| {
        for alloc in als.borrow().iter() {
            unsafe {
                libc::free(*alloc);
            }
        }
        als.borrow_mut().clear();
    });

    labels.with(|ls| {
        for op in ls.borrow_mut().values_mut() {
            unsafe {
                drop_operande(*op);
            }
        }
        ls.borrow_mut().clear();
    });

    constants.with(|cs| {
        for op in cs.borrow_mut().values_mut() {
            unsafe {
                drop_operande(*op);
            }
        }
        cs.borrow_mut().clear();
    });

    temps.with(|ts| {
        for op in ts.borrow_mut().values_mut() {
            unsafe {
                drop_operande(*op);
            }
        }
        ts.borrow_mut().clear();
    });

    variables.with(|vars| {
        for op in vars.borrow_mut().values_mut() {
            unsafe {
                drop_operande(*op);
            }
        }
        vars.borrow_mut().clear();
    });
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
                        Some(indice) => indice.into(),
                        None => ptr::null_mut(),
                    },
                },
            },
        }
    }
}

impl From<&Variable> for *mut operande {
    fn from(v: &Variable) -> Self {
        variables.with(|vars| {
            *vars
                .borrow_mut()
                .entry(v.clone())
                .or_insert_with(|| Box::into_raw(Box::new(v.into())))
        })
    }
}

unsafe fn drop_operande(op: *mut operande) {
    if op.is_null() {
        return;
    }

    drop_operande_ref_mut(&mut *op);

    Box::from_raw(op);
}

unsafe fn drop_operande_ref_mut(op: &mut operande) {
    if op.oper_type == O_VARIABLE as i32 {
        let nom = op.u.oper_var.oper_nom;
        if !nom.is_null() {
            CString::from_raw(nom);
            op.u.oper_var.oper_nom = ptr::null_mut();
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

impl From<&Constant> for *mut operande {
    fn from(c: &Constant) -> Self {
        constants.with(|cs| {
            *cs.borrow_mut()
                .entry(c.clone())
                .or_insert_with(|| Box::into_raw(Box::new(c.into())))
        })
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

impl From<&Temp> for *mut operande {
    fn from(t: &Temp) -> Self {
        temps.with(|ts| {
            *ts.borrow_mut()
                .entry(t.clone())
                .or_insert_with(|| Box::into_raw(Box::new(t.into())))
        })
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

impl From<&Label> for *mut operande {
    fn from(l: &Label) -> Self {
        labels.with(|ls| {
            *ls.borrow_mut()
                .entry(l.clone())
                .or_insert_with(|| Box::into_raw(Box::new(l.into())))
        })
    }
}

impl From<&CTV> for *mut operande {
    fn from(ctv: &CTV) -> Self {
        match ctv {
            CTV::C(c) => c.into(),
            CTV::T(t) => t.into(),
            CTV::V(v) => v.into(),
        }
    }
}

impl From<&TV> for *mut operande {
    fn from(tv: &TV) -> Self {
        match tv {
            TV::T(t) => t.into(),
            TV::V(v) => v.into(),
        }
    }
}

impl From<&CT> for *mut operande {
    fn from(ct: &CT) -> Self {
        match ct {
            CT::C(c) => c.into(),
            CT::T(t) => t.into(),
        }
    }
}

#[no_mangle]
extern "C" fn rust_malloc(size: libc::size_t) -> *mut c_void {
    allocs.with(|als| {
        let alloc = unsafe { libc::malloc(size) };
        als.borrow_mut().push(alloc);
        alloc
    })
}

#[no_mangle]
extern "C" fn rust_new_temporaire() -> *mut operande {
    (&Temp::new(unsafe { global_temp_counter } as u32)).into()
}

#[no_mangle]
extern "C" fn rust_function_enter(id: *mut std::os::raw::c_char) {
    symbol_table_info.with(|sti| {
        if let Some(info) = sti.borrow_mut().as_mut() {
            let (symbol_table, current_table) = info;
            let symbol_table = unsafe { symbol_table.as_ref().unwrap() };
            let id = unsafe { CStr::from_ptr(id) };

            use crate::symbol_table::SymbolKind;

            let symbol = symbol_table
                .iter(*current_table)
                .find(|symbol| symbol.id.as_bytes() == id.to_bytes() && symbol.is_function());

            match symbol {
                Some(symbol) => match symbol.kind {
                    SymbolKind::Function { symbol_table, .. } => {
                        *current_table = symbol_table;
                    }
                    _ => unreachable!(),
                },
                None => unreachable!(),
            }
        }
    });
}

#[no_mangle]
extern "C" fn rust_function_exit() {
    symbol_table_info.with(|sti| {
        if let Some(info) = sti.borrow_mut().as_mut() {
            let (symbol_table, current_table) = info;
            let symbol_table = unsafe { symbol_table.as_ref().unwrap() };

            if let Some(parent) = symbol_table.tables[*current_table].parent {
                *current_table = parent;
            }
        }
    });
}

#[no_mangle]
extern "C" fn rust_function_nb_arguments(id: *mut std::os::raw::c_char) -> usize {
    symbol_table_info.with(|sti| match sti.borrow_mut().as_mut() {
        Some(info) => {
            let (symbol_table, current_table) = info;
            let symbol_table = unsafe { symbol_table.as_ref().unwrap() };
            let id = unsafe { CStr::from_ptr(id) };

            use crate::symbol_table::SymbolKind;

            let symbol = symbol_table
                .iter(*current_table)
                .find(|symbol| symbol.id.as_bytes() == id.to_bytes() && symbol.is_function());

            match symbol {
                Some(symbol) => match symbol.kind {
                    SymbolKind::Function { nb_arguments, .. } => nb_arguments,
                    _ => unreachable!(),
                },
                None => unreachable!(),
            }
        }
        None => unreachable!(),
    })
}
