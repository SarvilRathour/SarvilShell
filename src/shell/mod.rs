mod builtins;
mod external;
mod std_out;
use std::fs;
use std::collections::HashMap;
use builtins::handle_builtins;
use builtins::BuiltInCommand;
use external::run_external;
use external::ExternalCommand;
use std_out::moving;
use std_out::StdOutCommand;
use std_out::StdOutType;
pub enum CommandType {
    BuiltIn(BuiltInCommand),
    External(ExternalCommand),
    Redirect(StdOutCommand),
}
pub fn parse(input: &str) -> Option<CommandType> {
    let inputs: Vec<&str> = input.split(" ").collect();
    if inputs.is_empty() {
        return None;
    }
    if input.contains('>') {
        let split: Vec<&str> = input.split(">").collect();
        let left = split[0].trim();
        let right = split.get(1)?.trim().to_string();
        let left_parts: Vec<&str> = left.split_whitespace().collect();
        let cmd = left_parts[0];
        let args = left_parts[1..]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let return_redirect = vec!["echo", "cat", "ls", "pwd"];
        if return_redirect.contains(&cmd) {
            let cmdd = match cmd {
                "echo" => BuiltInCommand::Echo(args.join(" ")),
                "cat" => BuiltInCommand::Cat(args),
                "ls" => BuiltInCommand::Ls,
                "pwd" => BuiltInCommand::Pwd,
                _ => return None,
            };
            return Some(CommandType::Redirect(StdOutCommand{
                command:StdOutType::Builtin(cmdd),
                destination:right,
            }));
        }
        //behanchod ya nhi chalega
        // else{
        //   // let ext=ExternalCommand{
        //   //   program:cmd.to_string(),
        //   //   args,
        //   // };
        //   return Some(CommandType::Redirect(StdOutCommand{
        //     command:StdOutType::External(cmd.to_string()),
        //     destination:right,
        //   }));
        // }
    }
    
    match inputs[0] {
        "exit" => {
            let code = inputs[1].parse().unwrap_or(0);
            Some(CommandType::BuiltIn(BuiltInCommand::Exit(code)))
        }
        "echo" => {
            let parts = inputs[1..].join(" ");
            Some(CommandType::BuiltIn(BuiltInCommand::Echo(parts)))
        }
        "type" => Some(CommandType::BuiltIn(BuiltInCommand::Type(
            inputs[1].to_string(),
        ))),
        "pwd" => Some(CommandType::BuiltIn(BuiltInCommand::Pwd)),
        "cd" => Some(CommandType::BuiltIn(BuiltInCommand::Cd(
            inputs[1].to_string(),
        ))),
        "cat" => Some(CommandType::BuiltIn(BuiltInCommand::Cat(
            inputs[1..].iter().map(|s| s.to_string()).collect(),
        ))),
        "ls" => Some(CommandType::BuiltIn(BuiltInCommand::Ls)),
        "touch"=>Some(CommandType::BuiltIn(BuiltInCommand::Touch(
            inputs[1].to_string()
        ))),
        "clear" => Some(CommandType::BuiltIn(BuiltInCommand::Clear)),
        ext => Some(CommandType::External(ExternalCommand {
            program: ext.to_string(),
            args: inputs[1..].iter().map(|s| s.to_string()).collect(),
        })),
    }
}
pub fn execute(cmd: CommandType) -> Result<String, String> {
    match cmd {
        CommandType::BuiltIn(built) => handle_builtins(built, true),
        CommandType::External(ext) => run_external(ext),
        CommandType::Redirect(out) => moving(out),
    }
}

