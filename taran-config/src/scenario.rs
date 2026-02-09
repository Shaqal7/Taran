use crate::duration::HumanDuration;
use crate::error::{ConfigError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Root scenario configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub scenario: ScenarioInfo,
    pub load_profile: LoadProfile,
    #[serde(default)]
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioInfo {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum LoadProfile {
    Constant {
        users: usize,
        duration: HumanDuration,
        #[serde(default)]
        ramp_up: Option<HumanDuration>,
    },
    Ramp {
        from: usize,
        to: usize,
        duration: HumanDuration,
    },
    Stepped {
        steps: Vec<LoadStep>,
    },
    Spike {
        baseline: usize,
        peak: usize,
        spike_duration: HumanDuration,
        total_duration: HumanDuration,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadStep {
    pub users: usize,
    pub duration: HumanDuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    pub name: String,
    pub protocol: String,
    pub method: String,
    pub url: String,

    #[serde(default)]
    pub headers: HashMap<String, String>,

    #[serde(default)]
    pub body: Option<String>,

    #[serde(default)]
    pub assertions: Option<Assertions>,

    #[serde(default)]
    pub extract: Option<HashMap<String, Extractor>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assertions {
    #[serde(default)]
    pub status: Option<u16>,

    #[serde(default)]
    pub max_response_time: Option<HumanDuration>,

    #[serde(default)]
    pub body_contains: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extractor {
    pub from: String, // "body", "header", "status"
    #[serde(rename = "type")]
    pub extractor_type: String, // "jsonpath", "regex", "xpath"
    pub expr: String,
}

impl Scenario {
    /// Load scenario from TOML content
    pub fn from_toml(content: &str) -> Result<Self> {
        let content = toml::from_str(content).map_err(ConfigError::TomlParse)?;
        Ok(content)
    }

    /// Load scenario from TOML file
    pub fn from_file(path: &std::path::Path) -> Result<Self> {
        let content = std::fs::read_to_string(path).map_err(ConfigError::FileRead)?;
        Self::from_toml(&content)
    }

    /// Validate the scenario configuration
    pub fn validate(&self) -> Result<()> {
        if self.scenario.name.is_empty() {
            return Err(ConfigError::MissingField("scenario.name".to_string()));
        }

        if self.steps.is_empty() {
            return Err(ConfigError::InvalidScenario(
                "Scenario must have at least one step".to_string(),
            ));
        }

        for (i, step) in self.steps.iter().enumerate() {
            if step.name.is_empty() {
                return Err(ConfigError::MissingField(format!("steps[{}].name", i)));
            }
            if step.url.is_empty() {
                return Err(ConfigError::MissingField(format!("steps[{}].url", i)));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_scenario() {
        let toml = r#"
[scenario]
name = "Basic HTTP Test"

[load_profile]
type = "constant"
users = 100
duration = "60s"

[[steps]]
name = "GET Homepage"
protocol = "http"
method = "GET"
url = "https://example.com/"
"#;
        let scenario = Scenario::from_toml(toml).unwrap();
        assert_eq!(scenario.scenario.name, "Basic HTTP Test");
        assert_eq!(scenario.steps.len(), 1);
    }
}
