use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct ParseError { 
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
