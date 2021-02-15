use std::fmt;
#[derive(Debug, Clone)]
pub struct GrihFullError;

impl fmt::Display for GrihFullError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No space left for more user!")
    }
}

