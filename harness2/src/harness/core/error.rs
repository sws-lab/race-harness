use std::{error::Error, fmt::Display};


#[derive(Debug)]
pub struct HarnessError(pub String);

impl Display for HarnessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Harness error: {}", self.message())
    }
}

impl Error for HarnessError {}

impl HarnessError {
    pub fn message(&self) -> &str {
        self.0.as_str()
    }
}
