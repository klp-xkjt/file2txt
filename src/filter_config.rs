// 默认的常用扩展名配置（文本文件）
pub const DEFAULT_EXTENSIONS: &[&str] = &[
    // 配置 & 数据
    "rs",
    "toml",
    "md",
    "txt",
    "json",
    "yaml",
    "yml",
    "ini",
    "cfg",
    "conf",
    "config",
    "env",
    "example",
    "lock",
    "license",
    "readme",
    "authors",
    "gitignore",
    "gitattributes",
    "dockerignore",
    "editorconfig",
    "prettierrc",
    "eslintrc",
    "babelrc",
    // Web 前端
    "html",
    "htm",
    "css",
    "scss",
    "sass",
    "less",
    "styl",
    "js",
    "mjs",
    "cjs",
    "ts",
    "jsx",
    "tsx",
    "vue",
    "svelte",
    "wasm",
    "wast",
    // 标记 / 文档
    "xml",
    "svg",
    "xhtml",
    "rss",
    "atom",
    "graphql",
    "gql",
    "proto",
    "thrift",
    "rst",
    "tex",
    "latex",
    "ltx",
    "org",
    "wiki",
    "adoc",
    "asciidoc",
    "pod",
    "rtf",
    "roff",
    "man",
    // Shell & 脚本
    "sh",
    "bash",
    "zsh",
    "fish",
    "ps1",
    "psm1",
    "psd1",
    "bat",
    "cmd",
    "vbs",
    "pl",
    "pm",
    "t",
    "raku",
    "nu",
    // Python
    "py",
    "pyi",
    "pyx",
    "pxd",
    "pyd",
    "pyc",
    "pyo",
    "rpy",
    "ipynb",
    "pyproject",
    "toml",
    "requirements",
    // Go
    "go",
    "mod",
    "sum",
    "go",
    "work",
    "tmpl",
    // C / C++ / 系统
    "c",
    "h",
    "cc",
    "cpp",
    "cxx",
    "c++",
    "hpp",
    "hxx",
    "hh",
    "inl",
    "ipp",
    "tpp",
    "m",
    "mm",
    "swift",
    "d",
    "di",
    "zig",
    "v",
    "nim",
    "pony",
    // Java & JVM
    "java",
    "jav",
    "class",
    "jsh",
    "kt",
    "kts",
    "scala",
    "sc",
    "clj",
    "cljs",
    "edn",
    "groovy",
    "gvy",
    "gy",
    // Rust
    "rs",
    "rlib",
    "depend",
    // .NET / C# / F#
    "cs",
    "csx",
    "fs",
    "fsx",
    "fsi",
    "vb",
    "vbx",
    "xaml",
    // 动态语言
    "rb",
    "erb",
    "rake",
    "gemspec",
    "ru",
    "php",
    "phtml",
    "php3",
    "php4",
    "php5",
    "phps",
    "lua",
    "wlua",
    "moonscript",
    "r",
    "rmd",
    "jl",
    "ex",
    "exs",
    "eex",
    "leex",
    "cr",
    // 数据库 / SQL
    "sql",
    "psql",
    "pgsql",
    "sqlite",
    "sqlitedb",
    "db",
    "ddl",
    "dml",
    "prisma",
    "mongodb",
    "graphql",
    // 日志 & 调试
    "log",
    "trace",
    "out",
    "err",
    "stderr",
    "stdout",
    // 通用文本后备
    "text",
    "ascii",
    "utf8",
    "utf16",
];

// 过滤的类型
pub enum FilterDecision {
    Keep,           // 保留
    ExcludeDir,     // 以目录排除
    ExcludeExt,     // 以后缀名排除
    ExcludeSize,    // 以文件大小排除
    ExcludeNotFile, // 以是否为二进制或者非文件排除
    ExcludeName // 以是否为指定名称排除
}

use std::path::Path as StdPath;

pub struct FilterConfig {
    pub extensions: Vec<String>,
    pub exclude_dirs: Vec<String>,
    pub max_size: u64,
    pub exclude_names: Vec<String>
}
impl FilterConfig {
    // 创建默认配置
    pub fn default() -> Self {
        Self {
            extensions: DEFAULT_EXTENSIONS.iter().map(|s| s.to_string()).collect(),
            exclude_dirs: vec![
                ".git".to_string(),
                "target".to_string(),
                "node_modules".to_string(),
            ],
            max_size: 1024 * 1024, // 1MB
            exclude_names: Vec::new()
        }
    }

    // 检查目录是否应该被跳过（不遍历）
    pub fn should_skip_dir(&self, path: &StdPath) -> bool {
        if !path.is_dir() {
            return false;
        }
        self.exclude_dirs.iter().any(|exclude| {
            if exclude.contains("/") || exclude.contains("\\") {
                let exclude_path = StdPath::new(exclude);
                path.ends_with(exclude_path)
            } else {
                path.components()
                .any(|comp| &comp.as_os_str().to_string_lossy().to_string() == exclude)
            }
        })
    }

    // 判断是否符合要求
    pub fn decide(&self, entry: &walkdir::DirEntry) -> FilterDecision {
        let path = entry.path();
    
        if !entry.file_type().is_file() {
            return FilterDecision::ExcludeNotFile;
        }
    
        // ── 1. 黑名单：按文件名排除（无论有没有扩展名）──
        // ── 黑名单检查（放在最前面）──
        let name = path.file_name()
            .and_then(|n| n.to_str())
    .unwrap_or("");

    if self.exclude_names.iter().any(|x| {
        if x.contains('/') || x.contains('\\') {
        // 子目录文件名匹配，如 "src/readme.md"
        path.ends_with(StdPath::new(x))
        } else {
        // 简单文件名匹配，如 "LICENSE"
        x == name
        }
    }) {
        return FilterDecision::ExcludeName;
    }
    
        // ── 2. 扩展名过滤 ──
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if !self.extensions.is_empty() && !self.extensions.contains(&ext.to_string()) {
                return FilterDecision::ExcludeExt;
            }
        }
    
        // ── 3. 大小过滤 ──
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
