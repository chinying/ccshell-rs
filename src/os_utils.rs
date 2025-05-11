use std::env;
use std::fs;
use std::io;

pub fn get_path() -> Result<String, env::VarError> {
    env::var("PATH")
}

pub fn get_dir_from_path(path: &str) -> Vec<String> {
    path.split(':').map(|dir| dir.to_string()).collect()
}

fn is_dir(dir: &str) -> bool {
    fs::metadata(dir).unwrap().is_dir()
}

pub fn list_dir(dir: &str) -> Result<Vec<String>, io::Error> {
    if !is_dir(dir) {
        return Err(io::Error::new(io::ErrorKind::Other, "not a directory"));
    }
    fs::read_dir(dir)?
      .map(|res| {
        res
        .map(|e| e.file_name().into_string().unwrap())
      })
      .collect()
}

pub fn pwd() -> Result<String, io::Error> {
    env::current_dir()
        .map(|path| path.display().to_string())
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "failed to get current directory"))
}
