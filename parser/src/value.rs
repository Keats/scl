use std::collections::BTreeMap;

/// Representation of a SCL value.
#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Date(Date),
    Array(Array),
    Dict(Dict),
}

impl Value {
    /// Tests whether this and another value have the same type.
    pub fn same_type(&self, other: &Value) -> bool {
        match (self, other) {
            (&Value::String(..), &Value::String(..))
            | (&Value::Integer(..), &Value::Integer(..))
            | (&Value::Float(..), &Value::Float(..))
            | (&Value::Boolean(..), &Value::Boolean(..))
            | (&Value::Date(..), &Value::Date(..))
            | (&Value::Array(..), &Value::Array(..))
            | (&Value::Dict(..), &Value::Dict(..)) => true,
            _ => false,
        }
    }

    /// Returns a human-readable representation of the type of this value.
    pub fn type_str(&self) -> &'static str {
        match *self {
            Value::String(..) => "string",
            Value::Integer(..) => "integer",
            Value::Float(..) => "float",
            Value::Boolean(..) => "bool",
            Value::Date(..) => "date",
            Value::Array(..) => "array",
            Value::Dict(..) => "dict",
        }
    }
}

pub type Array = Vec<Value>;
pub type Dict = BTreeMap<String, Value>;

#[derive(Clone, Debug, PartialEq)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

impl Date {
    pub fn from_str(input: &str) -> Date {
        // TODO: error handling
        let parts: Vec<&str> = input.split('-').collect();
        Date {
            year: parts[0].parse().unwrap(),
            month: parts[1].parse().unwrap(),
            day: parts[2].parse().unwrap(),
        }
    }

    pub fn to_iso(&self) -> String {
        "TODO".to_string()
    }
}
