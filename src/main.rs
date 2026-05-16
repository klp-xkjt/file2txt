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
    #[arg(short, long, value_delimiter = ',')]
    ext: Option<Vec<String>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let extensions = match cli.ext {
        Some(exts) => exts,  // 用户指定了，用用户的
        None => DEFAULT_EXTENSIONS.iter().map(|s| s.to_string()).collect(),  // 用户没指定，用默认的
    };

    let filter = FilterConfig {
        extensions,
        exclude_dirs: vec![".git".to_string(), "target".to_string()],
        max_size: cli.max_size * 1024
    };
    let files = collect_files(&filter)?;
    println!("找到 {} 个文件", files.len());
    
    let output = cli.output;
    write_bundle(&files, &output)?; 
    Ok(())
}