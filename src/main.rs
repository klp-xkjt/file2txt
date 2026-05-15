use file2txt::{FilterConfig, collect_files, write_bundle};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filter = FilterConfig::default();
    
    let files = collect_files(&filter)?;
    println!("找到 {} 个文件", files.len());
    
    write_bundle(&files, "output.txt")?; 
    
    Ok(())
}