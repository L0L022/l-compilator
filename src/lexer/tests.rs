use super::*;
use std::fs::read_to_string;

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

fn test(filename: &str) {
    let l_file = read_to_string(format!("tests/resources/{}.l", filename)).unwrap();
    let lex_file = read_to_string(format!("tests/resources/{}.lex", filename)).unwrap();

    let generated_lex = Lexer::new(&l_file).into_lex().unwrap();

    assert_eq!(lex_file, generated_lex);
}
