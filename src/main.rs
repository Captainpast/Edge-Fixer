use std::fs;
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;
use std::env;
use urlencoding::decode;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut uninstall_arg = false;
    let mut is_debugger = false;
    for arg in args {
        if arg == "uninstall" {
            uninstall_arg = true;
        } else if arg == "--as-debugger" {
            is_debugger = true;
        }
    }

    if uninstall_arg {
        uninstall()
    } else if is_debugger {
        decode_url()
    } else {
        install()
    }
}

fn decode_url() {
    let args: Vec<String> = env::args().collect();
    let args_string = args.join(" ") + "\n";
    let decoded_url = decode(&args_string).unwrap();
    let url_regex = Regex::new(r"(https?://.+?)(\s|$)").unwrap();

    let mut res_url = "";
    for (_, [path, _]) in url_regex.captures_iter(&decoded_url).map(|c| c.extract()) {
        res_url = path;
    }

    let bing_regex = Regex::new(r"www\.bing\.com.*?\?.*?q=(.*?)(&|\s|$)").unwrap();
    let mut bing_search = "";
    //let mut bing_regex_res = vec![];
    for (_, [path, _]) in bing_regex.captures_iter(&res_url).map(|c| c.extract()) {
        //bing_regex_res.push(path);
        bing_search = path;
    }
    
    if bing_search != "" {
        open_browser(format!("https://duckduckgo.com/?q={}", bing_search).to_string());
    } else {
        open_browser(res_url.to_string());
    }
}

fn open_browser(url: String) {
    Command::new("C:/Program Files/Mozilla Firefox/firefox.exe")
        .arg(url)
        .spawn()
        .unwrap();
}


const ETC_HOSTS: &str = "C:/Windows/System32/drivers/etc/hosts";
const HOST_REDIRECT: &str = "0.0.0.0 www.bing.com # by edge fixer";

fn install() {
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
