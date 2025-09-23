use anyhow::Result;
use clap::{Parser, Subcommand};
use kugiri::{extract, insert, remove, trim, update, wrap};
use std::fs;

mod io;
use io::{read_file_or_stdin, write_output};

#[derive(Parser)]
#[command(name = "kugiri")]
#[command(about = "Marker-based block editing CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Insert a new section before or after a marker
    Insert {
        /// File to edit
        file: String,
        /// Section ID for the new section
        #[arg(long)]
        id: String,
        /// Content file (default: stdin, use '-' for stdin explicitly)
        #[arg(long)]
        body_file: Option<String>,
        /// Insert before this marker ID
        #[arg(long, conflicts_with = "after")]
        before: Option<String>,
        /// Insert after this marker ID
        #[arg(long, conflicts_with = "before")]
        after: Option<String>,
        /// Write changes in-place
        #[arg(short, long)]
        write: bool,
    },
    /// Update an existing section identified by --id
    Update {
        /// File to edit
        file: String,
        /// Section ID to update
        #[arg(long)]
        id: String,
        /// Content file (default: stdin, use '-' for stdin explicitly)
        #[arg(long)]
        body_file: Option<String>,
        /// Write changes in-place
        #[arg(short, long)]
        write: bool,
    },
    /// Remove a section and its markers
    Remove {
        /// File to edit
        file: String,
        /// Section ID to remove
        #[arg(long)]
        id: String,
        /// Write changes in-place
        #[arg(short, long)]
        write: bool,
    },
    /// Print inner content of a section (without markers)
    Extract {
        /// File to read
        file: String,
        /// Section ID to extract
        #[arg(long)]
        id: String,
    },
    /// Output the file with all marker lines removed
    Trim {
        /// File to read
        file: String,
    },
    /// Wrap content with KUGIRI markers
    Wrap {
        /// Section ID for the markers
        #[arg(long)]
        id: String,
        /// Content file (default: stdin, use '-' for stdin explicitly)
        #[arg(long)]
        body_file: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Insert {
            file,
            id,
            body_file,
            before,
            after,
            write,
        } => {
            let text = fs::read_to_string(&file)?;
            let body = read_file_or_stdin(body_file.as_deref())?;
            let result = insert(&text, &id, &body, before.as_deref(), after.as_deref())?;
            write_output(&file, &result, write)?;
        }
        Commands::Update {
            file,
            id,
            body_file,
            write,
        } => {
            let text = fs::read_to_string(&file)?;
            let body = read_file_or_stdin(body_file.as_deref())?;
            let result = update(&text, &id, &body)?;
            write_output(&file, &result, write)?;
        }
        Commands::Remove { file, id, write } => {
            let text = fs::read_to_string(&file)?;
            let result = remove(&text, &id)?;
            write_output(&file, &result, write)?;
        }
        Commands::Extract { file, id } => {
            let text = fs::read_to_string(&file)?;
            let result = extract(&text, &id)?;
            println!("{}", result);
        }
        Commands::Trim { file } => {
            let text = fs::read_to_string(&file)?;
            let result = trim(&text);
            println!("{}", result);
        }
        Commands::Wrap { id, body_file } => {
            let content = read_file_or_stdin(body_file.as_deref())?;
            let result = wrap(&content, &id);
            println!("{}", result);
        }
    }

    Ok(())
}
