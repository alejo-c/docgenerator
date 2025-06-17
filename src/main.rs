use anyhow::Result;
use clap::Parser;
use std::path::Path;

mod csv_parser;
mod docx_processor;
mod pdf_generator;

use csv_parser::parse_csv;
use docx_processor::process_docx_template;
use pdf_generator::generate_pdf;

#[derive(Parser, Debug)]
#[command(name = "docgenerator")]
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

    // Validate template exists
    if !Path::new(&args.template).exists() {
        anyhow::bail!("Template file '{}' not found", args.template);
    }

    // Validate CSV exists
    if !Path::new(&args.csv).exists() {
        anyhow::bail!("CSV file '{}' not found", args.csv);
    }

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&args.output_dir)?;

    // Create temp directory for intermediate DOCX files
    let temp_dir = format!("{}/temp", args.output_dir);
    std::fs::create_dir_all(&temp_dir)?;

    // Read data from CSV
    let csv_data = parse_csv(&args.csv)?;
    println!("Loaded {} records from CSV", csv_data.len());

    // Process each record
    for (index, data) in csv_data.iter().enumerate() {
        println!("Processing record {}: {}", index + 1, data.nombre);

        // Create safe filename
        let safe_name = data
            .nombre
            .replace(" ", "_")
            .replace(",", "")
            .replace(".", "")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect::<String>();

        // Create temporary DOCX with replacements
        let temp_docx = format!("{}/temp_{}.docx", temp_dir, safe_name);
        process_docx_template(&args.template, data, &temp_docx)?;

        // Generate PDF from processed DOCX
        let output_pdf = format!("{}/certificado_{}.pdf", args.output_dir, safe_name);
        generate_pdf(&temp_docx, &output_pdf, data)?;

        // Clean up temporary file
        let _ = std::fs::remove_file(&temp_docx);

        println!("Created: {}", output_pdf);
    }

    // Clean up temp directory
    let _ = std::fs::remove_dir(&temp_dir);

    println!("\nSuccessfully processed {} certificates", csv_data.len());
    Ok(())
}
