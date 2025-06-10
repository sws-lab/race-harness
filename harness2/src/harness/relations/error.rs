use std::{error::Error, fmt::{Debug, Display}};

use crate::harness::core::error::HarnessError;

pub enum Sqlite3RelationsDbError {
    Harness(HarnessError),
    Sqlite3(rusqlite::Error)
}

impl Error for Sqlite3RelationsDbError {}

impl Display for Sqlite3RelationsDbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Harness(err) => Display::fmt(err, f),
            Self::Sqlite3(err) => Display::fmt(err, f)
        }
    }
}

impl Debug for Sqlite3RelationsDbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl From<rusqlite::Error> for Sqlite3RelationsDbError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Sqlite3(value)
    }
}

impl From<HarnessError> for Sqlite3RelationsDbError {
    fn from(value: HarnessError) -> Self {
        Self::Harness(value)
    }
}