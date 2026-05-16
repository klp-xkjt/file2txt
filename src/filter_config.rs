// 默认的常用扩展名配置
pub const DEFAULT_EXTENSIONS: &[&str] = &[
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

// 过滤的类型
pub enum FilterDecision {
    Keep, // 保留
    ExcludeDir, // 以目录排除
    ExcludeExt, // 以后缀名排除
    ExcludeSize, // 以文件大小排除
    ExcludeNotFile, // 以是否为二进制或者非文件排除
}

pub struct FilterConfig {
    pub extensions: Vec<String>,
    pub exclude_dirs: Vec<String>,
    pub max_size: u64
}
impl FilterConfig {
    // 创建默认配置
    pub fn default() -> Self {
        Self {
            extensions: DEFAULT_EXTENSIONS.iter().map(|s| s.to_string()).collect(),
            exclude_dirs: vec![".git".to_string(), "target".to_string(), "node_modules".to_string()],
            max_size: 1024 * 1024, // 1MB
        }
    }
    // 判断是否符合要求
    pub fn decide(&self, entry: &walkdir::DirEntry) -> FilterDecision {
        let path = entry.path();

        // 判断是否为路径并排除
        if !entry.file_type().is_file() {
            return FilterDecision::ExcludeNotFile;
        }

        // 判断是否为被忽略的目录并排除其中文件
        for exclude in &self.exclude_dirs {
            if path.components().any(|e| &e.as_os_str().to_string_lossy().to_string() == exclude) {
                return FilterDecision::ExcludeDir;
            }
        }
        // 判断文件扩展名
        if !self.extensions.is_empty() {
            if let Some(ext) = path.extension().and_then(|x| x.to_str()) {
                if !self.extensions.contains(&ext.to_string()) {
                    return FilterDecision::ExcludeExt;
                }
            } else {
                return FilterDecision::ExcludeExt;
            }
        }
        // 判断文件大小
        if self.max_size > 0 {
            if let Ok(meta) = entry.metadata() {
                if meta.len() > self.max_size {
                    return FilterDecision::ExcludeSize;
                }
            }
        }
        FilterDecision::Keep
    }
}