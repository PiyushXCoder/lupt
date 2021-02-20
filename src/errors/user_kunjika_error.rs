use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct AlreadyExistError;

impl fmt::Display for AlreadyExistError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User kunjika already exist!")
    }
}

impl Error for AlreadyExistError {
    
}