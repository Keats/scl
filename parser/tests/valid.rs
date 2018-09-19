extern crate scl_parser;
#[macro_use]
extern crate pretty_assertions;

use std::env;

use scl_parser::{parse_file, Date, Dict, Value as V};

macro_rules! btreemap {
    // trailing comma case
    ($($key:expr => $value:expr,)+) => (btreemap!($($key => $value),+));

    ( $($key:expr => $value:expr),* ) => {
        {
            let mut _map = ::std::collections::BTreeMap::new();
            $(
                let _ = _map.insert($key.to_string(), $value);
            )*
            _map
        }
    };
}

fn assert_valid(filename: &str, expected: Dict) {
    let res = parse_file(&format!("./tests/valid/{}.scl", filename));
    if let Err(e) = res.clone() {
        println!("{}", e);
    }
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), expected);
}

#[test]
fn test_basic() {
    assert_valid(
        "basic",
        btreemap!(
            "debug" => V::Boolean(true),
            "title" => V::String("hey".to_string()),
            "ho" => V::Array(vec![V::Integer(1), V::Integer(2), V::Integer(3)]),
            "hey" => V::Array(vec![V::Integer(1), V::Integer(2)]),
            "max_upload_size" => V::Integer(10000000),
            "db" => V::Dict(btreemap!(
                "url" => V::String("blabla".to_string()),
                "password" => V::String("****".to_string()),
            )),
            "users" => V::Dict(btreemap!(
                "me" => V::Dict(btreemap!(
                    "admin" => V::Boolean(true),
                ))
            ))
        ),
    );
}

#[test]
fn test_with_includes() {
    env::set_var("STRIPE_KEY", "YOUHOU");
    assert_valid("with_includes", btreemap!(
        "debug" => V::Boolean(false),
        "hostname" => V::String("something else".to_string()),
        "max_upload_size" => V::Integer(1000000000),
        "logging" => V::Dict(btreemap!(
            "enabled" => V::Boolean(true),
            "dir" => V::String("/var/logs".to_string()),
        )),
        "secrets" => V::Dict(btreemap!(
            "db" => V::String("pass".to_string()),
            "stripe" => V::String("YOUHOU".to_string()),
        ))
    ));
}

#[test]
fn test_cargo() {
    assert_valid(
        "cargo",
        btreemap!(
            "package" => V::Dict(btreemap!(
                "name" => V::String("scl-parser".to_string()),
                "version" => V::String("0.1.0".to_string()),
                "authors" => V::Array(vec![V::String("Vincent Prouillet".to_string())]),
            )),
            "dependencies" => V::Dict(btreemap!(
                "pest" => V::String("^1.0.0".to_string()),
                "pest_derive" => V::String("^1.0.0".to_string()),
            )),
            "dev-dependencies" => V::Dict(btreemap!(
                "tempdir" => V::String("0.3".to_string()),
                "pretty_assertions" => V::String("0.5".to_string()),
            ))
        ),
    );
}

#[test]
fn test_toml_example_converted() {
    assert_valid(
        "toml_example",
        btreemap!(
            "title" => V::String("TOML Example".to_string()),
            "owner" => V::Dict(btreemap!(
                "name" => V::String("Tom Preston-Werner".to_string()),
                "dob" => V::Date(Date { year: 1979, month: 05, day: 27}),
            )),
            "database" => V::Dict(btreemap!(
                "server" => V::String("192.168.1.1".to_string()),
                "ports" => V::Array(vec![V::Integer(8001), V::Integer(8001), V::Integer(8002)]),
                "connection_max" => V::Integer(5000),
                "enabled" => V::Boolean(true),
            )),
            "servers" => V::Dict(btreemap!(
                "alpha" =>  V::Dict(btreemap!(
                    "ip" => V::String("10.0.0.1".to_string()),
                    "dc" => V::String("eqdc10".to_string()),
                )),
                "beta" =>  V::Dict(btreemap!(
                    "ip" => V::String("10.0.0.2".to_string()),
                    "dc" => V::String("eqdc10".to_string()),
                )),
            )),
            "clients" => V::Dict(btreemap!(
                "data" =>  V::Array(vec![
                    V::Array(vec![V::String("gamma".to_string()), V::String("delta".to_string())]),
                    V::Array(vec![V::Integer(1), V::Integer(2)])
                ]),
                "hosts" => V::Array(vec![V::String("alpha".to_string()), V::String("omega".to_string())]),
            )),
        ),
    );
}
