# os-info 开发日志
这是一个开源之夏项目的开发日志，已经被mr了。
地址：https://gitee.com/openeuler/osinfor

## 获取

关于osinfor项目子功能实现思路的分析
osinfor作为基础项目，主要需要用到两块内容，一是linux操作系统基础，二是rust语言基础。下面我会由浅入深逐条列出项目各个功能，并提供相应分析及实现思路，供大家参考，大家也可以根据各自对rust的了解程度，有选择性地实现，至少实现五个功能。

1. 获取系统架构;get_system_architecture
linux操作系统命令arch可以返回当前系统架构，rust可以用Command::new(cmd)方法来调用shell命令；
/proc/cpuinfo：包含有关 CPU 的详细信息。
/proc/meminfo：包含有关内存使用情况的信息。

2. 获取系统主机名；get_hostname
可以查阅相关资料，如rust获取系统主机名,可以使用功能1)的方法，也可以直接调用封装好的函数获取(可查到)；
/etc/hosts

3. 修改系统主机名；set_hostname
linux操作系统可以使用hostnamectl命令修改主机名，修改主机名后参数请自行查询；
先从/etc/hostname获取，然后再将其更改即可。
/etc/hostname
/etc/hosts

4. 获取系统软件包数量；get_package_count
　　linux操作系统分为u系和r系，ｕ系操作系统查看系统软件包使用dpkg命令，r系系统使用rpm命令查看，关于用什么参数请自行查找，参考功能１)rust调用shell命令的方法，最后输出软件包个数即可；
U:/var/lib/dpkg/status
R"/var/lib/rpm/Packages

5. 获取系统硬件信息(机器型号，bios信息等);get_hardware_info
linux系统查看硬件等信息命令dmidecode,dmidecode --help以了解命令功能，通过不同参数输出对应信息即可；
wsl暂时无法完成，考虑用虚拟机


6. 磁盘使用情况或者格式化磁盘；get_disk_usage
参考命令df,实现方法同1),有其他方式也可；格式化磁盘功能较复杂，由于对磁盘格式化，具有一定的风险，尽量完成其他功能后再根据rust了解程度去考虑实现此功能；
/proc/mounts 和 /sys/block/

7. 获取系统ip; get_ip_address
方法一：命令ifconfig输出网卡ip网关等信息,从返回的内容中进行字符串切割等操作，可获取系统ip(尽量配置单网卡)；方法二：调用相关rust库即可获取出系统ip,此库只支持单网卡获取，库名请自行查找及学习；
/proc/net/fib_trie 不确定，这个不太好提取

8. 获取系统版本；get_system_version
linux系统版本信息配置文件均不同，如/etc/os-version,/etc/os-release等，请先确定所用操作系统版本信息所在文件，再通过对文件读写，文件内容处理，字符串切割，拼接等操作，最终获取系统版本信息进行输出；
/etc/os-release

9. 修改管理员密码；change_admin_password
涉及人机交互，通过1)执行passwd命令，进入交互界面，rust方面可以使用read_line方法获取新密码；其他细节百度即可；
这个细节有点烦，直接用passwd。


最后项目效果类似如下即可：
 
```rust
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

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

fn set_hostname(new_hostname: &str) {
    Command::new("hostnamectl")
        .arg("set-hostname")
        .arg(new_hostname)
        .status()
        .expect("failed to execute process");
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

    let status = Command::new("sh")
        .arg("-c")
        .arg(format!("echo 'root:{}' | chpasswd", new_password))
        .status()
        .expect("Failed to change password");

    if status.success() {
        println!("密码修改成功");
    } else {
        println!("密码修改失败");
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
                "-s" => {
                    if args.len() > 2 {
                        set_hostname(&args[2]);
                    } else {
                        println!("请提供新的主机名");
                    }
                },
                "-c" => change_admin_password(),
                _ => println!("未知参数: {}", arg),
            }
        }
    }
}

```
