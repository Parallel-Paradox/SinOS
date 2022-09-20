//! # Sensitive information
//!
//! These information should be set manually. Check whether they are initialized or not when use.
//! **DO NOT PUBLIC THEM!**

pub struct MongoAccount {
    pub usr: String,
    pub pwd: String,
}
pub const MONGO_ACCOUNT: Result<MongoAccount, &str> =
    Err("Mongo account should be set manually.");
