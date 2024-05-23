use serde_json::Value;

pub fn parse_image_version(json: &Value) -> String {
    let image_version = json["Config"]["Image"].as_str().unwrap_or_default().to_string();
    image_version
}
