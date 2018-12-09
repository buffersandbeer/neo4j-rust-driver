use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct PackingError {
    pub details: String    
}

impl PackingError {
    pub fn new(msg: &str) -> PackingError {
        PackingError{ details: msg.to_string() }
    }
}

impl fmt::Display for PackingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for PackingError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[cfg(test)]
mod tests {
    use super::{PackingError};

    fn raise_packing_err(raise: bool) -> Result<(), PackingError> {
        if raise {
            Err(PackingError::new("error"))
        } else {
            Ok(())
        }
    }

    #[test]
    fn test_packing_error() {
        match raise_packing_err(true) {
            Ok(_) => assert!(false, "The error was not raised"),
            Err(err) => assert_eq!(err.details, "error", "Error was raised but not correct")
        }
    }
}
