use std::env;
use std::process::Command;

fn get_ip_address() -> String {
    let output = Command::new("hostname")
        .arg("-I")
        .output()
        .expect("failed to execute process");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn get_system_architecture() -> String {
    let output = Command::new("arch")
        .output()
        .expect("failed to execute process");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn get_system_version() -> String {
    let output = Command::new("lsb_release")
        .arg("-d")
        .output()
        .expect("failed to execute process");
    String::from_utf8_lossy(&output.stdout).replace("Description:", "").trim().to_string()
}

fn get_hostname() -> String {
    let output = Command::new("hostname")
        .output()
        .expect("failed to execute process");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn get_package_count() -> String {
    let output = Command::new("dpkg-query")
        .arg("-f")
        .arg("'${binary:Package}\n'")
        .arg("-W")
        .output()
        .expect("failed to execute process");
    output.stdout.split(|&x| x == b'\n').count().to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("参数说明：
        -p    系统IP
        -a    系统架构
        -v    系统版本
        -h    主机名
        -n    软件包总量");
    } else {
        for arg in &args[1..] {
            match arg.as_str() {
                "-p" => println!("IP: {}", get_ip_address()),
                "-a" => println!("System arch: {}", get_system_architecture()),
                "-v" => println!("Version: {}", get_system_version()),
                "-h" => println!("Hostname: {}", get_hostname()),
                "-n" => println!("Number of packages: {}", get_package_count()),
                _ => println!("未知参数: {}", arg),
            }
        }
    }
}
