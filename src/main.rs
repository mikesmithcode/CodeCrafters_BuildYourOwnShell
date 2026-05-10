use std::io::Read;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut cmd_buffer = String::new();
    let _ = io::stdin().read_line(&mut cmd_buffer);
    
    println!("{}: command not found", cmd_buffer.trim());
    
}
