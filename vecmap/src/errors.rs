use std::fmt::{Display, Formatter, Result};
use std::error::Error;

#[derive(Debug)]
pub struct ElementNotFount;

#[derive(Debug)]
pub struct KeyAlreadyExist;

impl Display for ElementNotFount {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Element not found")
    }
}

impl Error for ElementNotFount {}

impl Display for KeyAlreadyExist {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Key already do exist")
    }
}

impl Error for KeyAlreadyExist {}