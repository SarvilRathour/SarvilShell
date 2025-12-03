use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
mod shell;
use crate::shell::CommandType;
pub struct History{
    pub entries:Vec<String>,
}
impl History{
    pub fn new()->Self{
        Self{entries:Vec::new()}
    }
    pub fn add(&mut self,entry:&str){
        self.entries.push(entry.to_string())
    }
}
fn main() {
    println!("----Welcome to Sarvil Shell---");
    
    let mut rl = DefaultEditor::new().unwrap();
    let mut history=History::new();
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
                history.add(pure_input);
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
