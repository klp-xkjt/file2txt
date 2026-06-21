mod filter_config;
pub use filter_config::*;
mod output_config;
pub use output_config::*;
mod error;
pub use error::*;

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

    let files: Vec<File> = entries_to_process
        .par_iter()
        .filter_map(|path| File::from_path(path).ok())
        .collect();

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
