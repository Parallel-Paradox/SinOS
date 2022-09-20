//! # Sensitive information
//!
//! These information should be set manually. Check whether they are initialized or not when use.
//! **DO NOT PUBLIC THEM!**

use mongodb::options::Credential;

pub const MONGO_CREDENTIAL_ACTION_CODE: Result<Credential, &str> =
    Err("Mongo account should be set manually.");
