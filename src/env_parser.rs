use serde_derive::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct EnvVariable {
    pub name: String,
    pub value: String,
}

pub fn parse_env_variables(json: &Value) -> Vec<EnvVariable> {
    // 从 JSON 数据中提取 "Config.Env" 数组
    json["Config"]["Env"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        // 遍历每个环境变量字符串
        .filter_map(|env_var| {
            env_var.as_str().and_then(|s| {
                // 使用 splitn 分割环境变量字符串,得到名称和值
                let mut parts = s.splitn(2, '=');
                parts.next().and_then(|name| {
                    // 检查环境变量名称是否全部大写
                    if name.to_uppercase() != name {
                        // 如果不是全部大写,则认为是自定义环境变量,返回 EnvVariable 实例
                        parts.next().map(|value| EnvVariable {
                            name: name.to_string(),
                            value: value.to_string(),
                        })
                    } else {
                        // 如果全部大写,则认为是 Docker 自带的环境变量,过滤掉
                        None
                    }
                })
            })
        })
        .collect()
}
