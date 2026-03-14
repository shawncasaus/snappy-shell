mod path_utils;
use path_utils::find_executables;
use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let type_map = HashMap::from([
        ("type", "used to identify if a type is availble in shell."),
        ("exit", "exits shell."),
        ("echo", "outputs to shell"),
    ]);

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        command = command.trim().to_string();
        if command == "exit" {
            break;
        } else if command.starts_with("type") {
            let input_type = &command[5..];
            if type_map.contains_key(input_type) {
                println!("{} is a shell builtin", input_type);
            } else if let Some(path) = find_executables(input_type) {
                println!("{} is {}", input_type, path);
            } else {
                println!("{}: not found", input_type);
            }
        } else if command.starts_with("echo") {
            let output = &command[5..];
            println!("{}", output);
        } else {
            println!("{}: command not found", command.trim());
        }
    }
}
