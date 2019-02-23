use super::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
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
    let asynt_file = format!("tests/resources/{}.asynt", filename);

    let parser = Parser::new().parse(Lexer::new(&l_file));

    if Path::new(&asynt_file).is_file() {
        let asynt_file = read(asynt_file).unwrap();
        let mut generated_asynt = Vec::with_capacity(asynt_file.capacity());

        parser.unwrap().to_asynt(&mut generated_asynt, 0).unwrap();

        print!("{}", String::from_utf8_lossy(&generated_asynt));

        assert!(asynt_file == generated_asynt);
    } else {
        assert!(parser.is_err());
    }
}
