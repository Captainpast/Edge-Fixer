mod admin;
mod open_browser;
mod log;

use std::fs;
use admin::reopen_as_admin;
use log::LOGGER;
use log::FileLogger;
use open_browser::decode_url;
use winreg::enums::*;
use winreg::RegKey;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut uninstall_arg = false;
    let mut is_debugger = false;
    let mut is_admin = false;
    let mut log_file = "";

    let mut log_file_path: String;

    for arg in args {
        if arg == "uninstall" {
            uninstall_arg = true;
        } else if arg == "--as-debugger" {
            is_debugger = true;
        } else if arg == "--is-admin" {
            is_admin = true;
        } else if arg == "--log-file" {
            let current_exe = env::current_exe().unwrap();
            log_file_path = format!("{}.log", current_exe.display());
            log_file = log_file_path.as_str();
        }
    }

    if !String::is_empty(&log_file.to_string()) {
        FileLogger::create(log_file);
    }

    if is_debugger {
        decode_url()
    } else {
        if is_admin {
            if uninstall_arg {
                uninstall() 
            } else {
                install()
            }
        } else {
            reopen_as_admin()
        }
    }
}

const ETC_HOSTS: &str = "C:/Windows/System32/drivers/etc/hosts";
const HOST_REDIRECT: &str = "0.0.0.0 www.bing.com # by edge fixer";

fn install() {
    unsafe { LOGGER.debug("install()") };

    println!("Hello, to Edge Fixer!\n");

    print!("edit host file...");

    let mut file_text = fs::read_to_string(ETC_HOSTS).unwrap();
    file_text = file_text.replace("\r", "");

    let splitted_text = file_text.split("\n");
    let mut new_text = String::from("");

    let mut added = false;
    for line in splitted_text {
        if line.contains("# by edge fixer") {
            new_text = format!("{}{}", new_text, HOST_REDIRECT);
            added = true;
        } else {
            new_text = format!("{}{}", new_text, line);
        }
        new_text = format!("{}\n", new_text);
    }

    if !added {
        new_text = format!("{}{}", new_text, HOST_REDIRECT);
    }

    fs::write(ETC_HOSTS, new_text).unwrap();

    println!("success\n");


    print!("edit regestry...");

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options").unwrap();
    let key = cur_ver.create_subkey("msedge.exe").unwrap().0;

    let current_exe = env::current_exe().unwrap();
    key.set_value("Debugger", &format!("{} --as-debugger", current_exe.display())).unwrap();

    println!("success\n");
}

fn uninstall() {
    unsafe { LOGGER.debug("uninstall()") };

    println!("Removing Edge Fixer\n");

    print!("edit host file...");

    let mut file_text = fs::read_to_string(ETC_HOSTS).unwrap();
    file_text = file_text.replace("\r", "");

    let splitted_text = file_text.split("\n");
    let mut new_text = String::from("");

    for line in splitted_text {
        if !line.contains("# by edge fixer") {
            new_text = format!("{}{}\n", new_text, line);
        }
    }

    fs::write(ETC_HOSTS, new_text).unwrap();
    
    println!("success\n");


    print!("edit regestry...");

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options").unwrap();
    cur_ver.delete_subkey_all("msedge.exe").unwrap();
    
    println!("success\n");
}
