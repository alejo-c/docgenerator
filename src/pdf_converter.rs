// Agregar esto a src/pdf_converter.rs

use anyhow::Result;
use std::path::Path;
use std::process::Command;

#[cfg(target_os = "windows")]
pub fn convert_to_pdf_windows(docx_dir: &str) -> Result<()> {
    println!("Convirtiendo archivos DOCX a PDF usando Microsoft Word...");

    // Crear script PowerShell temporal
    let ps_script = format!(
        r#"
$documents_path = "{}\*.docx"
$output_folder = "{}"

$word_app = New-Object -ComObject Word.Application
$word_app.Visible = $false

$converted = 0
Get-ChildItem -Path $documents_path | ForEach-Object {{
    Write-Host "Convirtiendo: $($_.Name)"
    $document = $word_app.Documents.Open($_.FullName)
    $pdf_filename = "$output_folder\" + $_.BaseName + ".pdf"
    $document.SaveAs([ref] $pdf_filename, [ref] 17)
    $document.Close()
    $converted++
}}

$word_app.Quit()
Write-Host "Convertidos $converted archivos a PDF"
"#,
        docx_dir.replace("\\", "\\\\"),
        docx_dir.replace("\\", "\\\\")
    );

    // Guardar script temporal
    let script_path = format!("{}/convert_temp.ps1", docx_dir);
    std::fs::write(&script_path, ps_script)?;

    // Ejecutar PowerShell
    let output = Command::new("powershell")
        .args(&["-ExecutionPolicy", "Bypass", "-File", &script_path])
        .output()?;

    // Eliminar script temporal
    let _ = std::fs::remove_file(&script_path);

    if output.status.success() {
        println!("Conversión completada exitosamente");
        Ok(())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Error en la conversión: {}", error)
    }
}

#[cfg(target_os = "linux")]
pub fn convert_to_pdf_linux(docx_dir: &str) -> Result<()> {
    println!("Convirtiendo archivos DOCX a PDF usando LibreOffice...");

    // Get all DOCX files in the directory
    let docx_files: Vec<_> = std::fs::read_dir(docx_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("docx"))
                .unwrap_or(false)
        })
        .map(|entry| entry.path())
        .collect();

    if docx_files.is_empty() {
        println!("No se encontraron archivos DOCX en el directorio");
        return Ok(());
    }

    println!(
        "Encontrados {} archivos DOCX para convertir",
        docx_files.len()
    );

    // Convert each file individually
    for docx_file in &docx_files {
        println!("Convirtiendo: {}", docx_file.display());

        let output = Command::new("soffice")
            .args(&[
                "--headless",
                "--convert-to",
                "pdf",
                "--outdir",
                docx_dir,
                docx_file.to_str().unwrap(),
            ])
            .output()?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error al convertir {}: {}", docx_file.display(), error);
        }
    }

    println!("Conversión completada");
    Ok(())
}

pub fn convert_to_pdf(docx_dir: &str) -> Result<()> {
    if !Path::new(docx_dir).exists() {
        anyhow::bail!("El directorio {} no existe", docx_dir);
    }

    #[cfg(target_os = "windows")]
    {
        convert_to_pdf_windows(docx_dir)?;
    }

    #[cfg(target_os = "linux")]
    {
        convert_to_pdf_linux(docx_dir)?;
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        anyhow::bail!("Conversión a PDF no soportada en este sistema operativo")
    }

    std::fs::remove_file(&docx_dir)?;
    Ok(())
}
