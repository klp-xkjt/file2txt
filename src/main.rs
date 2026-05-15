use file2txt::File;
use walkdir::WalkDir;
use std::io::Write;
use std::fs::File as StdFile;
use file2txt::FilterConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut output = StdFile::create("output.txt")?;
    let mut files: Vec<File> = Vec::new();
    let filter = FilterConfig::default();

    for entry in WalkDir::new(".") {
        let entry = entry?;
        if filter.should_process(&entry) {
            if let Ok(file) = File::from_path(entry.path()) {
                files.push(file);
            }
        }
    }
    println!("找到{}个文件", files.len());
    for file in files.iter() {
        writeln!(output, "--- {} ---", file.name)?;
        writeln!(output, "{}", file.content)?;
        writeln!(output)?;
    }
    
    Ok(())
}