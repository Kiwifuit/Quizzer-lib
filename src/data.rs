//! Functions to `load` and `dump` quiz data
//! # Example
//! ```rs
//! let raw_values = vec![
//!   (1u8, "Do you love her?", "yes"),
//!   (1u8, "Amoogns cronut", "no"),
//!   (1u8, "Weeeeeeeeeeee", "yes"),
//!   (1u8, "Fourth question?", "yes"),
//!   (1u8, "Is the next question of the same type?", "no"),
//!   (2u8, "Who is the bestest doggo in the world?", "Rosie"),
//!   (2u8, "Who is the smelliest doggo in the world?", "Tasha"),
//!   (3u8, "What is life?", "Damn bro idk"),
//!   (
//!       3u8,
//!       "What is this hit game called Amongus?",
//!       "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
//!   ),
//! ];
//!
//! let quiz = quizzer::Quiz::from(raw_values)
//! let mut file = OpenOptions::new()
//!                            .write(true)
//!                            .create(true)
//!                            .open("dump.bin");
//!
//! quizzer::dump(&mut file, &quiz);
//! ```

use std::io::{Read, Write};

use crate::quiz::Quiz;

cfg_if::cfg_if! {
    if #[cfg(feature = "json")] {
        use serde_json::{from_reader as deserialize, to_writer as serialize};
        type DataError = serde_json::Error;
    } else {
        use bincode::{deserialize_from as deserialize, serialize_into as serialize};
        type DataError = bincode::Error;
    }
}

pub fn load<R>(file: &mut R) -> Result<Quiz, DataError>
where
    R: Read,
{
    deserialize(file)
}

pub fn dump<W>(file: &mut W, data: &Quiz) -> Result<(), DataError>
where
    W: Write,
{
    serialize(file, data)
}
