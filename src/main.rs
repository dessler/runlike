use std::env;
use std::process;

mod inspect;

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查参数数量
    if args.len() != 2 {
        eprintln!("Usage: {} <container_id>", args[0]);
        process::exit(1);
    }

    // 获取容器 ID 参数
    let container_id = &args[1];

    // 检查容器 ID 的合法性
    if !is_valid_container_id(container_id) {
        eprintln!("Invalid container ID format");
        process::exit(1);
    }

    // 调用 inspect 模块处理容器 ID
    match inspect::inspect_container(container_id) {
        Ok(mount_bindings) => {
            println!("Mount Bindings: {:?}", mount_bindings);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}

// 检查容器 ID 是否符合预期格式
fn is_valid_container_id(container_id: &str) -> bool {
    // 在实际情况下，可以添加更多的容器 ID 格式检查逻辑
    // 这里只简单检查是否包含 12 位十六进制字符
    container_id.len() == 12 && container_id.chars().all(|c| c.is_ascii_hexdigit())
}