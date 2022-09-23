#![allow(unused)]

use std::fmt::Display;

#[derive(Debug)]
pub enum ErrCode {
    /// Mongo account should be set manually.
    MongoCredentialUnset,
    /// Mongo read write error, See [`mongodb::error::Error`]
    MongoRwError(mongodb::error::Error),
}

impl ErrCode {
    fn get_code(&self) -> usize {
        match &self {
            Self::MongoCredentialUnset => 120,
            Self::MongoRwError(_) => 121,
        }
    }
}

impl Display for ErrCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "err: {} ", self.get_code())?;
        match &self {
            _ => { /* Ignore the enum that has no content inside */ },
            Self::MongoRwError(err) => { write!(f, "{:?}", err)?; },
        }
        Ok(())
    }
}

impl From<mongodb::error::Error> for ErrCode {
    fn from(err: mongodb::error::Error) -> Self { Self::MongoRwError(err) }
}
