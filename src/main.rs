#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::PathBuf;

// Declare the other files as modules
mod utils;
mod handlers;

const BUILTINS: &[&str] = &["exit", "type", "echo", "pwd"];

pub fn is_builtin(cmd: &str) -> bool {
    BUILTINS.contains(&cmd)
}

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

fn parse(mut buffer: Vec<String>) -> Command {
    if buffer.is_empty() { return Command::Error(String::new()); }
    let cmd = buffer.remove(0);
    
    if cmd == "exit" { return Command::Exit; }

    let args = buffer;
    if is_builtin(&cmd) {
        return Command::Builtin { cmd, args };
    }
    
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
                _ => unreachable!("{} not implemented", cmd),
            }                                
        },
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
