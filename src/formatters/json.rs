// ABOUTME: JSON formatting for structured output.
// ABOUTME: Used when --json flag is provided.

use anyhow::Result;
use serde::Serialize;

/// Format a list of items as JSON
pub fn format_json<T: Serialize>(items: &[T]) -> Result<String> {
    Ok(serde_json::to_string_pretty(items)?)
}

/// Format a single item as JSON
pub fn format_json_single<T: Serialize>(item: &T) -> Result<String> {
    Ok(serde_json::to_string_pretty(item)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Email;

    #[test]
    fn test_format_json_empty() {
        let emails: Vec<Email> = vec![];
        let output = format_json(&emails).unwrap();
        assert_eq!(output, "[]");
    }

    #[test]
    fn test_format_json_with_items() {
        let emails = vec![Email {
            id: "email-123".to_string(),
            from: Some("from@example.com".to_string()),
            to: Some(vec!["to@example.com".to_string()]),
            subject: Some("Test".to_string()),
            created_at: None,
            last_event: None,
        }];
        let output = format_json(&emails).unwrap();
        assert!(output.contains("email-123"));
        assert!(output.contains("from@example.com"));
    }
}
