

#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::path::PathBuf;
use std::fs;
use std::os::unix::fs::MetadataExt;

const BUILTINS: &[&str] = &["exit", "type", "echo"];

fn is_builtin(cmd: &str)->bool{
    BUILTINS.contains(&cmd)
}

enum Command{
    Exit,
    Echo(Vec<String>),
    Type(String),
    Error(String),
}


fn prompt()-> Result<Vec<String>, std::io::Error>{
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut cmd_buffer = String::new();
    let _ = io::stdin().read_line(&mut cmd_buffer);
    
    let input: Vec<String> = cmd_buffer.split_whitespace().map(|x| x.to_string()).collect();
    Ok(input)
}

/// Figures out which enum variant we have and adds args to variant if appropriate
fn parse(mut buffer: Vec<String>)->Command{
    if buffer.is_empty(){return Command::Error(String::new());}

    let cmd_name = buffer.remove(0);

    match cmd_name.as_str(){
        "exit" => Command::Exit,
        "echo" => Command::Echo(buffer),
        "type" => if buffer.is_empty() {
                        Command::Error("type: missing argument".to_string())
                    } else {
                        Command::Type(buffer.remove(0))
                    }
        _ => Command::Error(format!("{}: command not found", cmd_name)),
    }
}



fn is_executable(filepath: &PathBuf)-> bool{
   let attr = fs::metadata(&filepath);
   
   // 0o111 represents the execution bits for Owner (0o100), Group (0o010) and Other (0o001).
   // mode + 0o111 sets all the read and Write bits to zero and one wherever mode is 1.
   // setting that != 0 returns true if any of the bits are nonzero.
   match attr{
    Ok(attr) => attr.is_file() && ((attr.mode() & 0o111) != 0),
    Err(e) => false,
   }   
}   


fn search_for_executables(cmd: String, paths: Vec<PathBuf>)->Option<PathBuf>{
    for path in &paths{
        let filepath = path.join(&cmd);
        if is_executable(&filepath){
            return Some(filepath);
        } 
    }
    None
}


fn get_path_env()-> Vec<PathBuf>{
    match env::var_os("PATH"){
        Some(path) => env::split_paths(&path).collect(),
        None => Vec::new(), 
    }
}

/// Execute the command
fn run_command(cmd: Command){
    match cmd{
        Command::Exit => std::process::exit(0),
        Command::Echo(args) => println!("{}", args.join(" ")),
        Command::Type(arg) => {if is_builtin(&arg)
                                {
                                    println!("{} is a shell builtin", arg);
                                }
                                else
                                { 
                                    let paths = get_path_env();
                                    let cmd = arg.clone();
                                    let file = search_for_executables(arg.to_string(), paths);
                                    match file{
                                        Some(filepath) => println!("{} is {}", cmd.to_string(), filepath.display()),
                                        None => println!("{}: not found", cmd),
                                        }                                
                                }
                            },
        Command::Error(arg) => println!("{}", arg),
                        }

}

fn main() {
    loop{
        
        let buffer: Vec<String> = prompt().expect("error with input");
        let cmd = parse(buffer);
        run_command(cmd);

        
        

    }
    
    
}
