use anyhow::Result;
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize, Clone)]
pub struct CsvData {
    #[serde(rename = "Código")]
    pub code: String,
    #[serde(rename = "Formación")]
    pub formation: String,
    #[serde(rename = "Nombre")]
    pub name: String,
    #[serde(rename = "Titulos")]
    pub degrees: String,
    #[serde(rename = "Rol")]
    pub role: String,
    #[serde(rename = "Revista")]
    pub journal: String,
    #[serde(rename = "Institución")]
    pub institution: String,
    #[serde(rename = "Ubicación")]
    pub location: String,
    #[serde(rename = "Respetado")]
    pub respected: String,
    #[serde(rename = "Invitarlo")]
    pub invite: String,
    // #[serde(rename = "Dirección de correo")]
    // pub email: String,
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
