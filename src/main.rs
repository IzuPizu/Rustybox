/* Suggestions: 1. Write a function for every command
                2. Start with the pwd command
                3. Continue with the other commands that do not have parameters
*/

use std::fs;
use std::fs::File;
use std::fs::Permissions;
use std::io::Read;
use std::os::unix;
use std::os::unix::prelude::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::{env, io};

fn chmod(args_arr: &[String]) -> Result<(), std::io::Error> {
    //check for correct number of arguments
    if args_arr.len() != 2 {
        println!("Invalid command");
        process::exit(-1);
    }

    let perm_mode = &args_arr[0];
    let file = &args_arr[1];

    if args_arr[0].starts_with("-") {
        eprintln!("Invalid command");
        process::exit(-1);
    }

    //case I : numeric permissions
    let mut numeric_perm = false;
    let mut symbolic_perm = false;

    for ch in args_arr[0].chars() {
        if ch.is_digit(10) {
            numeric_perm = true;
            break;
        } else {
            symbolic_perm = true;
            break;
        }
    }

    if numeric_perm {
        let oct_mode: u32 = u32::from_str_radix(&perm_mode, 8).unwrap();
        let permissions = Permissions::from_mode(oct_mode);
        fs::set_permissions(file, permissions)?;
    }

    //case II : symbolic permissions
    if symbolic_perm {
        let mut set_user = false;
        let mut set_group = false;
        let mut set_other = false;
        let mut add_perm = false;

        /* create tuple for r/w/x permissions
           tuple_perm.0 -> read, tuple_perm.1 -> write , tuple_perm.2 -> exec
        */
        let mut tuple_perm = (false, false, false);

        for ch in perm_mode.chars() {
            match ch {
                'u' => set_user = true,
                'g' => set_group = true,
                'o' => set_other = true,
                'a' => {
                    set_user = true;
                    set_group = true;
                    set_other = true
                }
                '+' => add_perm = true,
                '-' => add_perm = false,
                'r' => tuple_perm.0 = true,
                'w' => tuple_perm.1 = true,
                'x' => tuple_perm.2 = true,
                _ => {
                    std::process::exit(-25);
                }
            }
        }

        let mut permissions = fs::metadata(file)?.permissions();

        // set permissions for user
        if set_user {
            if add_perm {
                if tuple_perm.0 {
                    permissions.set_mode(permissions.mode() | 0o400);
                }
                if tuple_perm.1 {
                    permissions.set_mode(permissions.mode() | 0o200);
                }
                if tuple_perm.2 {
                    permissions.set_mode(permissions.mode() | 0o100);
                }
            } else {
                //remove permissions
                if tuple_perm.0 {
                    permissions.set_mode(permissions.mode() & !0o400);
                }
                if tuple_perm.1 {
                    permissions.set_mode(permissions.mode() & !0o200);
                }
                if tuple_perm.2 {
                    permissions.set_mode(permissions.mode() & !0o100);
                }
            }
        }
        // set permissions for group
        if set_group {
            if add_perm {
                if tuple_perm.0 {
                    permissions.set_mode(permissions.mode() | 0o040);
                }
                if tuple_perm.1 {
                    permissions.set_mode(permissions.mode() | 0o020);
                }
                if tuple_perm.2 {
                    permissions.set_mode(permissions.mode() | 0o010);
                }
            } else {
                //remove permissions
                if tuple_perm.0 {
                    permissions.set_mode(permissions.mode() & !0o040);
                }
                if tuple_perm.1 {
                    permissions.set_mode(permissions.mode() & !0o020);
                }
                if tuple_perm.2 {
                    permissions.set_mode(permissions.mode() & !0o010);
                }
            }
        }
        // set permissions for other
        if set_other {
            if add_perm {
                if tuple_perm.0 {
                    permissions.set_mode(permissions.mode() | 0o004);
                }
                if tuple_perm.1 {
                    permissions.set_mode(permissions.mode() | 0o002);
                }
                if tuple_perm.2 {
                    permissions.set_mode(permissions.mode() | 0o001);
                }
            } else {
                //remove permissions
                if tuple_perm.0 {
                    permissions.set_mode(permissions.mode() & !0o004);
                }
                if tuple_perm.1 {
                    permissions.set_mode(permissions.mode() & !0o002);
                }
                if tuple_perm.2 {
                    permissions.set_mode(permissions.mode() & !0o001);
                }
            }
        }

        fs::set_permissions(file, permissions)?;
    }

    Ok(())
}

fn touch(args_arr: &[String]) -> Result<(), io::Error> {
    let mut access_only = false;
    let mut create_file = true;
    let mut modify_only = false;
    let mut file_name = "";
    let mut simple_update = false;

    //check [option] and its type
    for arg in args_arr {
        match arg.as_str() {
            "-a" => access_only = true,
            "-c" | "--no-create" => create_file = false,
            "-m" => modify_only = true,
            _ => {
                file_name = arg;
                simple_update = true;
            }
        }
    }

    //create the file if it does not exist and [option] was set
    if create_file {
        fs::File::create(file_name)?;
    } else if !file_name.is_empty() {
        return Ok(());
    }

    if !file_name.is_empty() {
        //change date and time of access
        if access_only {
            fs::File::open(file_name)?;
        }
        //change date and time of modify
        if modify_only {
            let file = fs::OpenOptions::new().write(true).open(file_name)?;
            file.set_len(0)?;
        }
        //just update access and modify time
        if simple_update {
            let _ = fs::OpenOptions::new().write(true).open(file_name)?;
            fs::read_to_string(file_name)?;
        }
    }
    Ok(())
}

fn rm(args_arr: &[String]) -> Result<(), io::Error> {
    //keep track of number of files/directories that can be removed
    let mut deleted_entities = 0;

    // check if [option] was set and its type
    let recursive = args_arr
        .iter()
        .any(|arg| arg == "-r" || arg == "-R" || arg == "--recursive");

    let dir_only = args_arr.iter().any(|arg| arg == "-d" || arg == "--dir");

    //check for correct number of arguments
    if args_arr.len() < 1 || (recursive && args_arr.len() < 2) || (dir_only && args_arr.len() < 2) {
        println!("Invalid command");
        process::exit(-1);
    }

    let mut length = args_arr.len();
    //if [option] was set =>  update number of arguments that 
    // represents files/directories that should be deleted
    if recursive || dir_only {
        length -= 1;
    }

    //iterate over arguments array
    for arg in args_arr {
        //get path for each argument (file/directory)
        let path = Path::new(&arg);

        //CASE I : if the path is pointing to a directory
        if path.is_dir() && path.exists() {
            deleted_entities += 1;
            if recursive {
                // Recursively delete directories and their contents
                fs::remove_dir_all(path)?;
            } else if dir_only {
                // Delete empty directories
                fs::remove_dir(path)?;
            } else {
                //no option set for directory => cannot delete
                deleted_entities -= 1;
            }
        }

        //CASE II : if the path is pointing to a file
        if path.is_file() && path.exists() {
            // delete files
            deleted_entities += 1;
            fs::remove_file(path)?;
        }
    }

    if recursive && dir_only {
        return Ok(());
    }

    if deleted_entities != length {
        process::exit(-70);
    }

    Ok(())
}

fn list_directory_contents(path: &Path, show_hidden: bool, recursive: bool) -> io::Result<()> {
    // path is pointing to a file
    if path.is_file() {
        println!("{}", path.display());
    } else {
        //path is pointing to a directory

        if recursive {
            println!("{}:", path.display());
        }
        let entries = fs::read_dir(path)?;
        let mut subdirectories: Vec<PathBuf> = Vec::new();

        //if -a/-all option is set => print current and parent directories
        if show_hidden {
            println!(".");
            println!("..");
        }

        //iterate through entries
        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();
            let entry_name = entry_path.file_name().unwrap();
            let entry_name_str = entry_name.to_string_lossy();

            if show_hidden || !entry_name_str.starts_with(".") {
                println!("{}", entry_name_str);
            }

            if entry_path.is_dir() {
                if recursive {
                    // get subdirectories
                    subdirectories.push(entry_path.clone());
                }
            }
        }
        for subdir in subdirectories {
            list_directory_contents(&subdir, show_hidden, recursive)?;
        }
    }

    Ok(())
}

fn ls(args_arr: &[String]) -> Result<(), io::Error> {
    //get current working directory
    let mut path = env::current_dir()?;
    let mut show_hidden = false;
    let mut recursive = false;

    //check [option] was given and its type
    for arg in args_arr {
        match arg.as_str() {
            "-a" | "--all" => show_hidden = true,
            "-R" | "--recursive" => recursive = true,
            _ => path = Path::new(arg).to_path_buf(),
        }
    }

    if path.exists() {
        list_directory_contents(&path, show_hidden, recursive)?;
    } else {
        process::exit(-80);
    }

    Ok(())
}

fn ln(args_arr: &[String]) -> Result<(), io::Error> {
    //check for correct number of arguments
    if args_arr.len() < 2 {
        println!("Invalid command");
        process::exit(-1);
    }

    let mut symbolic = false;

    //check if [option] was given
    if args_arr.len() == 3 {
        match args_arr[0].as_str() {
            "-s" => symbolic = true,
            "--symbolic" => symbolic = true,
            _ => {
                println!("Invalid command");
                process::exit(-1);
            }
        };
    }

    if symbolic {
        // Create a symbolic link
        unix::fs::symlink(&args_arr[1], &args_arr[2])?;
    } else {
        // Create a hard link
        fs::hard_link(&args_arr[0], &args_arr[1])?;
    }

    Ok(())
}

fn copy_recursive(source: &Path, destination: &Path) -> Result<(), io::Error> {
    if source.is_file() {
        fs::copy(source, destination)?;
    } else if source.is_dir() {
        if !destination.exists() {
            fs::create_dir(destination)?;
        }

        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let source_entry = entry.path();
            let destination_entry = destination.join(source_entry.file_name().unwrap());

            copy_recursive(&source_entry, &destination_entry)?;
        }
    }

    Ok(())
}

fn cp(args_arr: &[String]) -> Result<(), io::Error> {
    //check for correct number of arguments
    if args_arr.len() < 2 {
        process::exit(-90);
    }

    let mut source_index = 0;
    let mut recursive = false;
    //check if the [option] was given and update index accordingly
    if args_arr[source_index] == "-R"
        || args_arr[source_index] == "-r"
        || args_arr[source_index] == "--recursive"
    {
        source_index += 1;
        recursive = true;
    }

    let source = &args_arr[source_index];
    let destination = &args_arr[args_arr.len() - 1];

    let source_path = Path::new(source);
    let destination_path = Path::new(destination);

    //if source path does not exists
    if !source_path.exists() {
        std::process::exit(-90);
    }

    //if destination path does not exists
    if !destination_path.exists() {
        if source_path.is_file() {
            fs::copy(source_path, destination_path)?;
        } else {
            //check if option was given
            if recursive {
                //create destination path
                fs::create_dir_all(destination_path)?;
                //copy directory
                copy_recursive(source_path, &destination_path)?;
            } else {
                //recursive option not given => error
                process::exit(-90);
            }
        }
    } else {
        //destination path already exists
        if recursive {
            //recursive option active
            let destination_with_source_name =
                destination_path.join(source_path.file_name().unwrap());
            copy_recursive(source_path, &destination_with_source_name)?;
        } else {
            //recursive option not active
            if source_path.is_file() {
                let destination_with_source_name =
                    destination_path.join(source_path.file_name().unwrap());
                fs::copy(source_path, &destination_with_source_name)?;
            } else {
                //source is directory , we cannot copy without recursive option
                process::exit(-90);
            }
        }
    }

    Ok(())
}

fn echo(args_arr: &[String]) -> Result<(), io::Error> {
    let mut echo_buffer = String::new();

    if args_arr[0] == "-n" {
        //do not add newline
        for current_arg in &args_arr[1..] {
            echo_buffer.push_str(&current_arg);
            echo_buffer.push(' ');
        }
        //get rid of last space
        echo_buffer.pop();
    } else {
        //add newline
        for current_arg in args_arr {
            echo_buffer.push_str(&current_arg);
            echo_buffer.push(' ');
        }
        //get rid of last space
        echo_buffer.pop();

        echo_buffer.push_str("\n");
    }
    print!("{}", echo_buffer);
    Ok(())
}

fn rmdir(dir_arr: &[String]) -> Result<(), io::Error> {
    for current_dir in dir_arr {
        let path = std::path::Path::new(current_dir);
        if path.exists() && path.is_dir() {
            if fs::read_dir(current_dir)?.count() == 0 {
                // Remove the empty directory
                fs::remove_dir(current_dir)?;
            } else {
                process::exit(-60);
            }
        } else {
            process::exit(-60);
        }
    }

    Ok(())
}

fn mv(args_arr: &[String]) -> Result<(), io::Error> {
    let source = &args_arr[0];
    let dest = &args_arr[1];
    //dest path is directory => move the file
    //dest path is a file => rename the file
    fs::rename(source, dest)?;
    Ok(())
}

fn mkdir(args_arr: &[String]) -> Result<(), io::Error> {
    //create each directory
    for current_dir in args_arr {
        fs::create_dir(current_dir)?;
    }
    Ok(())
}

fn cat(args_arr: &[String]) -> Result<(), io::Error> {
    //create a buffer to place content of files
    let mut buffer = String::new();

    for current_file in args_arr {
        //open each file passed as argument
        let mut opened_file = File::open(current_file)?;

        //read content of file
        let mut content = String::new();
        opened_file.read_to_string(&mut content)?;
        //place content of file in the buffer
        buffer.push_str(&content);
    }
    //print the buffer
    print!("{}", buffer);
    Ok(())
}

fn pwd() -> i32 {
    if let Ok(current_dir) = env::current_dir() {
        println!("{}", current_dir.display());
        return 0;
    } else {
        eprintln!("Invalid command");
        return -1;
    }
}

fn main() {
    let command_line_args: Vec<String> = env::args().collect();
    let first_command = &command_line_args[1];

    //check command received as argument
    match first_command.as_str() {
        "pwd" => {
            pwd();
        }

        "cat" => {
            if let Err(_err) = cat(&command_line_args[2..]) {
                process::exit(-20);
            }
        }

        "mkdir" => {
            if let Err(_err) = mkdir(&command_line_args[2..]) {
                process::exit(-30);
            }
        }

        "mv" => {
            if let Err(_err) = mv(&command_line_args[2..]) {
                process::exit(-40);
            }
        }

        "rmdir" => {
            if let Err(_err) = rmdir(&command_line_args[2..]) {
                process::exit(-60);
            }
        }

        "echo" => {
            if let Err(_err) = echo(&command_line_args[2..]) {
                process::exit(-10);
            }
        }

        "cp" => {
            if let Err(_err) = cp(&command_line_args[2..]) {
                process::exit(-90);
            }
        }

        "ln" => {
            if let Err(_err) = ln(&command_line_args[2..]) {
                process::exit(-50);
            }
        }

        "ls" => {
            if let Err(_err) = ls(&command_line_args[2..]) {
                process::exit(-80);
            }
        }

        "rm" => {
            if let Err(_err) = rm(&command_line_args[2..]) {
                process::exit(-70);
            }
        }

        "touch" => {
            if let Err(_err) = touch(&command_line_args[2..]) {
                process::exit(-100);
            }
        }

        "chmod" => {
            if let Err(_err) = chmod(&command_line_args[2..]) {
                process::exit(-25);
            }
        }

        _ => {
            println!("Invalid command");
            process::exit(-1);
        }
    };
}