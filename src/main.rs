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
