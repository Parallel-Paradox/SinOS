//! # Sensitive information
//!
//! These information should be set manually. Check whether they are initialized or not when use.
//! **DO NOT PUBLIC THEM!**

/// return [`ErrCode::MongoCredentialUnset`] if unset
pub mod mongo_credential {
    use mongodb::options::Credential;
    use crate::config::error::*;

    #[allow(dead_code)]
    pub fn root() -> Result<Credential> {
        Err(ErrCode::MongoCredentialUnset)
    }

    pub fn action_code() -> Result<Credential> {
        Err(ErrCode::MongoCredentialUnset)
    }
}
