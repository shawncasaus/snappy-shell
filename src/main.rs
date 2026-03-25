mod path_utils;
use path_utils::cd;
use path_utils::find_executables;
use std::collections::HashMap;
use std::env::current_dir;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() {
    let type_map = HashMap::from([
        ("type", "used to identify if a type is availble in shell."),
        ("exit", "exits shell."),
        ("pwd", "prints current working directory."),
        ("cd", "changes current working directory."),
        ("echo", "outputs to shell"),
    ]);

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        command = command.trim().to_string();
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        let cmd = parts[0];
        let args = &parts[1..];

        if cmd == "exit" {
            break;
        } else if cmd == "pwd" {
            if let Ok(path) = current_dir() {
                println!("{}", path.display());
            } else {
                println!("Error getting current directory");
            }
        } else if cmd == "cd" {
            if args.is_empty() {
                println!("{}: no path provided.", cmd);
            } else {
                let path = &args[0];
                cd(path);
            }
        } else if cmd == "type" {
            if args.is_empty() {
                println!("{}: no arguments provided", cmd);
            } else {
                let input_type = &args[0];
                if type_map.contains_key(input_type) {
                    println!("{} is a shell builtin", input_type);
                } else if let Some(path) = find_executables(input_type) {
                    println!("{} is {}", input_type, path);
                } else {
                    println!("{}: not found", input_type);
                }
            }
        } else if cmd == "echo" {
            println!("{}", args.join(" "));
        } else {
            if let Some(path) = find_executables(cmd) {
                Command::new(&path).arg0(cmd).args(args).status().unwrap();
            } else {
                println!("{}: command not found", cmd);
            }
        }
    }
}
