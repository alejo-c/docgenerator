use anyhow::Result;
use clap::Parser;
use std::path::Path;

mod csv_parser;
mod docx_processor;
mod pdf_converter;

use csv_parser::parse_csv;
use docx_processor::process_docx_template;
use pdf_converter::convert_to_pdf;

#[derive(Parser, Debug)]
#[command(name = "docgenerator")]
#[command(version = "1.0")]
#[command(about = "Procesa plantillas Word con datos CSV")]
struct Args {
    #[arg(short = 't', long = "template", default_value = "template.docx")]
    template: String,

    #[arg(short = 'c', long = "csv", default_value = "data.csv")]
    csv: String,

    #[arg(short = 'o', long = "output-dir", default_value = "output")]
    output_dir: String,

    #[arg(
        short = 'p',
        long = "pdf",
        help = "Convierte a PDF despues de generar DOCX"
    )]
    convert_to_pdf: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Validate template exists
    if !Path::new(&args.template).exists() {
        anyhow::bail!("Archivo plantilla '{}' no encontrado", args.template);
    }

    // Validate CSV exists
    if !Path::new(&args.csv).exists() {
        anyhow::bail!("Archivo de datos CSV '{}' no encontrado", args.csv);
    }

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&args.output_dir)?;

    // Read data from CSV
    let csv_data = parse_csv(&args.csv)?;
    println!("Se cargaron {} registros del archivo CSV", csv_data.len());

    // Process each record
    for (index, data) in csv_data.iter().enumerate() {
        println!("Procesando registro {}: {}", index + 1, data.nombre);

        // Create safe filename
        let safe_name = data
            .nombre
            .replace(" ", "_")
            .replace(",", "")
            .replace(".", "")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect::<String>();

        // Create DOCX with replacements
        let output_docx = format!("{}/{}.docx", args.output_dir, safe_name);
        process_docx_template(&args.template, data, &output_docx)?;

        println!("Se ha creado: {}", output_docx);
    }

    println!(
        "\n{} Documentos procesados satisfactoriamente",
        csv_data.len()
    );

    // Convert to PDF if requested
    if args.convert_to_pdf {
        println!("\nConvirtiendo a PDF...");
        match convert_to_pdf(&args.output_dir) {
            Ok(_) => println!("Conversión a PDF completada"),
            Err(e) => eprintln!("Error al convertir a PDF: {}", e),
        }
    } else {
        println!("Para convertir a PDF, ejecuta el programa con la opción --pdf");
    }

    Ok(())
}
