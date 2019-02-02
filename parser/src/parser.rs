use std::path::Path;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use pest::Parser;
use pest::iterators::Pair;

use errors::Error;
use value::{Date, Dict, Value};


// This include forces recompiling this source file if the grammar file changes.
// Uncomment it when doing changes to the .pest file
#[cfg(debug_assertions)]
const _GRAMMAR: &str = include_str!("scl.pest");


#[derive(Parser)]
#[grammar = "scl.pest"]
pub struct SclParser;


/// A struct that keeps the state of the current file being parsed
/// in order for the include to work and for the errors to point
/// to the file.
/// It is also used when parsing a string.
#[derive(Debug, PartialEq, Default)]
struct ParserState<'a> {
    /// If the path is `None`, we're parsing a string and the include
    /// should just resolve in whatever directory we're in
    path: Option<&'a Path>,
}

fn escape_string (s: &str) -> Result<String, Error> {

    fn next <T: Iterator<Item=char>> (iter: &mut T) -> Result<char, Error> {
        iter.next().ok_or_else(||
            Error::InvalidSyntax("Unfinished escape sequence".to_string())
        )
    }

    fn digit_from_char (ch: char) -> Result<u32, Error> {
        ch.to_digit(16).ok_or_else(||
            Error::InvalidSyntax("Invalid hexadecimal digit".to_string())
        )
    }

    let mut result = String::with_capacity(s.len() - 2);

    let mut chars = s[1 .. s.len()-1].chars();

    while let Some(c) = chars.next() {
        if c == '\\' {
            let c = next(&mut chars)?;
            match c {
                '\\' => result.push('\\'),
                '"' => result.push('"'),
                'n' => result.push('\n'),
                'r' => result.push('\r'),
                'x' => {
                    let a = digit_from_char(next(&mut chars)?)?;
                    let b = digit_from_char(next(&mut chars)?)?;
                    let n = a*16 + b;
                    if n > 127 {
                        return Err(Error::InvalidSyntax("Not an ASCII value".to_string()));
                    }
                    result.push(n as u8 as char);
                }
                c => return Err(Error::InvalidSyntax(
                    format!("Invalid escape sequence: \\{}", c)
                ))
            }
        } else {
            result.push(c);
        }
    }

    Ok(result)
}

impl<'a> ParserState<'a> {
    // TODO: error on different cast/default type
    fn parse_env_var(&self, pair: Pair<Rule>) -> Value {
        let mut key = None;
        let mut cast = None;
        let mut default = None;

        for p in pair.into_inner() {
            match p.as_rule() {
                Rule::key => {
                    key = Some(p.into_span().as_str().to_string());
                },
                Rule::env_var_cast => {
                    cast = Some(p.into_span().as_str().to_string());
                },
                _ => {
                    default = Some(self.parse_value(p));
                }
            };
        }

        if let Some(ref c) = cast {
            if let Some(ref d) = default {
                if c != d.type_str() {
                    panic!("TO IMPLEMENT: error on different cast/default type")
                }
            }
        }

        match env::var(&key.unwrap()) {
            Ok(s) => {
                if let Some(c) = cast {
                    // TODO: error handling
                    match c.as_str() {
                        "integer" => Value::Integer(s.parse().unwrap()),
                        "float" => Value::Float(s.parse().unwrap()),
                        "bool" => Value::Boolean(s.parse().unwrap()),
                        "date" => Value::Date(Date::from_str(&c)),
                        _ => unreachable!()
                    }
                } else {
                    Value::String(s)
                }
            },
            Err(_) => default.unwrap(),
        }
    }

    // TODO: return an error if types are different
    fn parse_array(&self, pair: Pair<Rule>) -> Value {
        let mut items = vec![];

        for p in pair.into_inner() {
            // we can only have Rule::Value here, no need to match
            let val = self.parse_value(p.into_inner().next().unwrap());
            if let Some(last) = items.last() {
                if !val.same_type(last) {
                    // TODO: return an error
                }
            }
            items.push(val);
        }

        Value::Array(items)
    }

    fn parse_byte_size(&self, pair: Pair<Rule>) -> Value {
        let mut num: Option<f64> = None;

        for p in pair.into_inner() {
            match p.as_rule() {
                Rule::byte_size_number => {
                    num = Some(p.as_str().parse().unwrap());
                }
                Rule::byte_size_unit => {
                    let n = num.unwrap();
                    let res = match p.as_str() {
                        "kB" | "KB" => n * 1e3,
                        "MB" => n * 1e6,
                        "GB" => n * 1e9,
                        "TB" => n * 1e12,
                        "PB" => n * 1e15,
                        _ => unreachable!(),
                    };

                    return Value::Integer(res as i64);
                }
                _ => unreachable!(),
            }
        }

        unreachable!("Got a byte size without a unit?")
    }

    fn parse_value(&self, pair: Pair<Rule>) -> Value {
        match pair.as_rule() {
            Rule::int => Value::Integer(pair.as_str().parse().unwrap()),
            Rule::float => Value::Float(pair.as_str().parse().unwrap()),
            Rule::byte_size => self.parse_byte_size(pair),
            Rule::boolean => match pair.as_str() {
                "true" => Value::Boolean(true),
                "false" => Value::Boolean(false),
                _ => unreachable!(),
            },
            Rule::string => Value::String(escape_string(pair.as_str()).unwrap()),
            Rule::multiline_string => {
                let text = pair.as_str().replace("\"\"\"", "");
                if text.starts_with('\n') {
                    Value::String(text.trim_left().to_string())
                } else {
                    Value::String(text.to_string())
                }
            }
            Rule::env_var => self.parse_env_var(pair),
            Rule::date => Value::Date(Date::from_str(pair.as_str())),
            Rule::array => self.parse_array(pair),
            Rule::dict => Value::Dict(self.parse_dict(pair).unwrap()), // todo: error handling
            _ => unreachable!("Got an unexpected value: {:?}", pair),
        }
    }

    fn parse_key_value(&self, pair: Pair<Rule>) -> (String, Value) {
        let mut key = None;
        let mut value = None;

        for p in pair.into_inner() {
            match p.as_rule() {
                Rule::key => {
                    key = Some(p.into_span().as_str().to_string());
                }
                // The grammar made sure we can only have one value or an include
                Rule::value => {
                    value = Some(self.parse_value(p.into_inner().next().unwrap()));
                }
                Rule::include => {
                    value = Some(Value::Dict(self.parse_include(p).unwrap())); // TODO: error handling
                }
                _ => unreachable!("Got something in key/value other than a key/value: {:?}", p),
            };
        }

        (key.unwrap(), value.unwrap())
    }

    fn parse_dict(&self, pair: Pair<Rule>) -> Result<Dict, Error> {
        let mut dict = Dict::new();

        for p in pair.into_inner() {
            match p.as_rule() {
                Rule::include => {
                    let included = self.parse_include(p)?;
                    dict.extend(included);
                }
                Rule::key_value => {
                    let (key, value) = self.parse_key_value(p);
                    dict.insert(key, value);
                }
                _ => unreachable!("unknown dict rule: {:?}", p.as_rule()),
            }
        }

        Ok(dict)
    }

    fn parse_include(&self, pair: Pair<Rule>) -> Result<Dict, Error> {
        // next inner token is the filename
        let path = pair.into_inner()
            .next()
            .unwrap()
            .into_span()
            .as_str()
            .replace("\"", "");

        // we have to deal wih an include
        // - if we do not have a current path, just call `parse_file`, we can't
        // give any context
        // - if we have one, first join the current dir and the filename

        if let Some(current_path) = self.path {
            // if the path is absolute, don't append the current path to it
            if path.starts_with('/') {
                parse_file(path)
            } else {
                // TODO: error handling for parent()?
                let full_path = current_path.parent().unwrap().join(path);
                parse_file(&full_path)
            }
        } else {
            parse_file(path)
        }
    }

    /// Parse the given string
    pub fn parse_str(&self, input: &str) -> Result<Dict, Error> {
        let mut pairs = match SclParser::parse(Rule::document, input) {
            Ok(p) => p,
            Err(e) => {
                let fancy_e = e.renamed_rules(|rule| {
                    match *rule {
                        Rule::document => "a key value, an include or a comment".to_string(),
                        Rule::key => "a key".to_string(),
                        Rule::boolean => "a boolean (true / false)".to_string(),
                        Rule::string => "a string".to_string(),
                        Rule::multiline_string => "a multiline string".to_string(),
                        Rule::int => "an integer".to_string(),
                        Rule::float => "a float".to_string(),
                        Rule::date => "a date".to_string(),
                        Rule::key_value => "a key value".to_string(),
                        Rule::byte_size_unit => "a byte size unit (kB / MB / GB / TB / PB)".to_string(),
                        Rule::value => "string / int / float / byte size / date / bool / array / dict / environment variable".to_string(),
                        Rule::include => "include".to_string(),
                        Rule::byte_size_number => "a number".to_string(),
                        Rule::env_var => "an environment variable".to_string(),
                        Rule::env_var_cast => "a cast to integer/float/date/bool".to_string(),
                        Rule::array => "an array".to_string(),
                        Rule::dict => "a dictionary".to_string(),
                        _ => format!("TODO: {:?}", rule),
                    }
                });
                return Err(Error::InvalidSyntax(format!("{}", fancy_e)));
            }
        };

        // We must have at least a `document` pair if we got there
        self.parse_dict(pairs.next().unwrap())
    }

}

/// Parse the file at the given path
pub fn parse_file<T: AsRef<Path>>(path: T) -> Result<Dict, Error> {
    let mut f = File::open(&path).expect("file not found");
    let mut contents = String::new();
    // TODO: error handling
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let state = ParserState { path: Some(&path.as_ref()) };

    state.parse_str(&contents)
}

/// Parse the file at the given path
pub fn parse_str(input: &str) -> Result<Dict, Error> {
    let state = ParserState { path: None };

    state.parse_str(&input)
}
