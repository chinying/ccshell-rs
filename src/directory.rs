use crate::os_utils;

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
        match dir {
            dir if dir.starts_with("/") => self.cd_absolute_path(dir, &error_message),
            _ => {
                let parts = dir.split("/").collect::<Vec<&str>>();
                for part in parts {
                    match part {
                        ".." => {
                            self.current_path.pop();
                            env::set_current_dir(self.current_path.join("/"))?;
                        }
                        _ => {
                            if os_utils::list_dir(self.pwd().as_str())?
                                .iter()
                                .any(|s| s == part)
                            {
                                self.current_path.push(part.to_string());
                                env::set_current_dir(self.current_path.join("/"))?;
                            }
                        }
                    }
                }
                Ok(())
            }
        }
    }

    pub fn pwd(&self) -> String {
        self.current_path.join("/")
    }

    fn cd_absolute_path(&mut self, dir: &str, error_message: &str) -> Result<(), io::Error> {
        env::set_current_dir(dir)
            .and_then(|_| {
                self.current_path = dir.to_string().split("/").map(|s| s.to_string()).collect();
                Ok(())
            })
            .map_err(|_| io::Error::new(io::ErrorKind::Other, error_message))
    }
}
