// src/ports.rs

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Port {
    #[serde(rename = "HostPort")]
    pub host_port: Option<String>,
    #[serde(rename = "ContainerPort")]
    pub container_port: String,
}

pub fn parse_ports(json: &Value) -> Vec<Port> {
    let ports = json["NetworkSettings"]["Ports"].as_object().unwrap();
    let mut result = Vec::new();

    for (key, value) in ports {
        let container_port = key.split('/').next().unwrap().to_string();
        let host_port = value[0]["HostPort"].as_str().map(|s| s.to_string());
        result.push(Port { host_port, container_port });
    }

    result
}

