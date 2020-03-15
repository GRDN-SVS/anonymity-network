use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ConfigParseError;

impl Error for ConfigParseError {}

impl fmt::Display for ConfigParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The configuration file could not be parsed correctly!")
    }
}
