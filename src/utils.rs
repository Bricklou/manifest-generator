use serde::{de, ser};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::Path,
};

#[derive(Debug)]
pub enum JsonFileError {
    IO(std::io::Error),
    Json(serde_json::error::Error),
}

impl From<std::io::Error> for JsonFileError {
    fn from(e: std::io::Error) -> Self {
        JsonFileError::IO(e)
    }
}

impl From<serde_json::error::Error> for JsonFileError {
    fn from(e: serde_json::error::Error) -> Self {
        JsonFileError::Json(e)
    }
}

pub fn read_json_from_file<T: de::DeserializeOwned>(
    path: impl AsRef<Path>,
) -> Result<T, JsonFileError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let json: T = serde_json::from_reader(reader)?;

    Ok(json)
}

pub fn save_json_to_file<T: ser::Serialize>(
    path: impl AsRef<Path>,
    json: T,
) -> Result<(), JsonFileError> {
    let file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;

    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &json)?;

    Ok(())
}
