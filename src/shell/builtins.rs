use super::external::find_path;
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::process;
// let mut default_print:bool=true;
#[derive(Debug)]
pub enum BuiltInCommand {
    Exit(i32),
    Echo(String),
    Type(String),
    Pwd,
    Ls,
    Cd(String),
    Cat(Vec<String>),
    Touch(String),
    Clear,
}
pub fn handle_builtins(cmd: BuiltInCommand,print:bool) -> Result<String, String> {
    match cmd {
        BuiltInCommand::Exit(code) => process::exit(code),
        BuiltInCommand::Echo(msg) => {
        // let output=msg;
        if print{
            println!("{}", msg);
        
        }
            Ok(msg)
        }
        BuiltInCommand::Type(cmd) => {
            let builtins = vec!["type", "echo", "exit", "cd", "pwd"];
            if builtins.contains(&cmd.as_str()) {
                println!("{} is a shell builtin", cmd);
                Ok("Success".to_string())
            } else if let Some(path) = find_path(&cmd) {
                println!("{}", path);
                Ok("Success".to_string())
            } else {
                Err(format!("{}:not found", cmd))
            }
        }
        BuiltInCommand::Pwd => {
            let current_path = env::current_dir().unwrap();
            println!("{}", current_path.display());
            Ok(current_path.display().to_string())
        }
        BuiltInCommand::Cd(destination) => {
            if destination == String::from("~") {
                let home_path = Path::new(r"C:\Users\lenovo");
                env::set_current_dir(home_path);
                return Ok(destination);
            }
            let dest_path = Path::new(&destination);
            if dest_path.exists() {
                match env::set_current_dir(dest_path) {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("Failed to change the directory:{}", e);
                    }
                }
                Ok(destination)
            } else {
                Err(format!("cd: {:?}: No such file or directory", dest_path))
            }
        }
        BuiltInCommand::Ls=>{
            let current_path=env::current_dir().unwrap();
            if let Ok(enteries)=fs::read_dir(current_path.as_path()){
                for entry in enteries{
                    if let Ok(entry)=entry{
                    if print{println!("{}",entry.file_name().to_string_lossy());}
                    }
                } 
            }else{
                eprintln!("Failed to list the current directories");
            }
            Ok(current_path.display().to_string())
        }
        BuiltInCommand::Touch(for_creation)=>{
            let created=fs::File::create(for_creation).unwrap();
            // match created{
            //     Ok(a)=>Ok("success".to_string()),
            //     Err(e)=>eprintln!("Error: {}",e),
            // }
            Ok("success".to_string())
        }
        BuiltInCommand::Cat(displays) => {
        let mut combined=String::new();
            for display in displays {
                match fs::read_to_string(display) {
                    Ok(contents) => {
                        combined.push_str(&contents);
                        combined.push('\n');
                        // return Ok(contents);
                    }
                    Err(e) => match e.kind() {
                        ErrorKind::NotFound => {
                            eprintln!("No such file or directory");
                        }
                        _ => eprintln!("Error: {}", e),
                    },
                }
            }
            if print{
                print!("{}",combined);
            }
            Ok(combined)
            // Ok(())
        }
        BuiltInCommand::Clear => {
            print!("\x1B[2J\x1B[1;1H");
            Ok("Success".to_string())
        }
    }
}
