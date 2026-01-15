// ABOUTME: Table formatting using the tabled crate.
// ABOUTME: Renders data as human-readable tables.

use crate::types::Tabular;

/// Format a list of items as a table
pub fn format_table<T: Tabular>(items: &[T]) -> String {
    if items.is_empty() {
        return "No results found.".to_string();
    }

    let headers = T::headers();
    let mut rows: Vec<Vec<String>> = vec![headers.iter().map(|s| s.to_string()).collect()];

    for item in items {
        rows.push(item.row());
    }

    format_rows(&rows)
}

/// Format a single item as key-value pairs
pub fn format_single<T: Tabular>(item: &T) -> String {
    let headers = T::headers();
    let values = item.row();

    let mut output = String::new();
    for (header, value) in headers.iter().zip(values.iter()) {
        output.push_str(&format!("{}: {}\n", header, value));
    }
    output
}

/// Format rows into a table string
fn format_rows(rows: &[Vec<String>]) -> String {
    if rows.is_empty() {
        return String::new();
    }

    // Calculate column widths
    let num_cols = rows[0].len();
    let mut widths: Vec<usize> = vec![0; num_cols];

    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if i < num_cols {
                widths[i] = widths[i].max(cell.len());
            }
        }
    }

    // Build output
    let mut output = String::new();

    for (row_idx, row) in rows.iter().enumerate() {
        let line: Vec<String> = row
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                if i < widths.len() {
                    format!("{:<width$}", cell, width = widths[i])
                } else {
                    cell.clone()
                }
            })
            .collect();

        output.push_str(&line.join("  "));
        output.push('\n');

        // Add separator after header
        if row_idx == 0 {
            let separator: Vec<String> = widths.iter().map(|w| "-".repeat(*w)).collect();
            output.push_str(&separator.join("  "));
            output.push('\n');
        }
    }

    output.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Email;

    #[test]
    fn test_format_table_empty() {
        let emails: Vec<Email> = vec![];
        let output = format_table(&emails);
        assert_eq!(output, "No results found.");
    }

    #[test]
    fn test_format_table_with_items() {
        let emails = vec![Email {
            id: "email-123".to_string(),
            from: Some("from@example.com".to_string()),
            to: Some(vec!["to@example.com".to_string()]),
            subject: Some("Test Subject".to_string()),
            created_at: Some("2025-01-15".to_string()),
            last_event: Some("delivered".to_string()),
        }];
        let output = format_table(&emails);
        assert!(output.contains("email-123"));
        assert!(output.contains("Test Subject"));
        assert!(output.contains("delivered"));
    }
}
