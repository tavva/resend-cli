// ABOUTME: Output formatting for CLI results.
// ABOUTME: Supports table and JSON output formats.

pub mod json;
pub mod table;

use anyhow::Result;
use serde::Serialize;
use std::fs;
use std::io::{self, Write};

use crate::types::{OutputFormat, Tabular};

/// Format and output data based on format setting
pub fn format_and_output<T: Serialize + Tabular>(
    data: &[T],
    format: OutputFormat,
    output_path: Option<&str>,
) -> Result<()> {
    let formatted = match format {
        OutputFormat::Table => table::format_table(data),
        OutputFormat::Json => json::format_json(data)?,
    };

    write_output(&formatted, output_path)
}

/// Format and output a single item
pub fn format_and_output_single<T: Serialize + Tabular>(
    data: &T,
    format: OutputFormat,
    output_path: Option<&str>,
) -> Result<()> {
    let formatted = match format {
        OutputFormat::Table => table::format_single(data),
        OutputFormat::Json => json::format_json_single(data)?,
    };

    write_output(&formatted, output_path)
}

/// Write output to file or stdout
fn write_output(content: &str, output_path: Option<&str>) -> Result<()> {
    match output_path {
        Some(path) => {
            fs::write(path, content)?;
            Ok(())
        }
        None => {
            let mut stdout = io::stdout().lock();
            writeln!(stdout, "{}", content)?;
            Ok(())
        }
    }
}

/// Output an error to stderr as JSON
pub fn output_error(error: &str, message: &str) {
    let error_json = serde_json::json!({
        "error": error,
        "message": message
    });
    eprintln!("{}", serde_json::to_string(&error_json).unwrap_or_default());
}
