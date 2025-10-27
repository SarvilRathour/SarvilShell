#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
mod shell;
fn main() {
    println!("----Welcome to Sarvil Shell----");
    loop {
        let current_path_home=env::current_dir().unwrap();
        print!("{}> ",current_path_home.display());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let pure_input=input.trim();
        if input.is_empty(){
            continue;
        }
        if let Some(par)=shell::parse(pure_input){
            if let Err(err)=shell::execute(par){
                    eprintln!("{}", err); 
            }
        }
    }
}
