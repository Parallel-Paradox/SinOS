//! # Sensitive information
//!
//! These information should be set manually. Check whether they are initialized or not when use.
//! **DO NOT PUBLIC THEM!**

use mongodb::options::Credential;
use crate::constant::error::*;

#[allow(dead_code)]
pub const MONGO_CREDENTIAL_ROOT: Result<Credential, ErrCode> =
    Err(ERR_MONGO_CREDENTIAL_UNSET);

pub const MONGO_CREDENTIAL_ACTION_CODE: Result<Credential, ErrCode> =
    Err(ERR_MONGO_CREDENTIAL_UNSET);
