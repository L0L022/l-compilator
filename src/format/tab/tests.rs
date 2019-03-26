use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::semantic_analyser::Analyse;
use std::fs::{read, read_to_string};
use std::path::Path;

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

#[test]
fn vide() {
    test("vide");
}

#[test]
fn already_declared_argument_scalar_declare_local_scalar_err() {
    test("already-declared-argument-scalar-declare-local-scalar-err");
}

#[test]
fn already_declared_global_scalar_declare_argument_scalar() {
    test("already-declared-global-scalar-declare-argument-scalar");
}

#[test]
fn already_declared_global_scalar_declare_local_scalar() {
    test("already-declared-global-scalar-declare-local-scalar");
}

#[test]
fn already_declared_global_scalar_err() {
    test("already-declared-global-scalar-err");
}

#[test]
fn already_declared_global_vector_declare_argument_scalar() {
    test("already-declared-global-vector-declare-argument-scalar");
}

#[test]
fn already_declared_global_vector_declare_local_scalar() {
    test("already-declared-global-vector-declare-local-scalar");
}

#[test]
fn already_declared_global_vector_err() {
    test("already-declared-global-vector-err");
}

#[test]
fn already_declared_local_scalar_err() {
    test("already-declared-local-scalar-err");
}

#[test]
fn vector_without_indice() {
    test("vector-without-indice");
}

#[test]
fn scalar_with_indice() {
    test("scalar-with-indice");
}

#[test]
fn variable_shadowing() {
    test("variable-shadowing");
}

#[test]
fn declared_argument_scalar() {
    test("declared-argument-scalar");
}

#[test]
fn declared_global_scalar() {
    test("declared-global-scalar");
}

#[test]
fn declared_global_vector() {
    test("declared-global-vector");
}

#[test]
fn declared_local_scalar() {
    test("declared-local-scalar");
}

fn test(filename: &str) {
    let l_file = read_to_string(format!("tests/resources/{}.l", filename)).unwrap();
    let tab_file = format!("tests/resources/{}.tab", filename);

    let analyse = Parser::new().parse(Lexer::new(&l_file)).unwrap().analyse();

    if Path::new(&tab_file).is_file() {
        let tab_file = read(tab_file).unwrap();
        let mut generated_tab = Vec::with_capacity(tab_file.capacity());

        analyse.unwrap().as_table(&mut generated_tab).unwrap();

        print!("{}", String::from_utf8_lossy(&generated_tab));

        assert!(tab_file == generated_tab);
    } else {
        assert!(analyse.is_err());
    }
}
