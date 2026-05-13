use std::env;
use std::path::PathBuf;
use std::fs;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;


pub fn get_path_env() -> Vec<PathBuf> {
    match env::var_os("PATH") {
        Some(path) => env::split_paths(&path).collect(),
        None => Vec::new(), 
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
