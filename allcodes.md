# 代码汇总

## 📊 统计信息

- **扫描总数**: 15
- **包含文件**: 8
- **排除总数**: 7

---

## 📑 目录

1. [.github\workflows\release.yml](#file-0)
2. [allcodes.md](#file-1)
3. [Cargo.toml](#file-2)
4. [readme.md](#file-3)
5. [src\filter_config.rs](#file-4)
6. [src\lib.rs](#file-5)
7. [src\main.rs](#file-6)
8. [src\output_config.rs](#file-7)

---

<div id="file-0"></div>

## 📄 .github\workflows\release.yml

```yml
name: Release

run-name: ${{github.actor}} is learning GitHub Actions

on:
    push:
        tags: 
            - "v*"
        branches: ["main"]
    pull_request:
        branches: ["main"]

permissions:
    contents: write

env:
  CRATE_NAME: file2txt
  CARGO_TERM_COLOR: always

jobs:
  build-and-release:
    name: 构建 ${{matrix.target}}
    runs-on: ${{matrix.os}}
    
    strategy: 
        fail-fast: false
        matrix:
            include:
              - os: ubuntu-latest
                target: x86_64-unknown-linux-gnu
                asset_name: actions-test-linux-x86_64

              - os: macos-latest
                target: x86_64-apple-darwin
                asset_name: actions-test-apple-x86_64

              - os: windows-latest
                target: x86_64-pc-windows-msvc
                asset_name: actions-test-windows-x86_64.exe
    
    steps:
      - name: Checkout
        uses: actions/checkout@v4  # v6 不存在，改回 v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with: 
            targets: ${{matrix.target}}

      - name: ⚡ 缓存 cargo 依赖
        uses: actions/cache@v4
        with:
            path: |
              ~/.cargo/registry
              ~/.cargo/git
              target
            # ↑ 关键：path 的块在这里结束
            key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
            # 建议加上 **/ 来匹配任何位置的 Cargo.lock
            restore-keys: |
              ${{ runner.os }}-cargo-

      - name: Cargo Check
        run: cargo check --target ${{matrix.target}}

      - name: Build Release  # 改名更清晰
        run: cargo build --release --target ${{matrix.target}}  # 改：build 不是 run
    
      - name: 准备产物
        shell: bash
        run: |
          if [ "${{ matrix.os }}" = "windows-latest" ]; then
            SRC="target/${{ matrix.target }}/release/${{ env.CRATE_NAME }}.exe"
          else
            SRC="target/${{ matrix.target }}/release/${{ env.CRATE_NAME }}"
          fi
          cp "$SRC" "${{ matrix.asset_name }}"
          echo "✅ 产物已准备: ${{ matrix.asset_name }}"

      - name: 上传到 Release
        uses: softprops/action-gh-release@v1
        with:
            files: ${{ matrix.asset_name }}
            draft: false
            prerelease: false
        env:
            GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

<div id="file-1"></div>

## 📄 allcodes.md

```md
# 代码汇总

## 📊 统计信息

- **扫描总数**: 12
- **包含文件**: 5
- **排除总数**: 7

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
version = "0.1.5"
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
    pub excluded_by_size: usize,    // 以文件大小排除的文件数
    pub exclude_by_not_file: usize, // 排除的二进制文件或其他不是文件的数量
}
impl Default for CollectStats {
    fn default() -> Self {
        Self {
            all_processed: 0,
            included: 0,
            excluded_by_ext: 0,
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

// 旧方法，从当前目录开始遍历
pub fn collect_files(filter: &FilterConfig) -> Result<(Vec<File>, CollectStats), Box<dyn Error>> {
    collect_files_in(".", filter)
}

// 支持自定义输入目录，用 Walkdir 循环递归目录，返回 Result
pub fn collect_files_in<P>(
    root: P,
    filter: &FilterConfig,
) -> Result<(Vec<File>, CollectStats), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut files = Vec::new();
    let mut stats = CollectStats::default();

    let walkdir = WalkDir::new(root)
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
            _ => {}
        }
    }

    Ok((files, stats))
}

// 将 File 相关信息写入输出文件
// 此函数已不再CLI使用
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
use file2txt::*;
use std::fs;

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
    #[arg(short = 'f', long, default_value = "normal")]
    format: String,

    /// 指定遍历目录，默认为当前目录
    #[arg(short = 'p', long, default_value = ".")]
    path: String,

    /// 指定输出目录，默认在遍历的目录（即 path 目录）
    #[arg(short = 't', long)]
    to_path: Option<String>,
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
        pretty_json: true,
    };

    // 解析过滤后缀
    let extensions = match cli.extensions {
        Some(exts) => exts, // 用户指定了，用用户的
        None => DEFAULT_EXTENSIONS.iter().map(|s| s.to_string()).collect(), // 用户没指定，用默认的
    };

    // 解析过滤目录
    let exclude_dirs = match cli.exclude_dirs {
        Some(exc) => exc,
        None => vec![
            ".git".to_string(),
            "target".to_string(),
            "node_modules".to_string(),
        ],
    };

    // 获得过滤后信息统计
    let filter = FilterConfig {
        extensions,
        exclude_dirs,
        max_size: cli.max_size * 1024,
    };
    let (files, stats) = collect_files_in(&cli.path, &filter)?;

    println!("📊 统计信息:");
    println!(
        "    已跳过目录: {} ({})",
        filter.exclude_dirs.len(),
        filter.exclude_dirs.join(", ")
    );
    println!("    扫描总数: {}", stats.all_processed);
    println!("    包含文件: {}", stats.included);
    println!("    排除总数: {}", stats.all_processed - stats.included);
    println!("    ├─ 扩展名排除: {}", stats.excluded_by_ext);
    println!(
        "    ├─ 大小排除 (>{}KB): {}",
        cli.max_size, stats.excluded_by_size
    );
    println!("    └─ 二进制或非文件: {}", stats.exclude_by_not_file);

    use std::path::Path;
    let output = match cli.to_path {
        Some(path) => path,
        None => cli.path,
    };

    // 构建输出路径：to_path + 基本文件名
    let output_path = Path::new(&output).join(&cli.output);
    let output_path_str = output_path.to_string_lossy().to_string();

    let content = generate_output(&files, &stats, &output_config)?;
    fs::write(&output_path_str, content)?;
    println!("\n✅ 已保存到: {}", output_path_str);

    Ok(())
}
```

<div id="file-4"></div>

## 📄 src\output_config.rs

```rs
use crate::CollectStats;
use crate::File;

use serde_json;
use std::error::Error;
use std::path::Path;

// 文件输出格式
pub enum OutputFormat {
    Normal,   // 默认输出
    Meta,     // 带有文件元信息的输出
    Markdown, // Markdown 格式输出
    Json,     // Json 格式输出
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

pub fn generate_output(
    files: &[File],
    stats: &CollectStats,
    config: &OutputConfig,
) -> Result<String, Box<dyn Error>> {
    match config.format {
        OutputFormat::Normal => Ok(generate_normal_output(files)),
        OutputFormat::Meta => Ok(generate_meta_output(files, stats)),
        OutputFormat::Markdown => Ok(generate_markdown_output(files, stats)),
        OutputFormat::Json => generate_json_output(files, stats, config.pretty_json),
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

fn generate_json_output(
    files: &[File],
    stats: &CollectStats,
    pretty: bool,
) -> Result<String, Box<dyn Error>> {
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
    output.push_str(&format!(
        "- **排除总数**: {}\n\n",
        stats.all_processed - stats.included
    ));
    output.push_str("---\n\n");

    // 目录
    output.push_str("## 📑 目录\n\n");
    for (i, file) in files.iter().enumerate() {
        let display_name = file.name.trim_start_matches(".\\").trim_start_matches("./");
        output.push_str(&format!("{}. [{}](#file-{})\n", i + 1, display_name, i));
    }
    output.push_str("\n---\n\n");

    // 文件内容（使用 HTML div 包裹）
    for (i, file) in files.iter().enumerate() {
        let display_name = file.name.trim_start_matches(".\\").trim_start_matches("./");
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

```

<div id="file-2"></div>

## 📄 Cargo.toml

```toml
[package]
name = "file2txt"
version = "0.1.5"
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

<div id="file-3"></div>

## 📄 readme.md

```md
# File2TXT

将当前目录下的所有文本文件内容聚合到一个文件，方便喂给 AI 分析或代码审查。

## 安装

### 方式一：下载预编译版本（推荐）

1. 访问 [Releases](https://github.com/klp-xkjt/file2txt/releases) 页面
2. 根据你的操作系统下载对应文件：

| 操作系统 | 下载文件 |
|---------|---------|
| Windows (64位) | `file2txt-windows-x86_64.exe` |
| macOS (Intel) | `file2txt-apple-x86_64` |
| Linux (64位) | `file2txt-linux-x86_64` |

3. **（推荐）** 重命名为 `file2txt`（或 `file2txt.exe`）以便使用
4. 将文件放入 PATH 目录

### 方式二：通过 cargo 安装（需要 Rust 环境）

```bash
cargo install file2txt
```

### 方法三：从源码编译
```bash
git clone https://github.com/klp-xkjt/file2txt.git
cd file2txt
cargo build --release
# 可执行文件在 target/release/ 目录下
```

## 快速开始
在任意项目目录下运行：
```bash
file2txt
```

## 命令行选项

| 参数 | 说明 | 示例 |
|------|------|------|
| `-o, --output` | 输出文件名（默认：`output.txt`） | `file2txt -o bundle.txt` |
| `-e, --ext` | 只处理指定扩展名（逗号分隔） | `file2txt -e rs,toml,md` |
| `-m, --max-size` | 最大文件大小，单位 KB（默认：1024） | `file2txt -m 1024` |
| `-d, --exclude_dirs` | 指定排除哪些目录（命令运行的同级目录）(逗号分隔，例如 --exclude_dirs .git,node_modules,target) 不指定时默认排除 .git node_modules target | `file2txt -d target` |
| `-f, --format` | 指定输出文件格式：normal(默认), meta(带有元数据的), markdown(Markdown格式), json(Json格式) | `file2txt -f meta` |
| `-p,  --path` | 指定遍历目录，默认为当前目录| `file2txt -p D:\my_project` |
| `-t,  --to-path` | 指定输出目录，默认在遍历的目录（即 path 目录）| `file2txt -t D:\backup` |


## 使用示例

```bash
# 只收集 Rust 和 Markdown 文件
file2txt -e rs,md

# 限制文件大小，跳过大于 512KB 的
file2txt -m 512

# 自定义输出文件名
file2txt -o analysis.txt

# 指定排除自定义目录
file2txt -d node_modules

# 指定输出文件格式
file2txt -f markdown

# 指定遍历目录
file2txt -p E:\Rust\my_project

# 指定输出目录
file2txt -t D:\backup

# 组合使用
file2txt -o my_code_backup.md -m 512 -e rs,toml,md -f markdown -p E:\Rust\my_project -t D:\backup
```

## 默认行为

- 递归遍历当前目录下的所有文件
- 自动排除 `.git`、`target`、`node_modules` 目录
- 只处理常见文本文件扩展名（除非用 `-e` 指定）
- 跳过大于 1MB 的文件（除非用 `-m` 调整）
- 输出格式默认为 normal
- 读取出错的文件自动跳过，不影响继续执行
- 输出的文件位置默认为指定遍历目录的位置

## 输出格式

### normal
```
--- ./src/main.rs ---
[文件内容]

--- ./src/lib.rs ---
[文件内容]
```

### meta

```
<!--
扫描总数: xxx
包含文件: xxx
排除总数: xxx
-->

// 大小: xxx 字节 | 行数: xxx | 类型: rs
--- ./src/main.rs ---
[文件内容]
```

### json
```
{
    "stats": {
        "all_processed": xxx,
        "included": xxx,
        "excluded_by_ext": xxx,
        "excluded_by_size": xxx,
        "exclude_by_not_file": xxx,
    },
    "files": [{
        "name": "main.rs",
        "content": "[文件内容]",
        "dir": "[文件目录]",
    }]
}
```

### markdown
```
# 代码汇总

## 📊 统计信息

- **扫描总数**: xxx
- **包含文件**: xxx
- **排除总数**: xxx

---

## 📑 目录

1. [src\main.rs](#file-0)
2. ...

---

<div id="file-0"></div>

## 📄 src\main.rs
[文件内容]
```
```

<div id="file-4"></div>

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

<div id="file-5"></div>

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
    pub excluded_by_size: usize,    // 以文件大小排除的文件数
    pub exclude_by_not_file: usize, // 排除的二进制文件或其他不是文件的数量
}
impl Default for CollectStats {
    fn default() -> Self {
        Self {
            all_processed: 0,
            included: 0,
            excluded_by_ext: 0,
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

// 旧方法，从当前目录开始遍历
pub fn collect_files(filter: &FilterConfig) -> Result<(Vec<File>, CollectStats), Box<dyn Error>> {
    collect_files_in(".", filter)
}

// 支持自定义输入目录，用 Walkdir 循环递归目录，返回 Result
pub fn collect_files_in<P>(
    root: P,
    filter: &FilterConfig,
) -> Result<(Vec<File>, CollectStats), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut files = Vec::new();
    let mut stats = CollectStats::default();

    let walkdir = WalkDir::new(root)
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
            _ => {}
        }
    }

    Ok((files, stats))
}

// 将 File 相关信息写入输出文件
// 此函数已不再CLI使用
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

<div id="file-6"></div>

## 📄 src\main.rs

```rs
use clap::Parser;
use file2txt::*;
use std::fs;

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
    #[arg(short = 'f', long, default_value = "normal")]
    format: String,

    /// 指定遍历目录，默认为当前目录
    #[arg(short = 'p', long, default_value = ".")]
    path: String,

    /// 指定输出目录，默认在遍历的目录（即 path 目录）
    #[arg(short = 't', long)]
    to_path: Option<String>,
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
        pretty_json: true,
    };

    // 解析过滤后缀
    let extensions = match cli.extensions {
        Some(exts) => exts, // 用户指定了，用用户的
        None => DEFAULT_EXTENSIONS.iter().map(|s| s.to_string()).collect(), // 用户没指定，用默认的
    };

    // 解析过滤目录
    let exclude_dirs = match cli.exclude_dirs {
        Some(exc) => exc,
        None => vec![
            ".git".to_string(),
            "target".to_string(),
            "node_modules".to_string(),
        ],
    };

    // 获得过滤后信息统计
    let filter = FilterConfig {
        extensions,
        exclude_dirs,
        max_size: cli.max_size * 1024,
    };
    let (files, stats) = collect_files_in(&cli.path, &filter)?;

    println!("📊 统计信息:");
    println!(
        "    已跳过目录: {} ({})",
        filter.exclude_dirs.len(),
        filter.exclude_dirs.join(", ")
    );
    println!("    扫描总数: {}", stats.all_processed);
    println!("    包含文件: {}", stats.included);
    println!("    排除总数: {}", stats.all_processed - stats.included);
    println!("    ├─ 扩展名排除: {}", stats.excluded_by_ext);
    println!(
        "    ├─ 大小排除 (>{}KB): {}",
        cli.max_size, stats.excluded_by_size
    );
    println!("    └─ 二进制或非文件: {}", stats.exclude_by_not_file);

    use std::path::Path;
    let output = match cli.to_path {
        Some(path) => path,
        None => cli.path,
    };

    // 构建输出路径：to_path + 基本文件名
    let output_path = Path::new(&output).join(&cli.output);
    let output_path_str = output_path.to_string_lossy().to_string();

    let content = generate_output(&files, &stats, &output_config)?;
    fs::write(&output_path_str, content)?;
    println!("\n✅ 已保存到: {}", output_path_str);

    Ok(())
}
```

<div id="file-7"></div>

## 📄 src\output_config.rs

```rs
use crate::CollectStats;
use crate::File;

use serde_json;
use std::error::Error;
use std::path::Path;

// 文件输出格式
pub enum OutputFormat {
    Normal,   // 默认输出
    Meta,     // 带有文件元信息的输出
    Markdown, // Markdown 格式输出
    Json,     // Json 格式输出
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

pub fn generate_output(
    files: &[File],
    stats: &CollectStats,
    config: &OutputConfig,
) -> Result<String, Box<dyn Error>> {
    match config.format {
        OutputFormat::Normal => Ok(generate_normal_output(files)),
        OutputFormat::Meta => Ok(generate_meta_output(files, stats)),
        OutputFormat::Markdown => Ok(generate_markdown_output(files, stats)),
        OutputFormat::Json => generate_json_output(files, stats, config.pretty_json),
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

fn generate_json_output(
    files: &[File],
    stats: &CollectStats,
    pretty: bool,
) -> Result<String, Box<dyn Error>> {
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
    output.push_str(&format!(
        "- **排除总数**: {}\n\n",
        stats.all_processed - stats.included
    ));
    output.push_str("---\n\n");

    // 目录
    output.push_str("## 📑 目录\n\n");
    for (i, file) in files.iter().enumerate() {
        let display_name = file.name.trim_start_matches(".\\").trim_start_matches("./");
        output.push_str(&format!("{}. [{}](#file-{})\n", i + 1, display_name, i));
    }
    output.push_str("\n---\n\n");

    // 文件内容（使用 HTML div 包裹）
    for (i, file) in files.iter().enumerate() {
        let display_name = file.name.trim_start_matches(".\\").trim_start_matches("./");
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

