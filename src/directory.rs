use std::env;
use std::io;
use std::path::{Path, PathBuf};

pub struct Directory {
    current_path: PathBuf,
}

impl Directory {
    pub fn new() -> Result<Self, io::Error> {
        let current_path = env::current_dir()?;
        Ok(Self { current_path })
    }

    pub fn cd(&mut self, dir: &str) -> Result<(), io::Error> {
        let error_message = format!("cd: {}: No such file or directory", dir);
        match dir {
            "~" => {
                let home_dir = env::var("HOME").unwrap_or_default();
                env::set_current_dir(&home_dir)?;
                self.current_path = PathBuf::from(&home_dir);
                Ok(())
            }
            _ => {
                if Path::new(dir).is_absolute() {
                    self.cd_absolute_path(dir, &error_message)
                } else {
                    let mut p = self.current_path.clone();
                    p.push(dir);
                    let normalized_dir = p.canonicalize()?;
                    env::set_current_dir(&normalized_dir)?;
                    self.current_path = normalized_dir;
                    Ok(())
                }
            }
        }
    }

    pub fn pwd(&self) -> String {
        self.current_path.display().to_string()
    }

    fn cd_absolute_path(&mut self, dir: &str, error_message: &str) -> Result<(), io::Error> {
        env::set_current_dir(dir)
            .and_then(|_| {
                self.current_path = PathBuf::from(dir);
                Ok(())
            })
            .map_err(|_| io::Error::new(io::ErrorKind::Other, error_message))
    }
}
