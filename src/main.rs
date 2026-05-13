#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::PathBuf;

// Declare the other files as modules
mod utils;
mod handlers;

const BUILTINS: &[&str] = &["exit", "type", "echo", "pwd", "cd"];

pub fn is_builtin(cmd: &str) -> bool {
    BUILTINS.contains(&cmd)
}

/// Command
///
/// Classifies type of input and stores key info for dealing with
enum Command {
    Exit,
    Builtin { cmd: String, args: Vec<String> },
    External { cmd: String, path: PathBuf, args: Vec<String> },
    Error(String),
}


fn prompt() -> Result<Vec<String>, std::io::Error> {
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut cmd_buffer = String::new();
    let _ = io::stdin().read_line(&mut cmd_buffer);
    let input: Vec<String> = cmd_buffer.split_whitespace().map(|x| x.to_string()).collect();
    Ok(input)
}

/// parse prompt input
///
/// the input from prompt will be vec![command, arg1, arg2 etc] where args are optional
fn parse(mut buffer: Vec<String>) -> Command {
    if buffer.is_empty() { return Command::Error(String::new()); }
    let cmd = buffer.remove(0);
    
    if cmd == "exit" { return Command::Exit; }

    let args = buffer;
    if is_builtin(&cmd) {
        return Command::Builtin { cmd, args };
    }
    
    // path is a Vec<PathBuf> of the PATH variable
    let path = utils::get_path_env();
    let found_exe = utils::search_for_executables(&cmd, &path);

    match found_exe {
        Some(path) => Command::External { cmd, path, args },
        None => Command::Error(format!("{}: command not found", cmd)),
    }
}

fn run_command(cmd: Command) {
    match cmd {
        Command::Exit => std::process::exit(0),
        Command::Builtin { cmd, args } => {
            match cmd.as_str() {
                "echo" => handlers::handle_echo(args),
                "type" => handlers::handle_type(args),
                "pwd" => handlers::handle_pwd(),
                "cd" => handlers::handle_cd(args),
                _ => unreachable!("{} not implemented", cmd),
            }                                
        },
        // Iteratively searches the various paths in PATH to see if cmd matches 
        // then executes at first occurrence.
        Command::External { cmd, path, args } => handlers::handle_external(cmd, path, args),
        Command::Error(msg) => if !msg.is_empty() { println!("{}", msg) },
    }
}

fn main() {
    loop {
        let buffer = prompt().expect("error with input");
        let cmd = parse(buffer);
        run_command(cmd);
    }
}
