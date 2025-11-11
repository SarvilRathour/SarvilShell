use super::builtins::*;
use super::external::*;
use std::fs;
#[derive(Debug)]
pub enum StdOutType{
  Builtin(BuiltInCommand),
  // External(String),
}
#[derive(Debug)]
pub struct StdOutCommand{
  pub command:StdOutType,
  pub destination:String,
}
pub fn moving(cmd:StdOutCommand)->Result<String,String>{
    // println!("{}",cmd.command);
    // println!("{}",cmd.source);
    // println!("{}",cmd.destination);
    // let output=handle_builtins(cmd.command, false);
    let output=match cmd.command{
      StdOutType::Builtin(builtin_cmd)=>handle_builtins(builtin_cmd, false)?,
      // StdOutType::External(ext_cmd)=>run_external(ext_cmd)?,
    };
    fs::write(cmd.destination, output.clone());
    
  Ok("string".to_string())
}
