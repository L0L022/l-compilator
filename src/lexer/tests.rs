use super::*;
use std::fs::{read, read_to_string};

#[test]
fn affect_err() {
    test("affect-err");
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

#[test]
fn tri_ugly() {
    test("tri_ugly");
}

#[test]
fn issue_1() {
    test("issue_1");
}

#[test]
fn issue_2() {
    test("issue_2");
}

#[test]
fn issue_3() {
    test("issue_3");
}

#[test]
fn issue_4() {
    test("issue_4");
}

#[test]
fn alone_read_call() {
    test("alone_read_call");
}

fn test(filename: &str) {
    let l_file = read_to_string(format!("tests/resources/{}.l", filename)).unwrap();
    let lex_file = read(format!("tests/resources/{}.lex", filename)).unwrap();
    let mut generated_lex = Vec::with_capacity(lex_file.capacity());

    Lexer::new(&l_file).into_lex(&mut generated_lex).unwrap();

    print!("{}", String::from_utf8_lossy(&generated_lex));

    assert_eq!(lex_file, generated_lex);
}
