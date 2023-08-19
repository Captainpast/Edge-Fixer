use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;
use std::env;
use urlencoding::decode;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut is_debugger = false;
    for arg in args {
        if arg == "--as-debugger" {
            is_debugger = true;
        }
    }
    if is_debugger {
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

    // let mut host_file = File::options()
    //     .read(true)
    //     .write(true)
    //     .create(true)
    //     .open("../../log.txt")
    //     .unwrap();
    // host_file.seek(SeekFrom::End(0)).unwrap();
    // host_file.write(bing_regex_res.join(", ").as_bytes()).unwrap();
}

fn open_browser(url: String) {
    Command::new("C:/Program Files/Mozilla Firefox/firefox.exe")
        .arg(url)
        .spawn()
        .unwrap();
}

fn install() {
    println!("Hello, to Edge Fixer!\n");

    print!("edit host file...");

    let mut host_file = File::options()
        .read(true)
        .write(true)
        .open("C:/Windows/System32/drivers/etc/hosts")
        .unwrap();

    host_file.seek(SeekFrom::End(0)).unwrap();
    host_file.write(b"\n0.0.0.0 www.bing.com # by edge fixer").unwrap();

    println!("success\n");


    print!("edit regestry...");

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options").unwrap();
    let key = cur_ver.create_subkey("msedge.exe").unwrap().0;

    let current_exe = env::current_exe().unwrap();
    key.set_value("Debugger", &format!("{} --as-debugger", current_exe.display())).unwrap();

    println!("success\n");
}
