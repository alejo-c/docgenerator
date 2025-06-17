use crate::csv_parser::CsvData;
use anyhow::Result;
use pdf_writer::{Content, Finish, Name, Pdf, Rect, Ref, Str};
use std::fs;

pub fn generate_pdf(docx_path: &str, output_path: &str, data: &CsvData) -> Result<()> {
    // Extract text from processed DOCX
    let text = crate::docx_processor::extract_text_from_docx(docx_path)?;

    // Create PDF with extracted text
    create_pdf_from_text(&text, output_path, data)?;

    Ok(())
}

pub fn create_pdf_from_text(text: &str, output_path: &str, _data: &CsvData) -> Result<()> {
    // Create a new PDF
    let mut pdf = Pdf::new();

    // Define IDs
    let catalog_id = Ref::new(1);
    let page_tree_id = Ref::new(2);
    let page_id = Ref::new(3);
    let font_id = Ref::new(4);
    let content_id = Ref::new(5);

    // Write the document catalog
    pdf.catalog(catalog_id).pages(page_tree_id);

    // Write the page tree
    pdf.pages(page_tree_id).kids([page_id]).count(1);

    // A4 page size
    let page_width = 595.0;
    let page_height = 842.0;

    // Start writing a page
    let mut page = pdf.page(page_id);
    page.media_box(Rect::new(0.0, 0.0, page_width, page_height));
    page.parent(page_tree_id);
    page.contents(content_id);

    // Add resources with font
    let mut resources = page.resources();
    let mut fonts = resources.fonts();
    fonts.pair(Name(b"F1"), font_id);
    fonts.finish();
    resources.finish();
    page.finish();

    // Add Helvetica font
    pdf.type1_font(font_id).base_font(Name(b"Helvetica"));

    // Create content stream
    let mut content = Content::new();

    // Set initial position (top of page with margin)
    let mut y_position = page_height - 50.0;
    let left_margin = 50.0;
    let right_margin = page_width - 50.0;
    let line_height = 14.0;
    let font_size = 12.0;

    // Set font
    content.set_font(Name(b"F1"), font_size);

    // Split text into lines
    let lines = text.split('\n').collect::<Vec<&str>>();

    for line in lines.iter() {
        // Skip empty lines but still move down
        if line.trim().is_empty() {
            y_position -= line_height;
            continue;
        }

        // Simple word wrapping
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let mut current_line = String::new();
        let max_chars_per_line = ((right_margin - left_margin) / (font_size * 0.5)) as usize;

        let mut lines_to_draw = Vec::new();

        for word in words {
            let test_line = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };

            if test_line.len() > max_chars_per_line && !current_line.is_empty() {
                // Save current line
                lines_to_draw.push(current_line);
                current_line = word.to_string();
            } else {
                current_line = test_line;
            }
        }

        // Add remaining text
        if !current_line.is_empty() {
            lines_to_draw.push(current_line);
        }

        // Draw all wrapped lines
        for wrapped_line in lines_to_draw {
            if y_position < 50.0 {
                break; // Would need a new page
            }

            // Position text and show it
            content.begin_text();
            content.move_to(left_margin, y_position);
            content.show(Str(wrapped_line.as_bytes()));
            content.end_text();

            y_position -= line_height;
        }
    }

    // Write the content stream
    let content_data = content.finish();
    pdf.stream(content_id, &content_data);

    // Write the PDF to file
    fs::write(output_path, pdf.finish())?;

    Ok(())
}
