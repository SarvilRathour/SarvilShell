use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
mod shell;
use crate::shell::CommandType;
// pub struct Parser {
//     pub command: CommandType,
//     pub std_out: Option<String>,
// }
fn main() {
    println!("----Welcome to Sarvil Shell----");
    loop {
        let current_path_home = env::current_dir().unwrap();
        print!("{}> ", current_path_home.display());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let pure_input = input.trim();
        if input.is_empty() {
            continue;
        }
            if let Some(par) = shell::parse(pure_input) {
                if let Err(err) = shell::execute(par) {
                    eprintln!("{}", err);
                }
            }
        }
    }

