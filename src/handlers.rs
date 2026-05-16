use std::io::Write;
use std::path::{PathBuf,Component};
use std::process;
use std::env;
use std::fs;
use std::str;
#[cfg(unix)]
use std::os::unix::process::CommandExt;

use anyhow::Error;


///-----------------------------------------------------------------
/// Builtin commands
/// ----------------------------------------------------------------
pub fn handle_echo(args: Vec<String>) {
    println!("{}", args.join(" "));
}

pub fn handle_type(args: Vec<String>) {
    let cmd_arg = args[0].clone();
    
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
    let path: PathBuf = path_for_cd(&args[0]).expect("Must have / in path");

    match std::fs::exists(&path){
        Ok(found) =>{
                        if found{env::set_current_dir(&path).expect("not found path");}
                        else{println!("cd: {}: No such file or directory", path.display())}
                    },
        Err(e) => println!("{}",e),
    };
    
}

fn path_for_cd(arg: &str) -> Result<PathBuf, &'static str> {
    
    
    let full_path = if arg.starts_with('/') {
        PathBuf::from(arg)
    } else if arg.starts_with('~'){
        return Ok(env::home_dir().expect("Could not get home_dir").join(PathBuf::from(&arg[1..].trim_start_matches('/'))))
    } 
    else {
        let current_dir = env::current_dir().expect("Couldn't get current dir");
        current_dir.join(arg)
    };

    let mut path = PathBuf::new();
    
    for component in full_path.components() {
        match component {
            Component::Prefix(p) => path.push(Component::Prefix(p)),
            Component::RootDir => path.push(Component::RootDir),
            Component::CurDir => {} 
            Component::ParentDir => {
                path.pop();
            }
            Component::Normal(c) => {
                path.push(c);
            }
        }
    }

    Ok(path)
}


    /*
    
    
    

    let home_dir= env::home_dir().expect("Error getting home_dir");
    let mut current_dir = env::current_dir().expect("Error getting current dir");
    &current_dir.pop();
    let parent_dir = current_dir();

    println!("{}", prefix);
*/

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

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use gag::BufferRedirect;
    use std::io::Read;
    use std::env;
    use tempfile::tempdir;



    #[test]
    fn test_path_for_cd() {
        let input = String::from("~");
        // call your function here
       let path = path_for_cd(&input);

       println!("Path is {}", path.unwrap().display());


    }
    


    #[test]
    fn handle_type_builtin() {
        let input = vec![String::from("echo")];
        // call your function here
        let mut buf = BufferRedirect::stdout().unwrap();
        
        handle_type(input);
        
        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        assert_eq!(output, "echo  is a shell builtin");
    }

    

    #[test]
    fn test_handle_type_finds_executable() {
        // Mocks up a temporary file in tempdir. Temporarily modifies
        // the path variable and calls command capturing output.
        let dir = tempdir().expect("Failed to create temp dir");
        let dir_path = dir.path();

        let file_path = dir_path.join("my_command");
        File::create(&file_path).expect("Failed to create dummy file");

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&file_path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&file_path, perms).unwrap();
        }

        let old_path = env::var_os("PATH");
        unsafe{
            env::set_var("PATH", dir_path);
        }

        handle_type(vec!["my_command".to_string()]);

        unsafe{
            if let Some(p) = old_path {
                env::set_var("PATH", p);
            } else {
                env::remove_var("PATH");
            }
        }
    }



}
