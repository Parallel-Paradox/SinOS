mod secret;
mod error;

pub use secret::*;
pub use error::*;

pub const SERVER_ADDR: ([u8; 4], u16) = ([127, 0, 0, 1], 3000);

pub const NUM_ALPHABET: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ];
