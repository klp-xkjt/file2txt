use file2txt::{FilterConfig, collect_files, write_bundle, DEFAULT_EXTENSIONS};
use clap::Parser;

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
    #[arg(short='e', long, value_delimiter = ',')]
    extensions: Option<Vec<String>>,

    /// 指定排除哪些目录（命令运行的同级目录）(逗号分隔，例如 --exclude_dirs .git,node_modules,target)
    /// 不指定时默认排除 .git node_modules target
    #[arg(short='d', long, value_delimiter = ',')]
    exclude_dirs: Option<Vec<String>>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let extensions = match cli.extensions {
        Some(exts) => exts,  // 用户指定了，用用户的
        None => DEFAULT_EXTENSIONS.iter().map(|s| s.to_string()).collect(),  // 用户没指定，用默认的
    };
    let exclude_dirs = match cli.exclude_dirs {
        Some(exc) => exc,
        None =>  vec![".git".to_string(), "target".to_string(), "node_modules".to_string()]
    };

    let filter = FilterConfig {
        extensions,
        exclude_dirs,
        max_size: cli.max_size * 1024
    };
    let (files, stats) = collect_files(&filter)?;
    println!("📊 统计信息:");
    println!("   扫描总数: {}", stats.all_processed);
    println!("   包含文件: {}", stats.included);
    println!("   排除总数: {}", stats.all_processed - stats.included);
    println!("   ├─ 目录排除: {}", stats.excluded_by_dir);
    println!("   ├─ 扩展名排除: {}", stats.excluded_by_ext);
    println!("   ├─ 大小排除 (>{}KB): {}", cli.max_size, stats.excluded_by_size);
    println!("   └─ 二进制或非文件: {}",stats.exclude_by_not_file);
    
    let output = cli.output;
    write_bundle(&files, &output)?; 
    Ok(())
}