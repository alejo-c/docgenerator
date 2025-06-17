use crate::csv_parser::CsvData;
use anyhow::Result;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

pub fn create_pdf_from_text(text: &str, output_path: &str, data: &CsvData) -> Result<()> {
    // Crear documento PDF
    let (doc, page1, layer1) = PdfDocument::new("Certificate", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Configurar fuente
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

    // Título
    current_layer.use_text("CERTIFICADO", 24.0, Mm(105.0), Mm(250.0), &font_bold);

    // Contenido principal
    let lines = format!(
        "Por la presente se certifica que:\n\n{}\n\nDe la institución: {}\nUbicada en: {}\n\nHa participado en la revista: {}",
        data.nombre, data.institucion, data.ciudad, data.revista
    );

    let mut y_position = 220.0;
    for line in lines.lines() {
        current_layer.use_text(line, 12.0, Mm(20.0), Mm(y_position), &font);
        y_position -= 8.0;
    }

    // Guardar PDF
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer)?;

    Ok(())
}
