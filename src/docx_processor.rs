use crate::csv_parser::CsvData;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::io::{Cursor, Read, Write};
use zip::write::FileOptions;
use zip::{ZipArchive, ZipWriter};

pub fn process_docx_template(template_path: &str, data: &CsvData, output_path: &str) -> Result<()> {
    // Read template DOCX
    let template_data = fs::read(template_path)?;
    let reader = Cursor::new(&template_data);
    let mut zip = ZipArchive::new(reader)?;

    // Create output ZIP
    let output_file = fs::File::create(output_path)?;
    let mut output_zip = ZipWriter::new(output_file);

    // Process each file in the ZIP
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let name = file.name().to_string();

        if name == "word/document.xml" {
            // Read and process document.xml
            let mut content = String::new();
            file.read_to_string(&mut content)?;

            // Apply replacements
            content = apply_replacements(&content, data);

            // Write processed content
            let options = FileOptions::<()>::default();
            output_zip.start_file(&name, options)?;
            output_zip.write_all(content.as_bytes())?;
        } else {
            // Copy other files as-is
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            let options = FileOptions::<()>::default();
            output_zip.start_file(&name, options)?;
            output_zip.write_all(&buffer)?;
        }
    }

    output_zip.finish()?;
    Ok(())
}

fn apply_replacements(content: &str, data: &CsvData) -> String {
    let mut result = content.to_string();

    // Create replacements map
    let replacements = HashMap::from([
        ("{NOMBRE}", data.nombre.as_str()),
        ("{REVISTA}", data.revista.as_str()),
        ("{INSTITUCION}", data.institucion.as_str()),
        ("{CIUDAD}", data.ciudad.as_str()),
        ("{CORREO}", data.correo.as_str()),
    ]);

    // Apply replacements
    for (placeholder, value) in replacements {
        result = result.replace(placeholder, value);
    }

    result
}

pub fn extract_text_from_docx(docx_path: &str) -> Result<String> {
    let file_data = fs::read(docx_path)?;
    let reader = Cursor::new(&file_data);
    let mut zip = ZipArchive::new(reader)?;

    let mut document_xml = String::new();
    {
        let mut file = zip.by_name("word/document.xml")?;
        file.read_to_string(&mut document_xml)?;
    }

    extract_text_from_xml(&document_xml)
}

fn extract_text_from_xml(xml_content: &str) -> Result<String> {
    use xml::reader::{EventReader, XmlEvent};

    let mut text_content = String::new();
    let mut in_text = false;
    let reader = EventReader::from_str(xml_content);

    for event in reader {
        match event? {
            XmlEvent::StartElement { name, .. } => {
                if name.local_name == "t" {
                    in_text = true;
                }
            }
            XmlEvent::EndElement { name } => {
                if name.local_name == "t" {
                    in_text = false;
                }
                if name.local_name == "p" {
                    text_content.push('\n');
                }
            }
            XmlEvent::Characters(text) => {
                if in_text {
                    text_content.push_str(&text);
                }
            }
            _ => {}
        }
    }

    Ok(text_content.trim().to_string())
}
