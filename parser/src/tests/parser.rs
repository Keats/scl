use std::env;
use std::io::prelude::*;
use std::fs::File;

use tempdir::TempDir;

use ::parser::parse_str;
use value::{Date, Dict, Value};

#[test]
fn parse_empty_document() {
    let doc = parse_str("").unwrap();
    assert_eq!(doc.len(), 0);
}

#[test]
fn parse_comment() {
    let doc = parse_str("# hello world").unwrap();
    assert_eq!(doc.len(), 0);
}

#[test]
fn parse_simple_key_value() {
    // for the env variable case
    env::set_var("SCL_TEST", "YOUHOU");
    env::set_var("SCL_PORT", "5555");

    let inputs = vec![
        ("val = 2", Value::Integer(2)),
        ("val = 2", Value::Integer(2)),
        ("val = 1.5GB", Value::Integer(1.5e9 as i64)),
        ("val = 2.0", Value::Float(2.0)),
        ("val = true", Value::Boolean(true)),
        ("val = false", Value::Boolean(false)),
        (r#"val = "a string""#, Value::String("a string".to_string())),
        (
            r#"val = """a \n\r "'string""""#,
            Value::String(r#"a \n\r "'string"#.to_string()),
        ),
        (
            "val = 2018-01-01",
            Value::Date(Date {
                year: 2018,
                month: 1,
                day: 1,
            }),
        ),
        (
            "val = [1, 2, 3]",
            Value::Array(vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3),
            ]),
        ),
        ("val = ${SCL_TEST}", Value::String("YOUHOU".to_string())),
        ("val = ${INEXISTENT || 1}", Value::Integer(1)),
        ("val = ${INEXISTENT as bool || true}", Value::Boolean(true)),
        ("val = ${SCL_PORT as integer || 1}", Value::Integer(5555)),
        ("val = {}", Value::Dict(Dict::new())),
    ];

    for (text, val) in inputs {
        println!("{}", text);
        if let Err(e) = parse_str(text) {
            println!("{}", e);
        }
        let doc = parse_str(text).unwrap();
        assert_eq!(doc.len(), 1);
        assert_eq!(doc["val"], val);
    }
}

#[test]
fn parse_multiline_followed_by_newline() {
    let doc = parse_str(
        r#"
        val = """
        hey"""
    "#,
    ).unwrap();
    assert_eq!(doc.len(), 1);
    assert_eq!(doc["val"], Value::String("hey".to_string()));
}

#[test]
fn parse_dict() {
    let mut expected = Dict::new();
    expected.insert("name".to_string(), Value::String("pg".to_string()));
    expected.insert("port".to_string(), Value::Integer(5555));
    let doc = parse_str(
        r#"
        val = { name = "pg", port = 5555, }
    "#,
    ).unwrap();
    assert_eq!(doc.len(), 1);
    assert_eq!(doc["val"], Value::Dict(expected));
}

// creates a couple of file that can be included
fn create_test_files(dir: &TempDir) {
    let file_path = dir.path().join("a.scl");
    let mut f = File::create(file_path).unwrap();
    f.write_all(br#"key = 1 # a value"#).unwrap();
    f.sync_all().unwrap();

    let file_path = dir.path().join("b.scl");
    let mut f = File::create(file_path).unwrap();
    f.write_all(
        br#"key = {
        name = "hey"
        database = { port = 2000, }
    } # a value"#,
    ).unwrap();
    f.sync_all().unwrap();
}

#[test]
fn parse_local_include() {
    let tmp_dir = TempDir::new("tests").unwrap();
    create_test_files(&tmp_dir);
    env::set_current_dir(tmp_dir.path()).unwrap();

    let doc = parse_str(r#"include "a.scl""#).unwrap();
    assert_eq!(doc.len(), 1);
    assert_eq!(doc["key"], Value::Integer(1));
}

#[test]
fn parse_non_local_include() {
    let tmp_dir = TempDir::new("tests").unwrap();
    create_test_files(&tmp_dir);
    let file_path = tmp_dir.path().join("hey.scl");
    let mut f = File::create(&file_path).unwrap();
    f.write_all(br#"key = true # a value"#).unwrap();
    f.sync_all().unwrap();

    let doc = parse_str(&format!(r#"include "{}""#, file_path.display())).unwrap();
    assert_eq!(doc.len(), 1);
    assert_eq!(doc["key"], Value::Boolean(true));
}

#[test]
fn parse_include_on_key() {
    let mut expected = Dict::new();
    expected.insert("key".to_string(), Value::Integer(1));

    let tmp_dir = TempDir::new("tests").unwrap();
    create_test_files(&tmp_dir);
    env::set_current_dir(tmp_dir.path()).unwrap();

    let doc = parse_str(r#"hey = include "a.scl""#).unwrap();
    assert_eq!(doc.len(), 1);
    assert_eq!(doc["hey"], Value::Dict(expected));
}
