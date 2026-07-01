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
    let exclude_name = cli.exclude_name.unwrap_or_default();

    // 获得过滤后信息统计
    let filter = FilterConfig {
        extensions,
        exclude_dirs,
        max_size: cli.max_size * 1024,
        exclude_names: exclude_name,
    };

    // 输出
    println!("{}", "🔍 File2TXT正在扫描文件...".cyan().bold());
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
    println!("⏰ 耗时: {:.3?}", start.elapsed());

    let debug = DebugConfig {
        debug_in_terminal: cli.debug,
        debug_output: cli.debug_output,
        files: &files,
        stats: &stats,
        filter: &filter,
        root: Path::new(&cli.path),
    };

    debug.run()?;

    println!(
        "\n🦀 File2TXT v{} · Made by klp-xkjt",
        env!("CARGO_PKG_VERSION")
    );
    println!("🔗 {}", env!("CARGO_PKG_REPOSITORY"));
    Ok(())
}
