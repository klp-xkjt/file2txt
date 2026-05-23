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
