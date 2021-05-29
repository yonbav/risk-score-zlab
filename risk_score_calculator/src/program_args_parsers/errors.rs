use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ArgsError {
    details: String
}

impl ArgsError {
    pub fn new(msg: &str) -> ArgsError {
        ArgsError{details: msg.to_string()}
    }
}

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for ArgsError {
    fn description(&self) -> &str {
        &self.details
    }
}