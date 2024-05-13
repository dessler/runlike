use std::process::{Command, Stdio};
use serde::Deserialize;

// 定义 Docker inspect 输出的整体结构
#[derive(Debug, serde::Deserialize)]
struct DockerInspect {
    #[serde(rename = "HostConfig")] // 将 JSON 中的字段名映射为结构体中的字段名
    host_config: HostConfig,
}

// 定义 HostConfig 结构体，表示 Docker inspect 中的 HostConfig 字段
#[derive(Debug, Deserialize)]
struct HostConfig {
    #[serde(rename = "Binds")] // 指定 JSON 字段名为 "Binds"
    binds: Vec<String>,
}

// 调用 Docker 命令获取容器信息并返回 JSON 字符串
fn get_docker_inspect(container_id: &str) -> Result<String, std::io::Error> {
    let output = Command::new("docker")
        .args(&["inspect", container_id])
        .stdout(Stdio::piped())
        .output()?; // 执行 Docker 命令并捕获输出

    let stdout = String::from_utf8_lossy(&output.stdout); // 将输出转换为字符串
    println!("Docker inspect output: {:?}", stdout); // 打印获取的 Docker inspect 输出

    if stdout.trim().is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Docker inspect output is empty",
        ));
    }

    let mut json_str = stdout.to_string();

    // 如果输出是一个数组，则取出第一个元素
    if json_str.starts_with("[") {
        json_str = json_str.trim_start_matches("[").to_string();
    }
    // 如果输出是一个数组，则取出最后一个元素
    if json_str.ends_with("]") {
        json_str = json_str.trim_end_matches("]").to_string();
    }

    Ok(json_str)
}
// 从 Docker inspect JSON 字符串中解析 DockerInspect 结构体
fn parse_docker_inspect_json(json_str: &str) -> Result<DockerInspect, serde_json::Error> {
    let docker_inspect: DockerInspect = serde_json::from_str(json_str)?; // 解析 JSON 字符串为 DockerInspect 结构体

    Ok(docker_inspect) // 返回解析后的结果
}


// 从 DockerInspect 结构体中获取挂载信息
fn get_mount_bindings(docker_inspect: &DockerInspect) -> Vec<String> {
    docker_inspect.host_config.binds.clone() // 返回 HostConfig 中的 binds 字段
}

// 外部调用接口，处理容器 ID 并返回挂载信息
pub fn inspect_container(container_id: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let json_str = get_docker_inspect(container_id)?; // 调用 get_docker_inspect 获取 Docker inspect 结果
    let docker_inspect = parse_docker_inspect_json(&json_str)?; // 解析 Docker inspect 结果
    let mount_bindings = get_mount_bindings(&docker_inspect); // 获取挂载信息
    Ok(mount_bindings)
}
