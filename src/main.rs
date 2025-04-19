use std::env;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

use dirs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        let lang = env::var("LANG").unwrap_or_else(|_| "en".to_string());
        if lang.starts_with("zh") {
            eprintln!("用法: work <命令> [参数...]");
            eprintln!("请在 $HOME/.config/work/<命令>.work 或 /etc/work/<命令>.work 中添加命令脚本");
            eprintln!("Work v1.0 by xiaozhao45");
        } else {
            eprintln!("Usage: work <command> [args...]");
            eprintln!("Add command script in $HOME/.config/work/<command>.work or /etc/work/<command>.work");
            eprintln!("Work v1.0 by xiaozhao45");
        }
        return;
    }
    let command = &args[1];
    let params = &args[2..];

    let public_path = PathBuf::from(format!("/etc{}work{}{}.work", MAIN_SEPARATOR, MAIN_SEPARATOR, command));
    let user_config_dir = dirs::config_dir()
        .unwrap_or_else(|| {
            let lang = env::var("LANG").unwrap_or_else(|_| "en".to_string());
            if lang.starts_with("zh") {
                eprintln!("无法确定用户配置目录");
            } else {
                eprintln!("Unable to determine user config directory");
            }
            std::process::exit(1);
        })
        .join("work");
    let user_path = user_config_dir.join(format!("{}.work", command));

    if public_path.exists() {
        execute_script(&public_path, params);
    } else if user_path.exists() {
        execute_script(&user_path, params);
    } else {
        let lang = env::var("LANG").unwrap_or_else(|_| "en".to_string());
        if lang.starts_with("zh") {
            eprintln!("命令 {} 未找到配置文件", command);
        } else {
            eprintln!("Command {} not found in config files", command);
        }
        std::process::exit(1);
    }
}

fn execute_script(path: &Path, params: &[String]) {
    let script_content = std::fs::read_to_string(path).expect("无法读取配置文件");
    
    let main_def_re = Regex::new(r"^\s*main\w+\s*\(").unwrap();
    if !script_content.lines().any(|line| main_def_re.is_match(line)) {
        let lang = env::var("LANG").unwrap_or_else(|_| "en".to_string());
        if lang.starts_with("zh") {
            eprintln!("错误：脚本缺少main函数定义");
        } else {
            eprintln!("Error: Script is missing main function definition");
        }
        return;
    }

    let lines: Vec<&str> = script_content.lines().collect();
    let last_lines: Vec<&str> = lines.iter().rev().take(10).cloned().collect();
    let main_call_re = Regex::new(r"^\s*main\w+\b").unwrap();
    let has_main_call = last_lines.iter().rev().any(|line| main_call_re.is_match(line.trim()));
    if !has_main_call {
        let lang = env::var("LANG").unwrap_or_else(|_| "en".to_string());
        if lang.starts_with("zh") {
            eprintln!("错误：脚本末尾未找到main调用");
        } else {
            eprintln!("Error: No main call found at the end of the script");
        }
        return;
    }

    let mut cmd = match std::env::consts::OS {
        "windows" => std::process::Command::new("cmd.exe"),
        _ => std::process::Command::new("bash"),
    };
    let arg0 = path.file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("script");
    cmd.arg("-c").arg(script_content).arg(arg0);
    cmd.args(params);
    let status = cmd.status().expect("执行脚本失败");
    if !status.success() {
        let lang = env::var("LANG").unwrap_or_else(|_| "en".to_string());
        if lang.starts_with("zh") {
            eprintln!("命令执行失败，退出码: {}", status.code().unwrap_or(-1));
        } else {
            eprintln!("Command execution failed, exit code: {}", status.code().unwrap_or(-1));
        }
        std::process::exit(status.code().unwrap_or(1));
    }
}