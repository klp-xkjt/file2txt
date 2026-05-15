const DEFAULT_EXTENSIONS: &[&str] = &[
    "rs", "toml", "md", "txt", "json", "yaml", "yml",
    "js", "ts", "jsx", "tsx", "html", "css", "scss", "less",
    "xml", "svg", "graphql", "gql",
    "sh", "bash", "zsh", "fish",
    "py", "pyc",
    "go", "mod", "sum",
    "c", "cpp", "h", "hpp", "cc", "cxx",
    "java", "kt", "kts", "scala", "clj", "cljs",
    "rb", "erb", "rake",
    "php", "phtml",
    "swift", "m", "mm",
    "rs", "rlib",  // Rust
    "cs", "fs", "fsx", // C#, F#
    "vue", "svelte",
    "ini", "cfg", "conf", "config",
    "log", "gitignore", "gitattributes", "dockerignore",
    "env", "example", "lock", "license", "readme", "authors",
    "sql", "psql",
    "tex", "latex",
    "org", "wiki",
    "adoc", "asciidoc"
];

pub struct FilterConfig {
    pub extensions: Vec<String>,
    pub exclude_dirs: Vec<String>,
    pub max_size: u64
}
impl FilterConfig {
    pub fn default() -> Self {
        Self {
            extensions: DEFAULT_EXTENSIONS.iter().map(|s| s.to_string()).collect(),
            exclude_dirs: vec![".git".to_string(), "target".to_string(), "node_modules".to_string()],
            max_size: 1024 * 1024, // 1MB
        }
    }
    pub fn should_process(&self, entry: &walkdir::DirEntry) -> bool {
        let path = entry.path();

        if !entry.file_type().is_file() {
            return false;
        }

        for exclude in &self.exclude_dirs {
            if path.components().any(|e| &e.as_os_str().to_string_lossy().to_string() == exclude) {
                return false;
            }
        }

        if !self.extensions.is_empty() {
            if let Some(ext) = path.extension().and_then(|x| x.to_str()) {
                if !self.extensions.contains(&ext.to_string()) {
                    return false;
                }
            } else {
                return false;
            }
        }

        if self.max_size > 0 {
            if let Ok(meta) = entry.metadata() {
                if meta.len() > self.max_size {
                    return false;
                }
            }
        }
        true
    }
}