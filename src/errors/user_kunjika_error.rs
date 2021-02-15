use std::fmt;

#[derive(Debug, Clone)]
pub struct AlreadyExistError;

impl fmt::Display for AlreadyExistError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User kunjika already exist!")
    }
}