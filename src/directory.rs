use std::env;
use std::io;
pub struct Directory {
    current_path: String,
}

impl Directory {
    pub fn new() -> Self {
        Self { current_path: env::current_dir()
            .map(|path| path.display().to_string())
            .unwrap_or_default() }
    }

    pub fn cd(&mut self, dir: &str) -> Result<(), io::Error> {
        env::set_current_dir(dir)
            .and_then(|_| {
                self.current_path = dir.to_string();
                Ok(())
            })
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "failed to change directory"))
    }

    pub fn pwd(&self) -> String {
        self.current_path.clone()
    }
}
