use std::env;
use std::path::{Path,PathBuf};
use std::fs;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;


pub fn get_path_env() -> Vec<PathBuf> {
    match env::var_os("PATH") {
        Some(path) => env::split_paths(&path).collect(),
        None => Vec::new(), 
    }
}

///Takes the input and expands special characters, treats anything in
/// single quotes as a string literal.
pub fn parse_string(cmd_buffer: String)-> Vec<String>{
    let mut output = Vec::new();
    let mut part_string = String::new();
    let mut expand: bool = true;

    for c in cmd_buffer.chars(){
        if expand == true{
            match c{
            '\'' => expand = false,  
            '~' => part_string.push_str(&get_home_dir()),
            ' ' => {if !part_string.is_empty(){output.push(part_string.clone()); part_string.clear()}},     
            _=> part_string.push(c),
            }
        }else{
            match c{
                '\''=> expand=true,
                _ => part_string.push(c),
            }
        }
        
    }
    
    output.push(part_string);

    output
}



fn get_home_dir()-> String{
    match env::home_dir(){
        Some(path)=> match path.to_str(){
                                    Some(path_str) => path_str.to_string(),
                                    None => String::new(),
                            }
        None => String::new(),
    }
}



pub fn is_executable(filepath: &PathBuf) -> bool {
   let attr = fs::metadata(filepath);
   match attr {
    #[cfg(unix)]
    Ok(attr) => attr.is_file() && ((attr.mode() & 0o111) != 0),
    #[cfg(not(unix))]
    Ok(attr) => attr.is_file(),
    Err(_e) => false,
   }   
}   

pub fn search_for_executables(cmd: &str, paths: &Vec<PathBuf>) -> Option<PathBuf> {
    for path in paths {
        let filepath = path.join(cmd);
        if is_executable(&filepath) {
            return Some(filepath);
        } 
    }
    None
}




#[cfg(test)]
mod tests{
    use super::*;


    #[test]
    fn test_home_dir() {
       let home=get_home_dir();
       assert_eq!(Path::new(&home), env::home_dir().unwrap());
    }
    
    #[test]
    fn test_parse_string(){
        let mut home = String::new();
        if let Some(path) = env::home_dir(){
            home.push_str(path.to_str().unwrap());
        }
        home.push_str("hello~  world");

        let input = String::from("echo ~'hello~  world' ab");
        let test_result = vec!["echo".to_string(), home, "ab".to_string()];
        
        assert_eq!(test_result, parse_string(input));
    }



}
