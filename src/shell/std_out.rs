use super::builtins::*;
use std::fs;
#[derive(Debug)]
pub struct StdOutCommand{
  pub command:BuiltInCommand,
  pub destination:String,
}
pub fn moving(cmd:StdOutCommand)->Result<String,String>{
    // println!("{}",cmd.command);
    // println!("{}",cmd.source);
    // println!("{}",cmd.destination);
    let output=handle_builtins(cmd.command, false);
    fs::write(cmd.destination, output.unwrap());
    
  Ok("string".to_string())
}
