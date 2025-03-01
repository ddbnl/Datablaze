use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
pub struct DatabaseClientError {
    pub(crate) message: String
}
impl fmt::Display for DatabaseClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
