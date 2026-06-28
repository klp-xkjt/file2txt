# 代码汇总

## 📊 统计信息

- **扫描总数**: 22
- **包含文件**: 13
- **排除总数**: 9

---

## 📑 目录

1. [.github\workflows\ci.yml](#file-0)
2. [.github\workflows\release.yml](#file-1)
3. [.gitignore](#file-2)
4. [Cargo.toml](#file-3)
5. [License](#file-4)
6. [src\debug_config.rs](#file-5)
7. [src\error.rs](#file-6)
8. [src\filter_config.rs](#file-7)
9. [src\lib.rs](#file-8)
10. [src\main.rs](#file-9)
11. [src\output_config.rs](#file-10)
12. [tests\test_filter.rs](#file-11)
13. [tests\test_output.rs](#file-12)

---

<div id="file-0"></div>

## 📄 .github\workflows\ci.yml

```yml
name: CI

on:
  push:
    branches: ["main"]
  pull_request:
    branches: ["main"]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: 测试与检查
    runs-on: ubuntu-latest 
    
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: ⚡ 缓存 cargo 依赖
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-

      - name: cargo check
        run: cargo check

      - name: cargo test
        run: cargo test --release
```

<div id="file-1"></div>

## 📄 .github\workflows\release.yml

```yml
name: Release

on:
  push:
    tags:
      - "v*"  # 只有tags触发，不包含branches

permissions:
  contents: write

env:
  CRATE_NAME: file2txt
  CARGO_TERM_COLOR: always

jobs:
  build-and-release:
    name: 构建 ${{ matrix.target }}
    runs-on: ${{ matrix.os }}
    
    strategy:
      fail-fast: false
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            asset_name: file2txt-linux-x86_64

          - os: macos-latest
            target: x86_64-apple-darwin
            asset_name: file2txt-macos-x86_64

          - os: windows-latest
            target: x86_64-pc-windows-msvc
            asset_name: file2txt-windows-x86_64.exe
    
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: ⚡ 缓存 cargo 依赖
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-

      - name: 构建 Release
        run: cargo build --release --target ${{ matrix.target }}

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

<div id="file-2"></div>

## 📄 .gitignore

```
/target
```

<div id="file-3"></div>

## 📄 Cargo.toml

```toml
[package]
name = "file2txt"
version = "0.1.7"
edition = "2024"
description = "将某一目录下所有文本文件内容聚合到一个文件"
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
rayon = "1.10"
thiserror = "2"
anyhow = { version = "1.0", default-features = false }
colored = "3"
```

<div id="file-4"></div>

## 📄 License

```
Copyright © 2026 <klp-xkjt>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
```

<div id="file-5"></div>

## 📄 src\debug_config.rs

```rs
use crate::*;
use colored::Colorize;
use std::collections::HashMap;

pub struct DebugConfig<'a> {
    pub debug_in_terminal: bool,
    pub debug_output: bool,
    pub files: &'a [File],
    pub stats: &'a CollectStats,
    pub filter: &'a FilterConfig,
    pub root: &'a Path,
}

// 存储所有调试统计数据的结构体
pub struct DebugStats {
    // 1. 目录分组
    pub groups: Vec<FileGroup>,

    // 2. 文件类型分布
    pub file_type_dist: Vec<(String, usize)>, // (扩展名, 数量)
    pub total_files: usize,

    // 3. 代码量统计
    pub total_lines: usize,
    pub total_chars: usize,
    pub avg_lines: usize,
    pub max_lines: usize,
    pub max_file: String,

    // 4. 过滤规则
    pub existing_dirs: Vec<String>,
    pub missing_dirs: Vec<String>,
    pub total_excluded: usize,
}

impl<'a> DebugConfig<'a> {
    fn compute_stats(&self) -> DebugStats {
        // ── 1. 目录分组 ──
        let groups = group_by_top_dir(self.files.to_vec(), self.root);

        // ── 2. 文件类型分布 ──
        let mut file_type: HashMap<String, usize> = HashMap::new();
        for file in self.files {
            let path = Path::new(&file.name);
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("无扩展名");
            *file_type
                .entry(ext.to_string())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let mut file_type_dist: Vec<(String, usize)> = file_type.into_iter().collect();
        file_type_dist.sort_by(|a, b| b.1.cmp(&a.1)); // 按数量降序
        let total_files = self.files.len();

        // ── 3. 代码量统计 ──
        let mut total_lines = 0;
        let mut total_chars = 0;
        let mut max_lines = 0;
        let mut max_file = String::new();

        for file in self.files {
            let lines = file.content.lines().count();
            let chars = file.content.len();

            total_lines += lines;
            total_chars += chars;

            if lines > max_lines {
                max_lines = lines;
                max_file = file.name.clone();
            }
        }

        let avg_lines = if total_files > 0 {
            total_lines / total_files
        } else {
            0
        };

        // ── 4. 过滤规则 ──
        let mut existing_dirs = Vec::new();
        let mut missing_dirs = Vec::new();

        for dir in &self.filter.exclude_dirs {
            let full_path = self.root.join(dir);
            if full_path.exists() && full_path.is_dir() {
                existing_dirs.push(dir.clone());
            } else {
                missing_dirs.push(dir.clone());
            }
        }

        let total_excluded = self.stats.all_processed - self.stats.included;

        DebugStats {
            groups,
            file_type_dist,
            total_files,
            total_lines,
            total_chars,
            avg_lines,
            max_lines,
            max_file,
            existing_dirs,
            missing_dirs,
            total_excluded,
        }
    }

    // 终端输出
    pub fn print_terminal(&self) {
        let stats = self.compute_stats();

        println!("\n{}", "Debug Information:".cyan().bold());

        // 1. 目录分组
        println!("\n{}", "📁 目录分组:".cyan().bold());
        for group in &stats.groups {
            println!(
                "   {} {} {}",
                "📂".bright_blue(),
                group.name.bright_green().bold(),
                format!("({} 个文件)", group.files.len()).bright_black()
            );
        }

        // 2. 文件类型分布
        println!("\n{}", "📊 文件类型分布:".cyan().bold());
        for (ext, count) in &stats.file_type_dist {
            let percentage = (*count as f64 / stats.total_files as f64) * 100.0;
            println!(
                "    {}: {} 个 ({:.1}%)",
                ext.bright_yellow(),
                count,
                percentage
            );
        }

        // 3. 代码量统计
        println!("\n{}", "📈 代码量统计:".cyan().bold());
        println!(
            "    总行数: {}",
            stats.total_lines.to_string().bright_green().bold()
        );
        println!(
            "    总字符数: {}",
            stats.total_chars.to_string().bright_yellow()
        );
        println!(
            "    平均每文件: {} 行",
            stats.avg_lines.to_string().bright_blue()
        );
        println!(
            "    最多行数: {} ({})",
            stats.max_lines.to_string().bright_cyan(),
            stats.max_file.dimmed()
        );

        // 4. 过滤规则
        println!("\n{}", "🚫 过滤规则:".cyan().bold());
        if !stats.existing_dirs.is_empty() {
            println!(
                "    实际跳过的目录: {}",
                stats.existing_dirs.join(", ").bright_blue()
            );
        } else {
            println!("    实际跳过的目录: {}", "无".dimmed());
        }
        if !stats.missing_dirs.is_empty() {
            println!(
                "    未命中的规则: {} {}",
                stats.missing_dirs.join(", ").bright_yellow(),
                "(目录不存在)".dimmed()
            );
        }
        println!(
            "    排除文件总数: {}",
            stats.total_excluded.to_string().bright_red().bold()
        );
        println!(
            "    允许的扩展名: {} 种",
            self.filter.extensions.len().to_string().bright_blue()
        );
        println!(
            "    文件大小限制: {} KB",
            (self.filter.max_size / 1024).to_string().bright_blue()
        );
        if !self.filter.exclude_names.is_empty() {
            println!(
                "    排除文件名: {}",
                self.filter.exclude_names.join(", ").bright_yellow()
            );
        }
    }

    // 文件输出
    fn generate_markdown_content(&self) -> String {
        let stats = self.compute_stats();
        let mut output = String::new();

        // 标题
        output.push_str("# File2TXT 调试报告\n\n");

        // 元信息
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        output.push_str(&format!("> 生成时间：{}\n", now));
        output.push_str(&format!("> 扫描目录：`{}`\n", self.root.display()));
        output.push_str(&format!("> 总文件数：{}\n\n", stats.total_files));
        output.push_str("---\n\n");

        // 1. 目录分组（表格）
        output.push_str("## 📁 目录分组\n\n");
        output.push_str("| 目录 | 文件数 |\n");
        output.push_str("|------|--------|\n");
        for group in &stats.groups {
            output.push_str(&format!("| `{}` | {} |\n", group.name, group.files.len()));
        }
        output.push_str("\n---\n\n");

        // 2. 文件类型分布（表格）
        output.push_str("## 📊 文件类型分布\n\n");
        output.push_str("| 扩展名 | 数量 | 占比 |\n");
        output.push_str("|--------|------|------|\n");
        for (ext, count) in &stats.file_type_dist {
            let percentage = (*count as f64 / stats.total_files as f64) * 100.0;
            output.push_str(&format!("| `{}` | {} | {:.1}% |\n", ext, count, percentage));
        }
        output.push_str("\n---\n\n");

        // 3. 代码量统计（表格）
        output.push_str("## 📈 代码量统计\n\n");
        output.push_str("| 指标 | 数值 |\n");
        output.push_str("|------|------|\n");
        output.push_str(&format!("| 总行数 | {} |\n", stats.total_lines));
        output.push_str(&format!("| 总字符数 | {} |\n", stats.total_chars));
        output.push_str(&format!("| 平均每文件 | {} 行 |\n", stats.avg_lines));
        output.push_str(&format!(
            "| 最多行数 | {} 行 (`{}`) |\n",
            stats.max_lines, stats.max_file
        ));
        output.push_str("\n---\n\n");

        // 4. 过滤规则（表格）
        output.push_str("## 🚫 过滤规则\n\n");
        output.push_str("| 规则 | 值 |\n");
        output.push_str("|------|------|\n");
        output.push_str(&format!(
            "| 实际跳过的目录 | {} |\n",
            if stats.existing_dirs.is_empty() {
                "无".to_string()
            } else {
                stats.existing_dirs.join(", ")
            }
        ));
        output.push_str(&format!(
            "| 未命中的规则 | {} |\n",
            if stats.missing_dirs.is_empty() {
                "无".to_string()
            } else {
                stats.missing_dirs.join(", ")
            }
        ));
        output.push_str(&format!("| 排除文件总数 | {} |\n", stats.total_excluded));
        output.push_str(&format!(
            "| 允许的扩展名 | {} 种 |\n",
            self.filter.extensions.len()
        ));
        output.push_str(&format!(
            "| 文件大小限制 | {} KB |\n",
            self.filter.max_size / 1024
        ));
        if !self.filter.exclude_names.is_empty() {
            output.push_str(&format!(
                "| 排除文件名 | {} |\n",
                self.filter.exclude_names.join(", ")
            ));
        }
        output.push_str("\n---\n\n");

        // 页脚
        output.push_str(&format!(
            "> 报告由 File2TXT v{} 生成\n",
            env!("CARGO_PKG_VERSION")
        ));

        output
    }

    pub fn write_markdown(&self, path: &Path) -> Result<(), File2txtError> {
        let content = self.generate_markdown_content();
        std::fs::write(path, content)?;
        Ok(())
    }
    // 统一入口：根据配置决定执行哪些
    pub fn run(&self) -> Result<(), File2txtError> {
        if self.debug_in_terminal {
            self.print_terminal();
        }
        if self.debug_output {
            // 生成文件名：DEBUG.md 或自定义
            let path = Path::new("DEBUG.md");
            self.write_markdown(path)?;
        }
        Ok(())
    }
}
```

<div id="file-6"></div>

## 📄 src\error.rs

```rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum File2txtError {
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("遍历目录出错: {0}")]
    WalkDir(#[from] walkdir::Error),

    #[error("JSON 序列化出错: {0}")]
    Json(#[from] serde_json::Error),

    #[error("路径错误：{0}")]
    InvalidPath(String),

    #[error("未知的输出格式: '{0}'，支持: normal, meta, markdown, json")]
    UnknownFormat(String),
}
```

<div id="file-7"></div>

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
    ExcludeExt,     // 以后缀名排除
    ExcludeSize,    // 以文件大小排除
    ExcludeNotFile, // 以是否为二进制或者非文件排除
    ExcludeName,    // 以是否为指定名称排除
}

use std::path::Path as StdPath;

pub struct FilterConfig {
    pub extensions: Vec<String>,
    pub exclude_dirs: Vec<String>,
    pub max_size: u64,
    pub exclude_names: Vec<String>,
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
            exclude_names: Vec::new(),
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

        // 1. 黑名单：按文件名排除（无论有没有扩展名）
        // 黑名单检查（放在最前面）
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

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
```

<div id="file-8"></div>

## 📄 src\lib.rs

```rs
mod filter_config;
pub use filter_config::*;
mod output_config;
pub use output_config::*;
mod error;
pub use error::*;
mod debug_config;
pub use debug_config::*;

use rayon::prelude::*;
use std::collections::HashMap;
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
    pub exclude_by_name: usize,     // 以文件名排除的文件数
}
impl Default for CollectStats {
    fn default() -> Self {
        Self {
            all_processed: 0,
            included: 0,
            excluded_by_ext: 0,
            excluded_by_size: 0,
            exclude_by_not_file: 0,
            exclude_by_name: 0,
        }
    }
}

#[derive(Debug, Clone)]
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
    pub fn from_path(path: &Path) -> Result<Self, File2txtError> {
        let name = path.to_string_lossy().to_string();
        let content = fs::read_to_string(path)?;
        let dir = path
            .parent()
            .unwrap_or_else(|| path)
            .to_string_lossy()
            .to_string();
        Ok(Self { name, content, dir })
    }
}

#[derive(Debug)]
pub struct FileGroup {
    pub name: String,
    pub files: Vec<File>,
}

pub fn group_by_top_dir(files: Vec<File>, root: &Path) -> Vec<FileGroup> {
    let mut groups: HashMap<String, Vec<File>> = HashMap::new();

    for file in files {
        let group_name = get_top_dir_group(&file.name, root);
        groups.entry(group_name).or_insert_with(Vec::new).push(file);
    }

    groups
        .into_iter()
        .map(|(name, files)| FileGroup { name, files })
        .collect()
}

pub fn get_top_dir_group<T>(file_path: T, root: &Path) -> String
where
    T: AsRef<Path>,
{
    let path = file_path.as_ref();
    let relative = path.strip_prefix(root).unwrap_or(path);

    let mut components = relative.components().filter_map(|comp| match comp {
        std::path::Component::Normal(s) => Some(s.to_string_lossy().to_string()),
        _ => None,
    });

    let first = components.next();
    let has_more = components.next().is_some();

    match (first, has_more) {
        (Some(dir), true) => dir, // 文件在子目录里，取顶层目录名
        _ => "root".to_string(),  // 文件在项目根目录
    }
}

// 支持自定义输入目录，用 Walkdir 循环递归目录，返回 Result
pub fn collect_files_in<P>(
    root: P,
    filter: &FilterConfig,
) -> Result<(Vec<File>, CollectStats), File2txtError>
where
    P: AsRef<Path>,
{
    let root_path = root.as_ref();
    if !root_path.exists() {
        return Err(File2txtError::InvalidPath(format!(
            "目录不存在: {}",
            root_path.display()
        )));
    }
    if !root_path.is_dir() {
        return Err(File2txtError::InvalidPath(format!(
            "路径不是目录: {}",
            root_path.display()
        )));
    }

    let mut entries_to_process = Vec::new();
    let mut stats = CollectStats::default();

    let walkdir = WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !filter.should_skip_dir(e.path()));

    for entry in walkdir {
        let entry = entry?;
        stats.all_processed += 1;

        match filter.decide(&entry) {
            FilterDecision::Keep => {
                entries_to_process.push(entry.path().to_path_buf());
                stats.included += 1;
            }
            FilterDecision::ExcludeExt => stats.excluded_by_ext += 1,
            FilterDecision::ExcludeSize => stats.excluded_by_size += 1,
            FilterDecision::ExcludeNotFile => stats.exclude_by_not_file += 1,
            FilterDecision::ExcludeName => stats.exclude_by_name += 1,
        }
    }

    let mut files: Vec<File> = entries_to_process
        .par_iter()
        .filter_map(|path| File::from_path(path).ok())
        .collect();
    files.sort_by(|a, b| a.name.cmp(&b.name));

    Ok((files, stats))
}

//  旧方法，从当前目录开始遍历
#[deprecated]
pub fn collect_files(filter: &FilterConfig) -> Result<(Vec<File>, CollectStats), File2txtError> {
    collect_files_in(".", filter)
}

// 将 File 相关信息写入输出文件
#[deprecated]
pub fn write_bundle(files: &[File], output_path: &str) -> Result<(), File2txtError> {
    let mut output = fs::File::create(output_path)?;

    for file in files {
        writeln!(output, "--- {} ---", file.name)?;
        writeln!(output, "{}", file.content)?;
        writeln!(output)?;
    }

    Ok(())
}
```

<div id="file-9"></div>

## 📄 src\main.rs

```rs
use anyhow::Context;
use clap::Parser;
use colored::Colorize;
use file2txt::*;
use std::fs;
use std::path::Path;
use std::time::Instant;

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

    /// 指定排除哪些目录（支持排除同级目录和子目录）(逗号分隔，例如 --exclude_dirs .git,node_modules,target)
    /// 不指定时默认排除 .git node_modules target
    #[arg(short = 'd', long, value_delimiter = ',')]
    exclude_dirs: Option<Vec<String>>,

    /// 指定排除叫哪某个名字的文件（逗号分隔，例如 -n License,Makefile）
    #[arg(short = 'n', long, value_delimiter = ',')]
    exclude_name: Option<Vec<String>>,

    /// 指定输出文件格式：normal(默认), meta(带有元数据的), markdown(Markdown格式), json(Json格式)
    #[arg(short = 'f', long, default_value = "normal")]
    format: String,

    /// 指定遍历目录，默认为当前目录
    #[arg(short = 'p', long, default_value = ".")]
    path: String,

    /// 指定输出目录，默认在遍历的目录（即 path 目录）
    #[arg(short = 't', long)]
    to_path: Option<String>,

    /// Debug in Terminal
    #[arg(long, default_value = "false")]
    debug: bool,

    /// Debug in File
    #[arg(long, default_value = "false")]
    debug_output: bool,
}

fn main() -> anyhow::Result<()> {
    let start = Instant::now();
    let cli = Cli::parse();
    let path = cli.path.clone();

    // 解析输出格式
    let format = match cli.format.as_str() {
        "normal" => OutputFormat::Normal,
        "meta" => OutputFormat::Meta,
        "markdown" | "md" => OutputFormat::Markdown,
        "json" => OutputFormat::Json,
        other => return Err(File2txtError::UnknownFormat(other.into()).into()),
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

    // 解析过滤文件名
    let exclude_name = match cli.exclude_name {
        Some(x) => x,
        None => Vec::new(),
    };

    // 获得过滤后信息统计
    let filter = FilterConfig {
        extensions,
        exclude_dirs,
        max_size: cli.max_size * 1024,
        exclude_names: exclude_name,
    };

    // 输出
    println!("{}", "🔍 正在扫描文件...".cyan().bold());
    let (files, stats) = collect_files_in(&cli.path, &filter).context("收集文件时出错")?;

    println!("\n{}", "📊 统计信息:".green().bold());
    println!(
        "    {} {}",
        "已跳过目录:".bright_yellow().bold(),
        format!("[{}]", filter.exclude_dirs.join(", ")).bright_blue()
    );
    println!(
        "    {} {}",
        "扫描总数:".bright_yellow().bold(),
        stats.all_processed.to_string().bright_cyan()
    );
    println!(
        "    {} {}",
        "包含文件:".bright_yellow().bold(),
        stats.included.to_string().bright_green().bold()
    );
    println!(
        "    {} {}",
        "排除总数:".bright_yellow().bold(),
        (stats.all_processed - stats.included)
            .to_string()
            .bright_red()
    );
    println!(
        "    │  ├─ 扩展名排除: {}",
        stats.excluded_by_ext.to_string().bright_purple()
    );
    println!(
        "    │  ├─ 大小排除 (>{}KB): {}",
        cli.max_size.to_string().bright_cyan(),
        stats.excluded_by_size.to_string().bright_purple()
    );
    println!(
        "    │  ├─ 名称排除: {}",
        stats.exclude_by_name.to_string().bright_purple()
    );
    println!(
        "    │  └─ 二进制或非文件: {}",
        stats.exclude_by_not_file.to_string().bright_purple()
    );

    // 如果包含文件数为 0，给出警告
    if stats.included == 0 {
        println!(
            "\n{}",
            "⚠️  警告: 没有找到符合条件的文件！".bright_red().bold()
        );
    }

    let output = match cli.to_path {
        Some(path) => path,
        None => path,
    };

    // 构建输出路径：to_path + 基本文件名
    let output_path = Path::new(&output).join(&cli.output);
    let output_path_str = output_path.to_string_lossy().to_string();

    let content = generate_output(&files, &stats, &output_config).context("生成输出内容时出错")?;
    fs::write(&output_path_str, content).context("写入输出文件失败")?;
    println!("\n✅ 已保存到: {}", output_path_str);
    println!("⏱ 耗时: {:.3?}", start.elapsed());

    let debug = DebugConfig {
        debug_in_terminal: cli.debug,
        debug_output: cli.debug_output,
        files: &files,
        stats: &stats,
        filter: &filter,
        root: Path::new(&cli.path),
    };

    debug.run()?;

    println!("\n🦀 File2TXT v{} · Made by klp-xkjt", env!("CARGO_PKG_VERSION"));
    println!("🔗 {}", env!("CARGO_PKG_REPOSITORY"));
    Ok(())
}
```

<div id="file-10"></div>

## 📄 src\output_config.rs

```rs
use crate::CollectStats;
use crate::File;
use crate::error::File2txtError;

use serde_json;
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
) -> Result<String, File2txtError> {
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
        output.push_str("BY File2TXT");
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
    output.push_str("BY File2TXT");
    output
}

fn generate_json_output(
    files: &[File],
    stats: &CollectStats,
    pretty: bool,
) -> Result<String, File2txtError> {
    let data = serde_json::json!({
        "stats": {
            "all_processed": stats.all_processed,
            "included": stats.included,
            "excluded_by_ext": stats.excluded_by_ext,
            "excluded_by_size": stats.excluded_by_size,
            "exclude_by_not_file": stats.exclude_by_not_file,
            "exclude_by_name": stats.exclude_by_name
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
    output.push_str("BY File2TXT");

    output
}
```

<div id="file-11"></div>

## 📄 tests\test_filter.rs

```rs
use file2txt::*;
use std::fs;
use walkdir::WalkDir;

#[test]
fn test_should_skip_dir_pure() {
    let config = FilterConfig::default();

    let tmp = std::env::temp_dir().join("file2txt_test_skip");
    let _ = fs::remove_dir_all(&tmp); // 清理上次残留
    fs::create_dir_all(tmp.join(".git/hooks")).unwrap();
    fs::create_dir_all(tmp.join("target")).unwrap();
    fs::create_dir_all(tmp.join("node_modules")).unwrap();
    fs::create_dir_all(tmp.join("src")).unwrap();

    // 这些应该被跳过
    assert!(config.should_skip_dir(&tmp.join(".git")));
    assert!(config.should_skip_dir(&tmp.join(".git/hooks")));
    assert!(config.should_skip_dir(&tmp.join("target")));
    assert!(config.should_skip_dir(&tmp.join("node_modules")));
    // 普通目录不跳过
    assert!(!config.should_skip_dir(&tmp.join("src")));

    // 清理
    fs::remove_dir_all(&tmp).unwrap();
}

#[test]
fn test_decide_using_current_project() {
    let config = FilterConfig::default();

    let entry = WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .find(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
        .expect("当前项目没有 .rs 文件，换个路径");
    // 应该 Keep
    assert!(matches!(config.decide(&entry), FilterDecision::Keep));
}
```

<div id="file-12"></div>

## 📄 tests\test_output.rs

```rs
use file2txt::*;

// 创建File向量
fn make_test_files() -> Vec<File> {
    vec![
        File::new("src/main.rs".into(), "fn main() {}".into(), "src".into()),
        File::new("readme.md".into(), "# Hello".into(), ".".into()),
    ]
}

// 创建统计数据
fn make_test_stats() -> CollectStats {
    CollectStats {
        all_processed: 10,
        included: 2,
        excluded_by_ext: 5,
        excluded_by_size: 2,
        exclude_by_not_file: 1,
        exclude_by_name: 0,
    }
}

// 测试
// 测试Normal输出
#[test]
fn test_normal_output_contains_content() {
    let files = make_test_files();
    let stats = make_test_stats();
    let config = OutputConfig {
        format: OutputFormat::Normal,
        pretty_json: false,
    };
    let out = generate_output(&files, &stats, &config).unwrap();

    assert!(out.contains("--- src/main.rs ---"));
    assert!(out.contains("fn main() {}"));
    assert!(out.contains("--- readme.md ---"));
    assert!(out.contains("# Hello"));
}

// 测试Meta输出
#[test]
fn test_meta_output_includes_stats() {
    let files = make_test_files();
    let stats = make_test_stats();
    let config = OutputConfig {
        format: OutputFormat::Meta,
        pretty_json: false,
    };
    let out = generate_output(&files, &stats, &config).unwrap();

    assert!(out.contains("扫描总数: 10"));
    assert!(out.contains("包含文件: 2"));
    assert!(out.contains("排除总数: 8"));
}

// 测试JSON输出
#[test]
fn test_json_output_format() {
    let files = make_test_files();
    let stats = make_test_stats();
    let config = OutputConfig {
        format: OutputFormat::Json,
        pretty_json: false,
    };
    let out = generate_output(&files, &stats, &config).unwrap();

    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["stats"]["all_processed"], 10);
    assert_eq!(v["files"].as_array().unwrap().len(), 2);
    assert_eq!(v["files"][0]["name"], "src/main.rs");
}

// 测试Markdown输出
#[test]
fn test_markdown_output_contains_anchors() {
    let files = make_test_files();
    let stats = make_test_stats();
    let config = OutputConfig {
        format: OutputFormat::Markdown,
        pretty_json: false,
    };
    let out = generate_output(&files, &stats, &config).unwrap();

    assert!(out.contains("# 代码汇总"));
    assert!(out.contains("## 📊 统计信息"));
    assert!(out.contains(r#"<div id="file-0"></div>"#));
    assert!(out.contains("```rs"));
}
```

BY File2TXT