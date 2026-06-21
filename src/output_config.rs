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
) -> Result<String, File2txtError> {
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
