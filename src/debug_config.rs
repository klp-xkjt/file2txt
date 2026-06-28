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
