use serde_json::Value;

#[derive(Debug)]
pub struct NetworkMode {
    pub mode: String,
}

pub fn parse_network_mode(json: &Value) -> NetworkMode {
    let mode = if let Some(networks) = json["NetworkSettings"]["Networks"].as_object() {
        if let Some(host_network) = networks.get("host") {
            if let Some(network_mode) = host_network["NetworkID"].as_str() {
                network_mode.to_string()
            } else {
                "bridge".to_string() // 如果没有找到 "host" 网络, 则默认为 "bridge"
            }
        } else {
            "bridge".to_string()
        }
    } else {
        "bridge".to_string()
    };

    NetworkMode { mode }
}
