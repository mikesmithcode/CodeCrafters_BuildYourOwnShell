
#[allow(unused_imports)]
use std::io::{self, Write};


fn repl()-> bool{
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut cmd_buffer = String::new();
    let _ = io::stdin().read_line(&mut cmd_buffer);
    
    let exit = match cmd_buffer.trim(){
        "exit" => true,
        _ => {println!("{}: command not found", cmd_buffer.trim());
                false},
    };
    exit
}


fn main() {
    loop{
        let exit = repl();
        if exit{break}
    }
    
    
}
