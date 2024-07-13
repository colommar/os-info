use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::{Command,Stdio};

fn get_system_architecture() -> String {
    let output = Command::new("arch")
        .output()
        .expect("failed to execute process");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn get_hostname() -> String {
    let output = Command::new("hostname")
        .output()
        .expect("failed to execute process");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn set_hostname() {
    println!("请输入新的主机名：");
    let mut new_hostname = String::new();
    io::stdin().read_line(&mut new_hostname).expect("Failed to read line");
    let new_hostname = new_hostname.trim();

    let status = Command::new("sudo")
        .arg("hostnamectl")
        .arg("set-hostname")
        .arg(new_hostname)
        .status()
        .expect("failed to execute process");

    if status.success() {
        println!("主机名已设置为 {}", new_hostname);
    } else {
        eprintln!("设置主机名失败。请确保您有足够的权限。");
    }
}

fn get_package_count() -> String {
    let output = if Command::new("which")
        .arg("dpkg")
        .output()
        .expect("failed to execute process")
        .status
        .success()
    {
        Command::new("dpkg-query")
            .arg("-f")
            .arg("${binary:Package}\n")
            .arg("-W")
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("rpm")
            .arg("-qa")
            .output()
            .expect("failed to execute process")
    };
    output.stdout.split(|&x| x == b'\n').count().to_string()
}

fn get_hardware_info() -> String {
    let output = Command::new("dmidecode")
        .output()
        .expect("failed to execute process");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn get_disk_usage() -> String {
    let output = Command::new("df")
        .arg("-h")
        .output()
        .expect("failed to execute process");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn get_ip_address() -> String {
    let output = Command::new("hostname")
        .arg("-I")
        .output()
        .expect("failed to execute process");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn get_system_version() -> String {
    let content = fs::read_to_string("/etc/os-release").expect("Unable to read file");
    content.lines()
        .find(|line| line.starts_with("PRETTY_NAME"))
        .map(|line| line.replace("PRETTY_NAME=", "").replace("\"", "").trim().to_string())
        .unwrap_or_else(|| "Unknown version".to_string())
}

fn change_admin_password() {
    println!("请输入新密码：");
    let mut new_password = String::new();
    io::stdin().read_line(&mut new_password).expect("Failed to read line");
    let new_password = new_password.trim();

    println!("请再次输入新密码：");
    let mut confirm_password = String::new();
    io::stdin().read_line(&mut confirm_password).expect("Failed to read line");
    let confirm_password = confirm_password.trim();

    if new_password != confirm_password {
        println!("两次输入的密码不一致。");
        return;
    }

    let mut child = Command::new("sudo")
        .arg("passwd")
        .arg("root")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to execute passwd command");

    {
        let child_stdin = child.stdin.as_mut().expect("Failed to open stdin");
        writeln!(child_stdin, "{}\n{}", new_password, new_password).expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to wait on passwd command");

    if output.status.success() {
        println!("密码修改成功");
    } else {
        println!("密码修改失败，请确保您有足够的权限。");
        io::stderr().write_all(&output.stderr).unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("参数说明：
        -p    系统IP
        -a    系统架构
        -v    系统版本
        -h    主机名
        -n    软件包总量
        -d    磁盘使用情况
        -H    获取硬件信息
        -s    设置主机名
        -c    修改管理员密码");
    } else {
        for arg in &args[1..] {
            match arg.as_str() {
                "-p" => println!("IP: {}", get_ip_address()),
                "-a" => println!("System arch: {}", get_system_architecture()),
                "-v" => println!("Version: {}", get_system_version()),
                "-h" => println!("Hostname: {}", get_hostname()),
                "-n" => println!("Number of packages: {}", get_package_count()),
                "-d" => println!("Disk usage:\n{}", get_disk_usage()),
                "-H" => println!("Hardware info:\n{}", get_hardware_info()),
                "-s" => set_hostname(),
                "-c" => change_admin_password(),
                _ => println!("未知参数: {}", arg),
            }
        }
    }
}
