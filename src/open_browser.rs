
use std::env;
use urlencoding::decode;
use regex::Regex;
use std::process::Command;

use crate::log::LOGGER;

pub fn decode_url() {
    unsafe { LOGGER.debug("decode_url()") };

    let args: Vec<String> = env::args().collect();
    let args_string = args.join(" ") + "\n";
    let decoded_url = decode(&args_string).unwrap();
    unsafe { LOGGER.info(format!("decoded_url: {}", decoded_url).as_str()) };

    let unwanted_regex = Regex::new(r"--out-pipe-name").unwrap();
    let unwanted_params = unwanted_regex.is_match(&decoded_url);

    if !unwanted_params {
        let url_regex = Regex::new(r"(https?://.+?)(\s|$)").unwrap();
    
        let mut res_url = "";
        for (_, [path, _]) in url_regex.captures_iter(&decoded_url).map(|c| c.extract()) {
            res_url = path;
        }
    
        let bing_regex = Regex::new(r"www\.bing\.com.*?\?.*?q=(.*?)(&|\s|$)").unwrap();
        let mut bing_search = "";
        for (_, [path, _]) in bing_regex.captures_iter(&res_url).map(|c| c.extract()) {
            bing_search = path;
        }
        
        if bing_search != "" {
            open_browser(format!("https://duckduckgo.com/?q={}", bing_search).to_string());
        } else {
            open_browser(res_url.to_string());
        }
    }
}

fn open_browser(url: String) {
    Command::new("C:/Program Files/Mozilla Firefox/firefox.exe")
        .arg(url)
        .spawn()
        .unwrap();
}