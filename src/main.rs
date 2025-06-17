use anyhow::Result;
use clap::Parser;

mod csv_parser;
mod docx_processor;
mod pdf_generator;

use csv_parser::parse_csv;
use docx_processor::process_docx_template;
use pdf_generator::generate_pdf;

#[derive(Parser, Debug)]
#[command(name = "certificate-cli")]
#[command(version = "1.0")]
#[command(about = "Processes Word templates with CSV data and converts to PDF")]
struct Args {
    #[arg(short = 't', long = "template", default_value = "template.docx")]
    template: String,

    #[arg(short = 'c', long = "csv", default_value = "data.csv")]
    csv: String,

    #[arg(short = 'o', long = "output-dir", default_value = "output")]
    output_dir: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Crear directorio de salida si no existe
    std::fs::create_dir_all(&args.output_dir)?;

    // Leer datos del CSV
    let csv_data = parse_csv(&args.csv)?;
    println!("Loaded {} records from CSV", csv_data.len());

    // Procesar cada registro
    for (index, data) in csv_data.iter().enumerate() {
        println!("Processing record {}: {}", index + 1, data.nombre);

        // Crear nombre de archivo único
        let safe_name = data.nombre.replace(" ", "_").replace(",", "");
        let output_path = format!("{}/certificate_{}.pdf", args.output_dir, safe_name);

        // Procesar plantilla DOCX
        let processed_text = process_docx_template(&args.template, data)?;

        // Crear PDF
        generate_pdf(&processed_text, &output_path, data)?;

        println!("Created: {}", output_path);
    }

    println!("Successfully processed {} certificates", csv_data.len());
    Ok(())
}
