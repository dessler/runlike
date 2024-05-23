// src/restart_policy.rs

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct RestartPolicy {
    pub name: String,
    pub maximum_retry_count: Option<u32>,
}

pub fn parse_restart_policy(json: &Value) -> RestartPolicy {
    let restart_policy = json["HostConfig"]["RestartPolicy"].as_object().unwrap();
    let name = restart_policy["Name"].as_str().unwrap_or("no");
    let maximum_retry_count = restart_policy["MaximumRetryCount"].as_u64().map(|n| n as u32);

    RestartPolicy {
        name: name.to_string(),
        maximum_retry_count,
    }
}

