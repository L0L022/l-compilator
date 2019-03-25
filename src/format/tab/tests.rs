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
