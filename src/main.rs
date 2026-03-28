mod date;
use anyhow::{Context, anyhow};
use clap::{Parser, Subcommand};
use pdf_oxide::PdfDocument;
use crate::date::Date;
use std::{path::{Path, PathBuf}, time::Duration};
use notify::RecursiveMode;
use notify_debouncer_full::new_debouncer;

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
    /// Monitor a folder for PDF to rename.
    Monitor {
        #[arg()]
        directory: PathBuf,
        /// The regex to extract the date.
        #[arg(short, long, default_value = DEFAULT_DATE_REGEX)]
        regex: String,
    }
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
            rename(file, &regex)?;
        }
        Commands::Monitor { directory, regex } => {
            monitor(directory, |file_path: &Path| {
                println!("File added: {}", file_path.display());
                match rename(file_path, &regex) {
                    Ok(()) => {},
                    Err(error) => {
                        eprintln!("Failed to rename: {error}");
                    },
                }
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

fn rename(pdf_file_path: impl AsRef<Path>, regex: &str) -> anyhow::Result<()> {
    let pdf_file_path = pdf_file_path.as_ref();
    let text = get_text(pdf_file_path)?;
    let date = extract(&text, &regex)?;
    let mut new_file_path = pdf_file_path.to_path_buf();
    new_file_path.set_file_name(format!("{:04}-{:02}-{:02}.pdf", date.year, date.month, date.day));
    if pdf_file_path != new_file_path {
        std::fs::rename(&pdf_file_path, &new_file_path).with_context(|| {
            format!(
                "Failed to rename file '{}' to '{}'",
                pdf_file_path.display(),
                new_file_path.display()
            )
        })?;
    }
    Ok(())
}

// This example in the notify crate was useful: https://github.com/notify-rs/notify/blob/2da899c48d326f3e5650ef2874ba33bee84c4108/notify/src/lib.rs#L105
fn monitor(directory: impl AsRef<Path>, on_file_added: impl Fn(&Path)) -> anyhow::Result<()> {
    let mut signals = signal_hook::iterator::Signals::new([
            signal_hook::consts::SIGTERM,
            signal_hook::consts::SIGINT,
        ])
        .unwrap();
    let (signal_sender, signal_receiver) = crossbeam_channel::bounded(1);
    let signal_thread_handle = std::thread::spawn(move || {
        println!("Signals thread starts");
        for _ in &mut signals {
            let _ = signal_sender.send(());
            break;
        }
        println!("Signals thread shutdowns");
    });
    let directory = directory.as_ref();
    let (event_sender, event_receiver) = crossbeam_channel::bounded(1024);
    let mut debouncer = new_debouncer(Duration::from_secs(2), None, event_sender)?;
    
    debouncer.watch(directory, RecursiveMode::Recursive)?;

    println!("Monitoring started");
    loop {
        crossbeam_channel::select! {
            recv(signal_receiver) -> _signal => break,
            recv(event_receiver) -> event => {
                if let Ok(Ok(debounced_events)) = event {
                    if debounced_events.iter().any(|event|{
                        matches!(event.event.kind, notify::event::EventKind::Create(notify::event::CreateKind::File))
                        || matches!(event.event.kind, notify::event::EventKind::Modify(_))
                    }) {
                        if let Some(event) = debounced_events.first() {
                            for path in &event.paths {
                                on_file_added(&path);
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Monitoring stopped");
    
    signal_thread_handle.join().unwrap();
    Ok(())
}
