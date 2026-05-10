
#[allow(unused_imports)]
use std::io::{self, Write};

fn echo(cmd_buffer: &str){
    print!("{}", cmd_buffer);
}

fn not_found(cmd_buffer: String){
    println!("{}: command not found", cmd_buffer.trim());
}

fn repl()-> bool{
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut cmd_buffer = String::new();
    let _ = io::stdin().read_line(&mut cmd_buffer);
    
    if cmd_buffer.trim() == "exit" {return true};

    // Remaining commands
    if cmd_buffer.starts_with("echo"){echo(&cmd_buffer[5..]);}
    else{not_found(cmd_buffer);}

    false
}


fn main() {
    loop{
        let exit = repl();
        if exit{break}
    }
    
    
}
