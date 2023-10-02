use std::env;

use runas::Command;


pub fn reopen_as_admin() {
    let mut args: Vec<String> = env::args().collect();

    let current_exe = env::current_exe().unwrap();

    args.push(String::from("--is-admin"));

    Command::new(current_exe.display().to_string()).args(&args).status().unwrap();
}