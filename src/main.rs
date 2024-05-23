// src/main.rs

use std::env;
use serde_json::Value;
use tokio::process::Command as AsyncCommand;
use crate::mounts::{parse_mounts};
use crate::ports::{parse_ports};
use crate::image::parse_image_version; 
use crate::restart_policy::parse_restart_policy;
use crate::env_parser::parse_env_variables;
use network_mode::parse_network_mode;

mod mounts;
mod ports;
mod image;
mod restart_policy;
mod env_parser;
mod network_mode;



async fn inspect_container(container_name: &str) -> Result<(), &'static str> {
    let output = AsyncCommand::new("docker")
        .arg("inspect")
        .arg(container_name)
        .output()
        .await
        .expect("Failed to execute command");

    if output.status.success() {
        Ok(())
    } else {
        Err("Container does not exist or there was an error.")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <container_id>", args[0]);
        std::process::exit(1);
    }

    let container_id = &args[1];

    // Inspect the container and handle possible errors
    match inspect_container(container_id).await {
        Ok(_) => {
            // Continue with your program...
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    let output = AsyncCommand::new("docker")
        .arg("inspect")
        .arg(container_id)
        .output()
        .await?;

    let output_str = String::from_utf8(output.stdout)?;
    let v: Value = serde_json::from_str(&output_str)?;
    let json = &v[0];

    let mounts = parse_mounts(json);
    let ports = parse_ports(json);
    let image = parse_image_version(json); 
    let restart_policy = parse_restart_policy(json);
    let env_variables = parse_env_variables(json);
    let json: Value = serde_json::from_str(r#"{ "NetworkSettings": { "Networks": { "host": { "NetworkID": "host" } } } }"#).unwrap();
    let network_mode = parse_network_mode(&json); 
    let mut docker_run_command = format!("docker run");


    if network_mode.mode == "host" {
        docker_run_command.push_str(" --network host");
    }
    
    for mount in mounts {
        docker_run_command.push_str(&format!(" -v {}:{}", mount.source, mount.destination));
    }

    for port in ports {
        if let Some(host_port) = &port.host_port {
            docker_run_command.push_str(&format!(" -p {}:{}", host_port, port.container_port));
        }
    }

    if restart_policy.name != "no" {
        docker_run_command.push_str(&format!(" --restart={}{}", restart_policy.name, match restart_policy.maximum_retry_count {
            Some(count) => format!(":{}", count),
            None => "".to_string(),
        }));
    }

    for env_var in env_variables {
        docker_run_command.push_str(&format!(" -e {}={}", env_var.name, env_var.value));
    }

    docker_run_command.push_str(&format!(" {}", image));


    println!("{}", docker_run_command);

    Ok(())
}
