use std::io::{self, Write};
use std::path::{Path,PathBuf};
use std::process;
use std::env;
use std::fs;
#[cfg(unix)]
use std::os::unix::process::CommandExt;


///-----------------------------------------------------------------
/// Builtin commands
/// ----------------------------------------------------------------
pub fn handle_echo(args: Vec<String>) {
    println!("{}", args.join(" "));
}

pub fn handle_type(args: Vec<String>) {
    let cmd_arg = args[0].clone();
    // Reaching back to main and utils
    if crate::is_builtin(&cmd_arg) {
        println!("{} is a shell builtin", cmd_arg);
    } else { 
        let paths = crate::utils::get_path_env();
        let file = crate::utils::search_for_executables(&cmd_arg, &paths);
        match file {
            Some(filepath) => println!("{} is {}", cmd_arg, filepath.display()),
            None => println!("{}: not found", cmd_arg),
        }                                
    }
}

pub fn handle_pwd(){
    let path = env::current_dir().expect("Error getting current dir");
    println!("{}", path.display());
}

pub fn handle_cd(args: Vec<String>){
    
    match std::fs::exists(args[0].as_str()){
        Ok(found) =>{
                        let path = Path::new(args[0].as_str());
                        if found{env::set_current_dir(&path).is_ok();}
                        else{println!("cd: {}: No such file or directory", path.display())}
                    },
        Err(e) => println!("{}",e),
    };
    
}

///-----------------------------------------------------------------
/// External commands
/// ----------------------------------------------------------------
pub fn handle_external(cmd: String, path: PathBuf, args: Vec<String>) {
    let mut process_cmd = process::Command::new(path);
    #[cfg(unix)]
    process_cmd.arg0(&cmd);
    process_cmd.args(args);
    let response = process_cmd.output().expect("failed to execute process");
    std::io::stdout().write_all(&response.stdout).unwrap();
}
