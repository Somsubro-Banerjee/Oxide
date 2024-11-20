use chrono::{Utc, DateTime};
use serde_json::{json, Value};
use std::{fs, path::Path};

const STORAGE_FILE: &str = "oxide_data.json";

pub fn persist_onboarding(data: &std::collections::HashMap<&str, String>, repo_url: &str) {
    let timestamp: DateTime<Utc> = Utc::now();
    let record = json!({
        "timestamp": timestamp.to_rfc3339(),
        "data": data,
        "repository_url": repo_url,
    });

    let path = Path::new(STORAGE_FILE);
    let mut existing_data = load_data();
    existing_data.push(record);

    fs::write(path, serde_json::to_string_pretty(&existing_data).unwrap())
        .expect("Failed to write onboarding data");
}


pub fn display_persisted_data() {
    let data = load_data();
    if data.is_empty() {
        println!("No onboarding records found.");
    } else {
        for record in data {
            println!("{}", serde_json::to_string_pretty(&record).unwrap());
        }
    }
}

fn load_data() -> Vec<Value> {
    let path = Path::new(STORAGE_FILE);
    if path.exists() {
        let content = fs::read_to_string(path).expect("Failed to read storage file");
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        vec![]
    }
}
