use std::fmt;

// TODO: proper error handling/reporting
#[derive(PartialEq, Debug, Clone)]
pub enum Error {
    InvalidSyntax(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidSyntax(ref s) => write!(f, "{}", s),
        }
    }
}
