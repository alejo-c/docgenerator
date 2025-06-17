use anyhow::Result;
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize, Clone)]
pub struct CsvData {
    #[serde(rename = "Nombre")]
    pub nombre: String,
    #[serde(rename = "Revista")]
    pub revista: String,
    #[serde(rename = "Institución")]
    pub institucion: String,
    #[serde(rename = "Ciudad")]
    pub ciudad: String,
    #[serde(rename = "Correo")]
    pub correo: String,
}

pub fn parse_csv(file_path: &str) -> Result<Vec<CsvData>> {
    let file = File::open(file_path)?;
    let mut reader = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);

    let mut records = Vec::new();
    for result in reader.deserialize() {
        let record: CsvData = result?;
        records.push(record);
    }

    Ok(records)
}
