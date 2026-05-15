mod filter_config;
pub use filter_config::*;
use std::fs;

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub content: String,
    pub dir: String
}
impl File {
    pub fn new(name: String, content: String, dir: String) -> Self {
        Self { name, content, dir }
    }
    pub fn from_path(path: &std::path::Path) -> Result<Self, std::io::Error> {
        let name = path.to_string_lossy().to_string();
        // let content = fs::read_to_string(path)?;
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => return Err(e)
        };
        let dir = path.parent().unwrap_or_else(|| path).to_string_lossy().to_string();
        Ok(Self {
            name,
            content,
            dir
        })
    }
}

