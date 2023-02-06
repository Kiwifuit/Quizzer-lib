use std::io::{Read, Write};

use crate::quiz::Quiz;
use bincode::{deserialize_from, serialize_into};

type DataError = bincode::Error;

pub fn load<R>(file: &mut R) -> Result<Quiz, DataError>
where
    R: Read,
{
    deserialize_from(file)
}

pub fn dump<W>(file: &mut W, data: &Quiz) -> Result<(), DataError>
where
    W: Write,
{
    serialize_into(file, data)
}
