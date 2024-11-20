use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub github_token: String,
    pub org: String,
}

impl Config {
    pub fn load() -> Self {
        // Mock loading config
        Config {
            github_token: "your-github-token".to_string(),
            org: "your-org".to_string(),
        }
    }
}
