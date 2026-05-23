# 代码汇总

## 📊 统计信息

- **扫描总数**: 11
- **包含文件**: 5
- **排除总数**: 6

---

## 📑 目录

1. [Cargo.toml](#file-0)
2. [src\filter_config.rs](#file-1)
3. [src\lib.rs](#file-2)
4. [src\main.rs](#file-3)
5. [src\output_config.rs](#file-4)

---

<div id="file-0"></div>

## 📄 Cargo.toml

```toml
[package]
name = "file2txt"
version = "0.1.4"
edition = "2024"
description = "将目录下所有文本文件内容聚合到一个文件"
license = "MIT"
repository = "https://github.com/klp-xkjt/file2txt"
readme = "readme.md"
authors = ["klp-xkjt <xkjt-tnt@outlook.com>"]

[[bin]]
name = "file2txt"
path = "src/main.rs"

[dependencies]
walkdir = "2"
clap = { version = "4.6", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

<div id="file-1"></div>

## 📄 src\filter_config.rs

```rs
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
}

use std::path::Path as StdPath;

pub struct FilterConfig {
    pub extensions: Vec<String>,
    pub exclude_dirs: Vec<String>,
    pub max_size: u64,
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
        }
    }

    // 检查目录是否应该被跳过（不遍历）
    pub fn should_skip_dir(&self, path: &StdPath) -> bool {
        if !path.is_dir() {
            return false;
        }
        self.exclude_dirs.iter().any(|exclude| {
            path.components()
                .any(|comp| &comp.as_os_str().to_string_lossy().to_string() == exclude)
        })
    }

    // 判断是否符合要求
    pub fn decide(&self, entry: &walkdir::DirEntry) -> FilterDecision {
        let path = entry.path();

        // 判断是否为路径并排除
        if !entry.file_type().is_file() {
            return FilterDecision::ExcludeNotFile;
        }

        // 已废除，请使用FilterConfig的should_skip_dir方法。
        // 判断是否为被忽略的目录并排除其中文件
        // for exclude in &self.exclude_dirs {
        //     if path.components().any(|e| &e.as_os_str().to_string_lossy().to_string() == exclude) {
        //         return FilterDecision::ExcludeDir;
        //     }
        // }

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
```

<div id="file-2"></div>

## 📄 src\lib.rs

```rs
mod filter_config;
pub use filter_config::*;
mod output_config;
pub use output_config::*;

use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct CollectStats {
    pub all_processed: usize,       // 总扫描的数量
    pub included: usize,            // 最终包含的文件数量
    pub excluded_by_ext: usize,     // 以后缀排除的文件数
    pub excluded_by_dir: usize,     // 以目录排除的文件数
    pub excluded_by_size: usize,    // 以文件大小排除的文件数
    pub exclude_by_not_file: usize, // 排除的二进制文件或其他不是文件的数量
}
impl Default for CollectStats {
    fn default() -> Self {
        Self {
            all_processed: 0,
            included: 0,
            excluded_by_ext: 0,
            excluded_by_dir: 0,
            excluded_by_size: 0,
            exclude_by_not_file: 0,
        }
    }
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub content: String,
    pub dir: String,
}
impl File {
    pub fn new(name: String, content: String, dir: String) -> Self {
        Self { name, content, dir }
    }
    // 通过文件路径来创建 File
    pub fn from_path(path: &Path) -> Result<Self, Box<dyn Error>> {
        let name = path.to_string_lossy().to_string();
        // let content = fs::read_to_string(path)?;
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => return Err(Box::new(e)),
        };
        let dir = path
            .parent()
            .unwrap_or_else(|| path)
            .to_string_lossy()
            .to_string();
        Ok(Self { name, content, dir })
    }
}

// 用 Walkdir 循环递归目录，返回 Result
pub fn collect_files(filter: &FilterConfig) -> Result<(Vec<File>, CollectStats), Box<dyn Error>> {
    let mut files = Vec::new();
    let mut stats = CollectStats::default();

    let walkdir = WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| !filter.should_skip_dir(e.path()));

    for entry in walkdir {
        let entry: walkdir::DirEntry = entry?;
        stats.all_processed += 1;

        match filter.decide(&entry) {
            FilterDecision::Keep => {
                if let Ok(file) = File::from_path(entry.path()) {
                    files.push(file);
                    stats.included += 1;
                }
            }
            FilterDecision::ExcludeExt => {
                stats.excluded_by_ext += 1;
            }
            FilterDecision::ExcludeSize => {
                stats.excluded_by_size += 1;
            }
            FilterDecision::ExcludeNotFile => {
                stats.exclude_by_not_file += 1;
            }
            _ => {} // ExcludeDir 不会出现
        }
    }

    Ok((files, stats))
}

// 将 File 相关信息写入输出文件
pub fn write_bundle(files: &[File], output_path: &str) -> Result<(), Box<dyn Error>> {
    let mut output = fs::File::create(output_path)?;

    for file in files {
        writeln!(output, "--- {} ---", file.name)?;
        writeln!(output, "{}", file.content)?;
        writeln!(output)?;
    }

    Ok(())
}
```

<div id="file-3"></div>

## 📄 src\main.rs

```rs
use clap::Parser;
use std::fs;
use file2txt::{collect_files, generate_output, CollectStats, File, FilterConfig, OutputConfig, OutputFormat, DEFAULT_EXTENSIONS};

#[derive(Parser)]
struct Cli {
    /// 指定输出文件 默认是 output.txt
    #[arg(short, long, default_value = "output.txt")]
    output: String,

    /// 指定文件最大大小（KB），默认 1024
    #[arg(short, long, default_value = "1024")]
    max_size: u64,

    /// 指定保留哪些后缀名文件（逗号分隔，例如：-e rs,toml,md）
    /// 不指定时使用内置的常用扩展名列表
    #[arg(short = 'e', long, value_delimiter = ',')]
    extensions: Option<Vec<String>>,

    /// 指定排除哪些目录（命令运行的同级目录）(逗号分隔，例如 --exclude_dirs .git,node_modules,target)
    /// 不指定时默认排除 .git node_modules target
    #[arg(short = 'd', long, value_delimiter = ',')]
    exclude_dirs: Option<Vec<String>>,

    /// 指定输出文件格式：normal(默认), meta(带有元数据的), markdown(Markdown格式), json(Json格式)
    #[arg(short='f', long, default_value = "normal")]
    format: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // 解析输出格式
    let format = match cli.format.as_str() {
        "normal" => OutputFormat::Normal,
        "meta" => OutputFormat::Meta,
        "markdown" | "md" => OutputFormat::Markdown,
        "json" => OutputFormat::Json,
        _ => {
            eprintln!("警告: 未知格式 '{}'，使用默认格式 normal", cli.format);
            OutputFormat::Normal
        }
    };

    let output_config = OutputConfig {
        format,
        pretty_json: true
    };

    let extensions = match cli.extensions {
        Some(exts) => exts, // 用户指定了，用用户的
        None => DEFAULT_EXTENSIONS.iter().map(|s| s.to_string()).collect(), // 用户没指定，用默认的
    };
    let exclude_dirs = match cli.exclude_dirs {
        Some(exc) => exc,
        None => vec![
            ".git".to_string(),
            "target".to_string(),
            "node_modules".to_string(),
        ],
    };

    let filter = FilterConfig {
        extensions,
        exclude_dirs,
        max_size: cli.max_size * 1024,
    };
    let (files, stats): (Vec<File>, CollectStats) = collect_files(&filter)?;
    println!("📊 统计信息:");
    println!("    扫描总数: {}", stats.all_processed);
    println!("    包含文件: {}", stats.included);
    println!("    排除总数: {}", stats.all_processed - stats.included);
    println!(
        "    ├─ 已跳过目录: {} ({})",
        filter.exclude_dirs.len(),
        filter.exclude_dirs.join(", ")
    );
    println!("    ├─ 扩展名排除: {}", stats.excluded_by_ext);
    println!(
        "    ├─ 大小排除 (>{}KB): {}",
        cli.max_size, stats.excluded_by_size
    );
    println!("    └─ 二进制或非文件: {}", stats.exclude_by_not_file);


    let content = generate_output(&files, &stats, &output_config)?;
    fs::write(&cli.output, content)?;
    
    println!("\n✅ 已保存到: {}", cli.output);
    Ok(())
}
```

<div id="file-4"></div>

## 📄 src\output_config.rs

```rs
use crate::CollectStats;
use crate::File;

use serde::Serialize;
use serde_json;
use std::path::Path;
use std::error::Error;

// 文件输出格式
pub enum OutputFormat {
    Normal,   // 默认输出
    Meta,     // 带有文件元信息的输出
    Markdown, // Markdown 格式输出
    Json,     // Json 格式输出
}

// 文件元信息
#[derive(Debug, Serialize)]
pub struct FileMeta {
    pub name: String,    // 文件路径
    pub content: String, // 文件内容
    pub size: usize,     // 文件大小（字节）
    pub lines: usize,    // 行数
    pub ext: String,     // 扩展名
}

pub struct OutputConfig {
    pub format: OutputFormat,
    pub pretty_json: bool, // JSON 是否格式化
}

impl Default for OutputConfig {
    fn default() -> Self {
        OutputConfig {
            format: OutputFormat::Normal,
            pretty_json: true,
        }
    }
}

pub fn generate_output(files: &[File], stats: &CollectStats, config: &OutputConfig) -> Result<String, Box<dyn Error>> {
    match config.format {
        OutputFormat::Normal => Ok(generate_normal_output(files)),
        OutputFormat::Meta => Ok(generate_meta_output(files, stats)),
        OutputFormat::Markdown => Ok(generate_markdown_output(files, stats)),
        OutputFormat::Json => generate_json_output(files, stats, config.pretty_json)
    }
}

fn generate_normal_output(files: &[File]) -> String {
    let mut output = String::new();
    for file in files {
        output.push_str(&format!("--- {} ---\n", file.name));
        output.push_str(&file.content);
        output.push_str("\n\n");
    }

    output
}

fn generate_meta_output(files: &[File], stats: &CollectStats) -> String {
    let mut output = String::new();

    // 添加统计信息头
    output.push_str(&format!("<!--\n"));
    output.push_str(&format!("扫描总数: {}\n", stats.all_processed));
    output.push_str(&format!("包含文件: {}\n", stats.included));
    output.push_str(&format!(
        "排除总数: {}\n",
        stats.all_processed - stats.included
    ));
    output.push_str(&format!("-->\n\n"));

    for file in files {
        let path = Path::new(&file.name);
        let size = file.content.len();
        let lines = file.content.lines().count();
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("none");
        output.push_str(&format!(
            "// 大小: {} 字节 | 行数: {} | 类型: {}\n",
            size, lines, ext
        ));
        output.push_str(&format!("--- {} ---\n", file.name));
        
        output.push_str(&file.content);
        output.push_str("\n\n");
    }

    output
}

fn generate_json_output(files: &[File], stats: &CollectStats, pretty: bool) -> Result<String, Box<dyn Error>> {
    let data = serde_json::json!({
        "stats": {
            "all_processed": stats.all_processed,
            "included": stats.included,
            "excluded_by_ext": stats.excluded_by_ext,
            "excluded_by_size": stats.excluded_by_size,
            "exclude_by_not_file": stats.exclude_by_not_file,
        },
        "files": files.iter().map(|file| {
            serde_json::json!({
                "name": file.name,
                "content": file.content,
                "dir": file.dir,
            })
        }).collect::<Vec<_>>()
    });

    if pretty {
        Ok(serde_json::to_string_pretty(&data)?)
    } else {
        Ok(serde_json::to_string(&data)?)
    }
}

fn generate_markdown_output(files: &[File], stats: &CollectStats) -> String {
    let mut output = String::new();
    
    // 标题
    output.push_str("# 代码汇总\n\n");
    
    // 统计信息
    output.push_str("## 📊 统计信息\n\n");
    output.push_str(&format!("- **扫描总数**: {}\n", stats.all_processed));
    output.push_str(&format!("- **包含文件**: {}\n", stats.included));
    output.push_str(&format!("- **排除总数**: {}\n\n", stats.all_processed - stats.included));
    output.push_str("---\n\n");
    
    // 目录
    output.push_str("## 📑 目录\n\n");
    for (i, file) in files.iter().enumerate() {
        let display_name = file.name
            .trim_start_matches(".\\")
            .trim_start_matches("./");
        output.push_str(&format!("{}. [{}](#file-{})\n", i + 1, display_name, i));
    }
    output.push_str("\n---\n\n");
    
    // 文件内容（使用 HTML div 包裹）
    for (i, file) in files.iter().enumerate() {
        let display_name = file.name
            .trim_start_matches(".\\")
            .trim_start_matches("./");
        let ext = Path::new(&file.name)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        
        // 使用 div 作为锚点容器
        output.push_str(&format!("<div id=\"file-{}\"></div>\n\n", i));
        output.push_str(&format!("## 📄 {}\n\n", display_name));
        output.push_str(&format!("```{}\n", ext));
        output.push_str(&file.content);
        if !file.content.ends_with('\n') {
            output.push('\n');
        }
        output.push_str("```\n\n");
    }
    
    output
}
```

