//! # Sensitive information
//!
//! These information should be set manually. Check whether they are initialized or not when use.
//! **DO NOT PUBLIC THEM!**

pub mod mongo_credential {
    use mongodb::options::Credential;
    use crate::config::error::*;

    #[allow(dead_code)]
    pub fn root() -> Result<Credential, ErrCode> {
        Err(ERR_MONGO_CREDENTIAL_UNSET)
    }

    pub fn action_code() -> Result<Credential, ErrCode> {
        Err(ERR_MONGO_CREDENTIAL_UNSET)
    }
}
