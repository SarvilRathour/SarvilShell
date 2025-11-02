use std::env;
use std::path::Path;
use std::process::Command;
pub struct ExternalCommand{
  pub program:String,
  pub args:Vec<String>,
}
pub fn find_path(cmd:&String)->Option<String>{
    if let Ok(path_string)=env::var("PATH"){
      let paths:Vec<&str>=path_string.split(';').collect();
      for path in paths{
        let full_path=format!("{}\\{}.exe",path,cmd);
        if Path::new(&full_path).is_file(){
          return Some(full_path)
        }
      }
    }
    None
}

pub fn run_external(cmd:ExternalCommand)->Result<String,String>{
        if let Some(path)=find_path(&cmd.program){
                let status=Command::new(path)
                           .args(cmd.args)
                           .status()
                           .expect("failed to execute process");
                 if !status.success(){
                       return Err(format!("{} exited with status {}", cmd.program, status));
                 }
                 Ok("Success".to_string())
        }else{
          Err(format!("{}: not found", cmd.program))
        }
}
