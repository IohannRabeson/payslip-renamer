mod date;
use anyhow::{Context, anyhow};
use clap::{Parser, Subcommand};
use pdf_oxide::PdfDocument;
use std::path::{Path, PathBuf};

use crate::date::Date;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

const DEFAULT_DATE_REGEX: &str = r"DATE PAYABLE: (\d{4})/(\d{2})/(\d{2})";

#[derive(Subcommand)]
enum Commands {
    /// Prints the text from a PDF.
    ///
    /// This command is useful to find the regex for the date extraction.
    Print {
        /// The PDF file to print
        #[arg()]
        file: PathBuf,
    },
    /// Extract the date from a PDF.
    Extract {
        /// The PDF file to print
        #[arg()]
        file: PathBuf,
        /// The regex to extract the date.
        #[arg(short, long, default_value = DEFAULT_DATE_REGEX)]
        regex: String,
    },
    /// Extract the date from the PDF then rename the PDF.
    Rename {
        /// The PDF file to print
        #[arg()]
        file: PathBuf,
        /// The regex to extract the date.
        #[arg(short, long, default_value = DEFAULT_DATE_REGEX)]
        regex: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Print { file } => {
            println!("{}", get_text(file)?);
        }
        Commands::Extract { file, regex } => {
            let text = get_text(file)?;
            let date = extract(&text, &regex)?;
            println!("{:04}-{:02}-{:02}", date.year, date.month, date.day);
        }
        Commands::Rename { file, regex } => {
            let text = get_text(&file)?;
            let date = extract(&text, &regex)?;
            let mut new_file_path = file.clone();
            new_file_path.set_file_name(format!("{:04}-{:02}-{:02}.pdf", date.year, date.month, date.day));
            std::fs::rename(&file, &new_file_path).with_context(|| {
                format!(
                    "Failed to rename file '{}' to '{}'",
                    file.display(),
                    new_file_path.display()
                )
            })?;
        }
    }
    Ok(())
}

fn get_text(pdf_file_path: impl AsRef<Path>) -> anyhow::Result<String> {
    let mut doc = PdfDocument::open(&pdf_file_path)?;

    Ok(doc.extract_all_text()?)
}

fn extract(text: &str, regex: &str) -> anyhow::Result<Date> {
    crate::date::parse_date(&text, &regex).ok_or(anyhow!("Failed to parse date"))
}