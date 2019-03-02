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
fn affect_err() {
    test("affect-err");
}

#[test]
fn appel() {
    test("appel");
}

#[test]
fn associativite() {
    test("associativite");
}

#[test]
fn calculette() {
    test("calculette");
}

#[test]
fn procedure_arg() {
    test("procedure_arg");
}

#[test]
fn procedure() {
    test("procedure");
}

#[test]
fn procedure_retour() {
    test("procedure_retour");
}

#[test]
fn procedure_varloc() {
    test("procedure_varloc");
}

#[test]
fn si() {
    test("si");
}

#[test]
fn sinon() {
    test("sinon");
}

#[test]
fn tableau2() {
    test("tableau2");
}

#[test]
fn tableau() {
    test("tableau");
}

#[test]
fn tantque0() {
    test("tantque0");
}

#[test]
fn tantque() {
    test("tantque");
}

#[test]
fn lexunits() {
    test("lexunits");
}

#[test]
fn factorielle() {
    test("factorielle");
}

#[test]
fn fibo() {
    test("fibo");
}

#[test]
fn pgcd() {
    test("pgcd");
}

#[test]
fn sommeneg() {
    test("sommeneg");
}

#[test]
fn lex_err() {
    test("lex-err");
}

#[test]
fn synt_err() {
    test("synt-err");
}

#[test]
fn extra() {
    test("extra");
}

#[test]
fn ordre() {
    test("ordre");
}

#[test]
fn three_three_a() {
    test("33a");
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
