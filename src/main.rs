

#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::path::PathBuf;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::process;

const BUILTINS: &[&str] = &["exit", "type", "echo"];

fn is_builtin(cmd: &str)->bool{
    BUILTINS.contains(&cmd)
}



enum Command{
    Exit,
    Builtin{cmd: String, 
            args: Vec<String>,
            },
    External{cmd: String,
             path: PathBuf,
             args: Vec<String>,
            },
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


fn parse(mut buffer: Vec<String>)->Command{
    if buffer.is_empty(){return Command::Error(String::new());}

    let cmd = buffer.remove(0);
    
    if cmd.as_str() == "exit"{
        return Command::Exit;
    }

    let args = buffer;
    if is_builtin(cmd.as_str()){
        return Command::Builtin {cmd, args};
    }
    
    let path = get_path_env();
    let found_exe = search_for_executables(&cmd, &path);

    match found_exe{
        Some(path) =>  Command::External{cmd, path, args},
        None =>  Command::Error(format!("{}: command not found", cmd)),
    }
    
}



fn is_executable(filepath: &PathBuf)-> bool{
   let attr = fs::metadata(&filepath);
   
   // 0o111 represents the execution bits for Owner (0o100), Group (0o010) and Other (0o001).
   // mode + 0o111 sets all the read and Write bits to zero and one wherever mode is 1.
   // setting that != 0 returns true if any of the bits are nonzero.
   match attr{
    Ok(attr) => attr.is_file() && ((attr.mode() & 0o111) != 0),
    Err(_e) => false,
   }   
}   


fn search_for_executables(cmd: &str, paths: &Vec<PathBuf>)->Option<PathBuf>{
    for path in paths{
        let filepath = path.join(cmd);
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

fn handle_echo(args: Vec<String>){
    println!("{}", args.join(" "));
}

fn handle_type(args: Vec<String>){
    let cmd_arg = args[0].clone();
    if is_builtin(&cmd_arg)
        {
            println!("{} is a shell builtin", cmd_arg);
        }
        else
        { 
            let paths = get_path_env();
            
            let file = search_for_executables(&cmd_arg.to_string(), &paths);
            match file{
                Some(filepath) => println!("{} is {}", cmd_arg.to_string(), filepath.display()),
                None => println!("{}: not found", cmd_arg),
                }                                
        }
}

fn handle_external(path: PathBuf, args: Vec<String>){
    let mut process_cmd = process::Command::new(path);
    process_cmd.args(args);
    let response = process_cmd.output().expect("failed to execute process");
    io::stdout().write_all(&response.stdout).expect("write process response to stdout");
}

/// Execute the command
fn run_command(cmd: Command){
    match cmd{
        Command::Exit => std::process::exit(0),
        Command::Builtin{cmd, args} => {
                    match cmd.as_str(){
                        "echo" => handle_echo(args),
                        "type" => handle_type(args),
                        _ => unreachable!("{} not implemented", cmd),
                    }                                
                },
        Command::External{cmd, path, args} => handle_external(path, args),
        Command::Error(msg) => println!("{}", msg),
  }

}

fn main() {
    loop{
        
        let buffer: Vec<String> = prompt().expect("error with input");
        let cmd = parse(buffer);
        run_command(cmd);

    }
    
    
}
