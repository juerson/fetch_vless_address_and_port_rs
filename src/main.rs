use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn main() {
    if let Ok(file) = File::open("vless.txt") {
        // 提取vless链接中的地址（IPv4地址或域名）
        let fetch_ipaddr: Vec<String> = io::BufReader::new(file)
            .lines()
            .filter_map(|line| line.ok())
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let start = line.find('@').unwrap_or(0);
                let end = line[start + 1..].find("?").unwrap_or(line.len());
                let ipaddr = line[start + 1..start + 1 + end].to_owned();
                ipaddr
            })
            .collect();

        let unique_result: Vec<String> = fetch_ipaddr.into_iter().unique().collect();

        let (ipv4_addresses, other_items): (Vec<&str>, Vec<&str>) = unique_result
            .iter()
            .map(|s| s.as_str())
            .partition(|&item| is_ipv4_address(item));
        let mut sorted_ipv4_addresses = ipv4_addresses.to_vec();
        sorted_ipv4_addresses.sort_by(|a, b| {
            let a_parts: Vec<u8> = a
                .rsplitn(2, ":")
                .nth(1)
                .unwrap_or("")
                .split('.')
                .map(|s| s.parse().unwrap())
                .collect();
            let b_parts: Vec<u8> = b
                .rsplitn(2, ":")
                .nth(1)
                .unwrap_or("")
                .split('.')
                .map(|s| s.parse().unwrap())
                .collect();

            a_parts.cmp(&b_parts)
        });
        let mut result: Vec<&str> = Vec::new();
        result.extend(sorted_ipv4_addresses);
        result.extend(other_items);
        if let Ok(mut output_file) = File::create("ip.txt") {
            /*
                这里的代码含义：根据用户输入的不同选择，选择不同的写入文件的方式
            */
            println!("是否将从vless链接中提取到的Port端口也写入文件中？您有下面3种情况可选择：");
            let repeated = "-".repeat(75);
            println!("{}", repeated);
            println!("【0】或【】：只写入IP或域名，写入文件的格式是：192.168.1.0");
            println!("【1】：将IP、域名与Port端口都写入文件中，写入文件的格式是：192.168.1.0 8080");
            println!("_：输入其它字符时，写入文件的格式是：192.168.1.0:8080");
            println!("{}", repeated);
            print!("输入您的选择(注意：您只有一次输入的机会)：");
            io::stdout().flush().expect("Failed to flush stdout");
            let mut write_mode = String::new();
            io::stdin()
                .read_line(&mut write_mode)
                .expect("Failed to read line");
            for ipaddr in &result {
                let mut ip_with_port = ipaddr.rsplitn(2, ":");
                let port = ip_with_port.next().unwrap_or("");
                let ip = ip_with_port.next().unwrap_or("");
                match write_mode.trim() {
                    "" | "0" => {
                        // 只写入IP或域名
                        writeln!(output_file, "{}", ip).expect("Failed to write to file");
                    }
                    "1" => {
                        // 以"IP PORT"格式写入
                        let s = format!("{}\t{}", ip, port);
                        writeln!(output_file, "{}", s).expect("Failed to write to file");
                    }

                    _ => {
                        // 原字符串写入（比如：192.168.1.0:80）
                        writeln!(output_file, "{}", ipaddr).expect("Failed to write to file");
                    }
                }
            }
        } else {
            eprintln!("Failed to create or write to ip.txt");
        }
    } else {
        eprintln!("未找到存放vless节点的vless.txt文件!");
        wait_for_enter();
        std::process::exit(1);
    }
    print!("程序执行完毕！");
    io::stdout().flush().expect("无法刷新标准输出缓冲区");
    wait_for_enter();
}

fn is_ipv4_address(s: &str) -> bool {
    // 提取字符串中的IP或域名
    let addr = s.rsplitn(2, ":").nth(1).unwrap_or("");
    // 使用正则表达式匹配IPv4地址的模式
    let re = Regex::new(r"^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();
    // 使用正则表达式进行匹配
    re.is_match(addr)
}

fn wait_for_enter() {
    print!("按Enter键退出程序...");
    io::stdout().flush().expect("无法刷新标准输出缓冲区");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("无法读取行");
    // 移除输入中的换行符
    let _ = input.trim();
    io::stdout().flush().expect("无法刷新缓冲区");
}