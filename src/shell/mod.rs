mod builtins;
mod external;
use builtins::BuiltInCommand;
use external::ExternalCommand;
use builtins::handle_builtins;
use external::run_external;
pub enum CommandType{
  BuiltIn(BuiltInCommand),
  External(ExternalCommand),
}
pub fn parse(input:&str)->Option<CommandType>{
    let mut inputs:Vec<&str>=input.split(" ").collect();
    if inputs.is_empty(){
      return None;
    }
    match inputs[0]{
        "exit"=>{
          let code=inputs[1].parse().unwrap_or(0);
          Some(CommandType::BuiltIn(BuiltInCommand::Exit(code)))
        },
        "echo"=>{
          let parts=inputs[1..].join(" ");
          Some(CommandType::BuiltIn(BuiltInCommand::Echo(parts)))
        },
        "type"=>{
          Some(CommandType::BuiltIn(BuiltInCommand::Type(inputs[1].to_string())))
        }
        "pwd"=>{
          Some(CommandType::BuiltIn(BuiltInCommand::Pwd))
        }
        "cd"=>{
          Some(CommandType::BuiltIn(BuiltInCommand::Cd(inputs[1].to_string())))
        }
        "cat"=>{
          Some(CommandType::BuiltIn(BuiltInCommand::Cat(inputs[1..].iter().map(|s| s.to_string()).collect())))
        },
        "ls"=>{
          Some(CommandType::BuiltIn(BuiltInCommand::Ls))
        }
        ext=>{
          Some(CommandType::External(ExternalCommand{
            program:ext.to_string(),
            args:inputs[1..].iter().map(|s| s.to_string()).collect()
          }))
        }
    }
}
pub fn execute(cmd:CommandType)->Result<(),String>{
  match cmd{
    CommandType::BuiltIn(built)=>handle_builtins(built),
    CommandType::External(ext)=>run_external(ext),
  }
}

