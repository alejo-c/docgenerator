use crate::csv_parser::CsvData;
use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read};
use zip::ZipArchive;

pub fn process_docx_template(template_path: &str, data: &CsvData) -> Result<String> {
    // Leer plantilla DOCX
    let mut template_file = File::open(template_path)?;
    let mut buffer = Vec::new();
    template_file.read_to_end(&mut buffer)?;

    // Abrir como ZIP
    let reader = Cursor::new(&buffer);
    let mut zip = ZipArchive::new(reader)?;

    // Leer document.xml
    let mut document_xml = String::new();
    {
        let mut file = zip.by_name("word/document.xml")?;
        file.read_to_string(&mut document_xml)?;
    }

    // Crear replacements
    let mut replacements = HashMap::new();
    replacements.insert("{{NOMBRE}}", &data.nombre);
    replacements.insert("{{REVISTA}}", &data.revista);
    replacements.insert("{{INSTITUCION}}", &data.institucion);
    replacements.insert("{{CIUDAD}}", &data.ciudad);

    // Aplicar replacements
    for (placeholder, value) in &replacements {
        document_xml = document_xml.replace(placeholder, value);
    }

    // Extraer texto plano del XML
    let plain_text = extract_text_from_xml(&document_xml)?;

    Ok(plain_text)
}

fn extract_text_from_xml(xml_content: &str) -> Result<String> {
    use xml::reader::{EventReader, XmlEvent};

    let mut text_content = String::new();
    let reader = EventReader::from_str(xml_content);

    for event in reader {
        match event? {
            XmlEvent::Characters(text) => {
                text_content.push_str(&text);
                text_content.push(' ');
            }
            _ => {}
        }
    }

    Ok(text_content.trim().to_string())
}
