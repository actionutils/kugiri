use anyhow::Result;
use clap::{Parser, Subcommand};
use kugiri::{extract, insert, remove, trim, update, upsert, wrap};
use std::fs;
use std::io::Read;

mod io;
use io::{read_file_or_stdin, write_output};

const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), env!("GIT_VERSION_SUFFIX"));

#[derive(Parser)]
#[command(name = "kugiri")]
#[command(about = "Marker-based block editing CLI", long_about = None)]
#[command(version = VERSION)]
#[command(after_help = "MARKERS:
  Kugiri uses HTML comment markers to define editable sections:

  <!-- KUGIRI-BEGIN: {id} -->  Start of a section
  <!-- KUGIRI-END: {id} -->    End of a section
  <!-- KUGIRI-INSERT: {id} --> Insertion point for new sections

  Where {id} is a unique identifier for the section.

EXAMPLES:
  # Extract content from a section
  kugiri extract README.md --id installation

  # Update a section with new content
  echo \"New content\" | kugiri update README.md --id docs --write

  # Insert a new section after an existing one
  echo \"Content\" | kugiri insert file.md --id new-section --after existing-id --write

  # Update existing section or create new one if not found
  echo \"Content\" | kugiri upsert file.md --id section --after other-id --write

  # Remove all marker lines from output
  kugiri trim file.md > clean.md

  # Wrap content with markers
  echo \"Content to wrap\" | kugiri wrap --id section-name")]
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
    /// Update existing section or insert if not found
    Upsert {
        /// File to edit
        file: String,
        /// Section ID for the section
        #[arg(long)]
        id: String,
        /// Content file (default: stdin, use '-' for stdin explicitly)
        #[arg(long)]
        body_file: Option<String>,
        /// Insert before this marker ID (for new sections)
        #[arg(long, conflicts_with = "after")]
        before: Option<String>,
        /// Insert after this marker ID (for new sections)
        #[arg(long, conflicts_with = "before")]
        after: Option<String>,
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
        /// File to read (use '-' for stdin)
        #[arg(default_value = "-")]
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
        Commands::Upsert {
            file,
            id,
            body_file,
            before,
            after,
            write,
        } => {
            let text = fs::read_to_string(&file)?;
            let body = read_file_or_stdin(body_file.as_deref())?;
            let result = upsert(&text, &id, &body, before.as_deref(), after.as_deref())?;
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
            println!("{result}");
        }
        Commands::Trim { file } => {
            let text = if file == "-" {
                let mut buffer = String::new();
                std::io::stdin().read_to_string(&mut buffer)?;
                buffer
            } else {
                fs::read_to_string(&file)?
            };
            let result = trim(&text);
            println!("{result}");
        }
        Commands::Wrap { id, body_file } => {
            let content = read_file_or_stdin(body_file.as_deref())?;
            let result = wrap(&content, &id);
            println!("{result}");
        }
    }

    Ok(())
}
