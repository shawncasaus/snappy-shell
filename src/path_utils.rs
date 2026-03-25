use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub fn find_executables(command: &str) -> Option<String> {
    // hard coded path variable
    // let path_var = "/usr/bin".to_string();

    // get path variable from rc file
    let path_var = env::var("PATH").ok()?;

    // split paths and loop through them
    for dir in env::split_paths(&path_var) {
        // create full path using type command
        let full_path = dir.join(command);

        // check if full path is a file
        if full_path.is_file() {
            // check if file has readable metadata and store it in variable metadata
            if let Ok(metadata) = fs::metadata(&full_path) {
                // get permissions
                let perms = metadata.permissions().mode();
                // if permissions exist and file is executable
                // uses bitwise and operator
                if perms & 0o111 != 0 {
                    // return Some string without invalid bits and owned string
                    return Some(full_path.to_string_lossy().into_owned());
                }
            }
        }
    }
    None
}

pub fn cd(path: &str) {
    let new_path = Path::new(path);

    // handle relative paths
    if !new_path.is_absolute() {
        eprint!("cd: {} is not an absolute path.", path);
        return;
    }
    // handle absolute paths
    else if new_path.is_absolute() {
        if let Err(_) = env::set_current_dir(new_path) {
            eprintln!("cd: {} no such file or directory.", path);
        }
    }
}
