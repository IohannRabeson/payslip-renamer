mod parser;
use anyhow::Context;
use pdf_oxide::PdfDocument;
use std::path::PathBuf;
fn main() -> anyhow::Result<()> {
    let pdf_file_path = match std::env::args().skip(1).next() {
        Some(arg) => PathBuf::from(arg),
        None => {
            eprintln!("Usage: {} <pdf_file_path>", env!("CARGO_PKG_NAME"));
            return Err(anyhow::anyhow!("No PDF file path specified"));
        }
    };
    let mut doc = PdfDocument::open(&pdf_file_path)?;
    let text = doc.extract_all_text()?;
    let date = crate::parser::parse_date(&text).ok_or(anyhow::anyhow!("Failed to parse the date"))?;
    let mut new_file_path = pdf_file_path.clone();
    new_file_path.set_file_name(format!("{:04}-{:02}-{:02}.pdf", date.year, date.month, date.day));
    std::fs::rename(&pdf_file_path, &new_file_path).with_context(|| {
        format!(
            "Failed to rename file '{}' to '{}'",
            pdf_file_path.display(),
            new_file_path.display()
        )
    })?;
    Ok(())
}
