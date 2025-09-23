use anyhow::Result;
use std::fs;
use std::io::{self, Read, Write as IoWrite};
use std::path::Path;
use tempfile::NamedTempFile;

pub fn read_file_or_stdin(path: Option<&str>) -> Result<String> {
    match path {
        Some("-") | None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }
        Some(path) => Ok(fs::read_to_string(path)?),
    }
}

pub fn write_output(path: &str, content: &str, write_in_place: bool) -> Result<()> {
    if write_in_place {
        // Atomic write using tempfile
        let file_path = Path::new(path);
        let parent = file_path.parent().unwrap_or(Path::new("."));

        let mut temp_file = NamedTempFile::new_in(parent)?;
        temp_file.write_all(content.as_bytes())?;
        temp_file.persist(path)?;
    } else {
        // Write to stdout
        print!("{}", content);
    }
    Ok(())
}
