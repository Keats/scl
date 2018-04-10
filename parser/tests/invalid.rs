extern crate scl_parser;

use scl_parser::{parse_file, Error};

fn assert_error_msg(filename: &str, needle: &str) {
    let res = parse_file(&format!("./tests/invalid/{}.scl", filename));
    assert!(res.is_err());
    let err = res.unwrap_err();
    match err {
        Error::InvalidSyntax(msg) => {
            println!("{}", msg);
            assert!(msg.contains(needle));
        }
    }
}

// Invalid syntax errors

#[test]
fn test_eof() {
    assert_error_msg(
        "eof",
        "expected include or string / int / float / byte size / date / bool / array / dict / environment variable"
    );
}

#[test]
fn test_invalid_int() {
    assert_error_msg(
        "invalid_int",
        "expected a byte size unit (kB / MB / GB / TB / PB)",
    );
}

#[test]
fn test_env_var_default() {
    assert_error_msg(
        "env_var_default",
        "expected a boolean (true / false), a string, a multiline string, an integer, a float, or a date"
    );
}

#[test]
fn test_invalid_key() {
    assert_error_msg("invalid_key", "expected include or a key");
}

#[test]
fn test_invalid_document() {
    assert_error_msg(
        "invalid_doc",
        "expected a key value, an include or a comment",
    );
}

#[test]
fn test_invalid_date() {
    assert_error_msg(
        "invalid_date",
        "expected a byte size unit (kB / MB / GB / TB / PB)",
    );
}

#[test]
fn test_invalid_array_comment() {
    assert_error_msg(
        "invalid_array_comment",
        "expected string / int / float / byte size / date / bool / array / dict / environment variable"
    );
}
