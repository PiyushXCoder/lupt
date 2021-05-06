use std::fmt;
#[derive(Debug, Clone)]
pub struct KakshFullError;

impl fmt::Display for KakshFullError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No space left for more user!")
    }
}

