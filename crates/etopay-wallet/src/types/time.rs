use chrono::{DateTime, TimeZone, Utc};

pub fn parse_date_or_default(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc.timestamp_opt(0, 0).unwrap()) // 1970-01-01T00:00:00Z
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_parse_valid_date() {
        // Given
        let valid_str = "2024-06-01T12:00:00Z";
        let expected: DateTime<Utc> = valid_str.parse().unwrap();

        // When
        let parsed = parse_date_or_default(valid_str);

        // Then
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_invalid_date_into_default() {
        // Given
        let invalid = "not-a-date";

        // When
        let fallback = parse_date_or_default(invalid); // 1970-01-01T00:00:00Z

        // Then
        assert_eq!(fallback, Utc.timestamp_opt(0, 0).unwrap());
    }
}
