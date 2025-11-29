use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
mod shell;
use crate::shell::CommandType;

fn main() {
    println!("----Welcome to Sarvil Shell---");
    
    let mut rl = DefaultEditor::new().unwrap();
    
    loop {
        let current_path = std::env::current_dir().unwrap();
        let prompt = format!("{}> ", current_path.display());
        
        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                let pure_input = line.trim();
                if pure_input.is_empty() {
                    continue;
                }
                
                if let Some(par) = shell::parse(pure_input) {
                    if let Err(err) = shell::execute(par) {
                        eprintln!("{}", err);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C
                continue;
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}
