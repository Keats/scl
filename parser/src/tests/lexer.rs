use pest::Parser;

use ::parser::{parse_str, Rule, SclParser};

macro_rules! assert_lex_rule {
    ($rule: expr, $input: expr) => {
        let res = SclParser::parse($rule, $input);
        println!("{:?}", $input);
        //println!("{:#?}", res);
        if res.is_err() {
            println!("{}", res.unwrap_err());
            panic!();
        }
        assert!(res.is_ok());
        assert_eq!(res.unwrap().last().unwrap().into_span().end(), $input.len());
    };
}

#[test]
fn lex_int() {
    let inputs = vec!["-10", "0", "100", "250000", "1_000", "2_500_000"];
    for i in inputs {
        assert_lex_rule!(Rule::int, i);
    }
}

#[test]
fn lex_comment() {
    let inputs = vec!["# hey", "# hey #", "#hey \n", "# hey //"];
    for i in inputs {
        assert!(SclParser::parse(Rule::comments, i).is_ok());
    }
}

#[test]
fn lex_float() {
    let inputs = vec![
        "0.0", "2.0", "123.5", "123.5", "0.1", "-1.1", "1_000.1", "1_000.1"
    ];
    for i in inputs {
        assert_lex_rule!(Rule::float, i);
    }
}

#[test]
fn lex_byte_size() {
    let inputs = vec!["2MB", "10MB", "100_000GB", "123.5kB", "123.5KB"];
    for i in inputs {
        assert_lex_rule!(Rule::byte_size, i);
    }
}

#[test]
fn lex_string() {
    let inputs = vec!["\"Blabla\"", "\"123\""];
    for i in inputs {
        assert_lex_rule!(Rule::string, i);
    }
}

#[test]
fn lex_multiline_string() {
    let inputs = vec![r#""""Blabla""""#, r#""""a \n\r "'string""""#];
    for i in inputs {
        assert_lex_rule!(Rule::multiline_string, i);
    }
}

#[test]
fn lex_date() {
    let inputs = vec!["1999-11-26", "1999-12-31", "2012-01-01"];
    for i in inputs {
        assert_lex_rule!(Rule::date, i);
    }
}

#[test]
fn lex_key() {
    let inputs = vec!["hello", "hello_", "hello_1", "HELLO", "_1"];
    for i in inputs {
        assert_lex_rule!(Rule::key, i);
    }
}

#[test]
fn lex_env_var() {
    let inputs = vec![
        "${HELLO}",
        "${HELLO  || 1}",
        "${HELLO || true}",
        "${HELLO || \"hello\"}",
        "${HELLO ||1}",
        "${HELLO as integer ||1}",
        "${HELLO as float}",
        "${HELLO as float ||1.0}",
        "${HELLO as bool || false }",
        "${HELLO as date || false }",
    ];
    for i in inputs {
        println!("{:?}", i);
        assert!(SclParser::parse(Rule::env_var, i).is_ok());
    }
}

#[test]
fn lex_value() {
    let inputs = vec![
        "1",
        "true",
        "1.0",
        "10MB",
        "\"ho\"",
        "2012-01-01",
        "${HELLO || \"hel lo\"}",
        "[1,2 ,3]",
        "{}",
        "{ hey = 1}",
    ];
    for i in inputs {
        println!("{} -> {:?}", i, SclParser::parse(Rule::value, i));
        assert!(SclParser::parse(Rule::value, i).is_ok());
    }
}

#[test]
fn lex_array() {
    let inputs = vec![
        "[]",
        "[1,2,3,]",
        "[1, 2,3 ,]",
        "[[1], [2, 4]]",
        r#"[
            1,
            2,
        ]"#,
        r#"[
            1,

              2
        ]"#,
    ];
    for i in inputs {
        println!("{}", i);
        assert!(SclParser::parse(Rule::array, i).is_ok());
    }
}

#[test]
fn lex_key_value() {
    let inputs = vec![
        "hey = 1",
        "hey = true",
        "hey = 2.0",
        "hey = 10MB",
        "hey = \"ho\"",
        "hey = [1, 2, 3]",
        "hey = {}",
        "hey = { hey = 1, ho = {}, }",
        "hey = ${HELLO || \"hel lo\"}",
        "hey = include \"ho.scl\"",
    ];
    for i in inputs {
        println!("{} -> {:?}", i, SclParser::parse(Rule::key_value, i));
        assert!(SclParser::parse(Rule::key_value, i).is_ok());
    }
}

#[test]
fn lex_dict() {
    let inputs = vec![
        "{}",
        "{ hey = 1}",
        "{ hey = 1, }",
        "{hey = 1, ho = 2,}",
        "{hey = 1, ho = 2}",
        "{ hey = { ho = 1 } }",
        "{ hey = include \"ho.scl\"}",
        "{ include \"ho.scl\"}",
        r#"{
            hey = 1,
            ho = 1
        }"#,
        r#"{
            hey = 1, # comment
            ho = 1,
        }"#,
    ];
    for i in inputs {
        assert_lex_rule!(Rule::dict, i);
    }
}

#[test]
fn lex_basic_document() {
    let inputs = vec![
        " hi = true  # base config\n",
        "\n#starting\n",
        r#"
        hi = true  # base config
        hello = "world"
        "#,
        r#"hi = true  # base config

        hello = "world"
        "#,
        r#"
        hi = include "something"  # base config
        include "else"

        hello = "world"
        "#,
    ];
    for i in inputs {
        println!("{}\n--", i);
        //        if let Err(e) = parse(i) {
        //            println!("{}\n{}", i, e);
        //        }
        assert!(SclParser::parse(Rule::document, i).is_ok());
    }
}

#[test]
fn lex_complex_document() {
    let inputs = vec![
        r#"
            # prod config

            # include "base.scl"
            debug = true

            title = "hey"
            ho = [1,2 ,3] # some comment
            hey = [
                # a comment here
                1, # one here
                2
                # and one here
            ]
            max_upload_size = 10MB

            # other = include "other.scl"

            # Database

            db = { url = "blabla", password = ${DB_PASSWORD || "****" } } # here
            users = {
                # one comment here
                me = { # or here
                    something = 1MB, # comment
                    admin = true,
                    # between dict keys
                    over = 1 # and after
                } # one more
            } # last one
            "#,
    ];
    for i in inputs {
        if let Err(e) = parse_str(i) {
            println!("{}\n{}", i, e);
        }
        assert!(SclParser::parse(Rule::document, i).is_ok());
    }
}

#[test]
fn lex_newline_required_in_document() {
    let inputs = vec![r#"include "hey" hey = 1"#, r#"hey = 1 ho = true"#];
    for i in inputs {
        println!("{} -> {:?}", i, SclParser::parse(Rule::document, i));
        assert!(SclParser::parse(Rule::document, i).is_err());
    }
}
