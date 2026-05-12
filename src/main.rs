

#[allow(unused_imports)]
use std::io::{self, Write};


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


/// Execute the command
fn run_command(cmd: Command){
    match cmd{
        Command::Exit => std::process::exit(0),
        Command::Echo(args) => println!("{}", args.join(" ")),
        Command::Type(arg) => {if is_builtin(&arg)
                                {
                                    println!("{} is a shell builtin", arg);
                                }
                                else{ 
                                    println!("{}: not found", arg)
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
