mod inspect;

fn main() {
    // 假设容器ID作为命令行参数传递给程序
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <container_id>", args[0]);
        std::process::exit(1);
    }
    let container_id = &args[1];

    // 调用 inspect 模块的函数处理容器
    match inspect::runlike(container_id) {
        Ok(cmd) => println!("{}", cmd),
        Err(e) => eprintln!("Error: {}", e),
    }
}