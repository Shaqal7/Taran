use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Parse duration from human-readable format like "60s", "500ms", "2m"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct HumanDuration(pub Duration);

impl HumanDuration {
    pub fn as_duration(&self) -> Duration {
        self.0
    }
}

impl TryFrom<String> for HumanDuration {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        parse_duration(&s).map(HumanDuration)
    }
}

impl From<HumanDuration> for String {
    fn from(d: HumanDuration) -> Self {
        format_duration(d.0)
    }
}

fn parse_duration(s: &str) -> Result<Duration, String> {
    let s = s.trim();

    if s.is_empty() {
        return Err("Empty duration string".to_string());
    }

    // Find where the number ends and unit begins
    let split_pos = s
        .chars()
        .position(|c| !c.is_numeric() && c != '.')
        .ok_or_else(|| format!("Missing time unit in '{}'", s))?;

    let (num_str, unit) = s.split_at(split_pos);
    let value: f64 = num_str.parse().map_err(|_| format!("Invalid number: '{}'", num_str))?;

    let duration = match unit.trim() {
        "ns" => Duration::from_nanos(value as u64),
        "us" | "µs" => Duration::from_micros(value as u64),
        "ms" => Duration::from_secs_f64(value / 1000.0),
        "s" => Duration::from_secs_f64(value),
        "m" => Duration::from_secs_f64(value * 60.0),
        "h" => Duration::from_secs_f64(value * 3600.0),
        _ => return Err(format!("Unknown time unit: '{}'", unit)),
    };

    Ok(duration)
}

fn format_duration(d: Duration) -> String {
    let secs = d.as_secs();
    let millis = d.subsec_millis();

    if secs == 0 {
        format!("{}ms", millis)
    } else if secs % 60 == 0 && millis == 0 {
        format!("{}m", secs / 60)
    } else if millis == 0 {
        format!("{}s", secs)
    } else {
        format!("{}.{:03}s", secs, millis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("100ms").unwrap(), Duration::from_millis(100));
        assert_eq!(parse_duration("5s").unwrap(), Duration::from_secs(5));
        assert_eq!(parse_duration("2m").unwrap(), Duration::from_secs(120));
        assert_eq!(parse_duration("1h").unwrap(), Duration::from_secs(3600));
    }
}
