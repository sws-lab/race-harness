use std::path::PathBuf;

use crate::harness::{core::error::HarnessError, harness::symbolic_harness::SymbolicHarness};

pub trait DSLInterpreter {
    type DSLUnit;

    fn parse<Input>(&self, input: &mut Input) -> Result<Self::DSLUnit, HarnessError>
        where Input: Iterator<Item = Result<char, HarnessError>>;
    fn interpret(&self, unit: &Self::DSLUnit, include_base_path: Option<PathBuf>) -> Result<SymbolicHarness, HarnessError>;
}
