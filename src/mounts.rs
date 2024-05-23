// src/mounts.rs

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Mount {
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(rename = "Destination")]
    pub destination: String,
}

pub fn parse_mounts(json: &Value) -> Vec<Mount> {
    let mounts = json["Mounts"].as_array().unwrap();
    let mut result = Vec::new();

    for mount in mounts {
        let source = mount["Source"].as_str().unwrap().to_string();
        let destination = mount["Destination"].as_str().unwrap().to_string();
        result.push(Mount { source, destination });
    }

    result
}

