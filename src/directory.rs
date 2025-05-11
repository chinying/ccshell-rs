use std::env;
use std::io;
pub struct Directory {
    current_path: Vec<String>,
}

impl Directory {
    pub fn new() -> Self {
        Self {
            current_path: env::current_dir()
                .map(|path| {
                    path.display()
                        .to_string()
                        .split("/")
                        .map(|s| s.to_string())
                        .collect()
                })
                .unwrap_or_default(),
        }
    }

    pub fn cd(&mut self, dir: &str) -> Result<(), io::Error> {
        let error_message = format!("cd: {}: No such file or directory", dir);
        env::set_current_dir(dir)
            .and_then(|_| {
                self.current_path = dir.to_string().split("/").map(|s| s.to_string()).collect();
                Ok(())
            })
            .map_err(|_| io::Error::new(io::ErrorKind::Other, error_message))
    }

    pub fn pwd(&self) -> String {
        self.current_path.join("/")
    }
}
