use std::io::{Read, Write};

use crate::quiz::Quiz;
use bincode::{deserialize_from, serialize_into};
use serde::Serialize;

type DataError = bincode::Error;

pub fn load<R>(file: &mut R) -> Result<Quiz, DataError>
where
    R: Read,
{
    deserialize_from(file)
}

pub fn dump<W, D>(file: &mut W, data: &D) -> Result<(), DataError>
where
    W: Write,
    D: Serialize,
{
    serialize_into(file, data)
}
