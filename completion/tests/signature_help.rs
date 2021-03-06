#[macro_use]
extern crate collect_mac;
extern crate env_logger;

extern crate gluon_base as base;
extern crate gluon_check as check;
extern crate gluon_completion as completion;
extern crate gluon_parser as parser;

mod support;

use support::*;

use base::source::Source;
use base::types::Type;

use completion::SignatureHelp;

fn signature_help(expr_str: &str, row: usize, column: usize) -> Option<SignatureHelp> {
    let offset = Source::new(expr_str)
        .lines()
        .offset(row.into(), column.into())
        .expect("Position is not in source");
    let (expr, _result) = support::typecheck_partial_expr(expr_str);
    completion::signature_help(&support::MockEnv::new(), &expr, offset)
}

#[test]
fn just_identifier() {
    let _ = env_logger::init();

    let result = signature_help(
        r#"
let test x y : Int -> String -> Int = x
test //
"#,
        2,
        5,
    );
    let expected = Some(SignatureHelp {
        name: "test".to_string(),
        typ: Type::function(collect![typ("Int"), typ("String")], typ("Int")),
        index: Some(0),
    });

    assert_eq!(result, expected);
}

#[test]
fn on_function() {
    let _ = env_logger::init();

    let result = signature_help(
        r#"
let test x y : Int -> String -> Int = x
test 123//
"#,
        2,
        3,
    );
    let expected = Some(SignatureHelp {
        name: "test".to_string(),
        typ: Type::function(collect![typ("Int"), typ("String")], typ("Int")),
        index: None,
    });

    assert_eq!(result, expected);
}

#[test]
fn after_first_argument() {
    let _ = env_logger::init();

    let result = signature_help(
        r#"
let test x y : Int -> String -> Int = x
test 123 //
"#,
        2,
        9,
    );
    let expected = Some(SignatureHelp {
        name: "test".to_string(),
        typ: Type::function(collect![typ("Int"), typ("String")], typ("Int")),
        index: Some(1),
    });

    assert_eq!(result, expected);
}

#[test]
fn inside_argument() {
    let _ = env_logger::init();

    let result = signature_help(
        r#"
let test x y : Int -> String -> Int = x
test { x = "" }
"#,
        2,
        13,
    );
    let expected = Some(SignatureHelp {
        name: "".to_string(),
        typ: typ("String"),
        index: None,
    });

    assert_eq!(result, expected);
}
