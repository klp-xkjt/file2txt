use file2txt::*;

// 创建File向量
fn make_test_files() -> Vec<File> {
    vec![
        File::new("src/main.rs".into(), "fn main() {}".into(), "src".into()),
        File::new("readme.md".into(), "# Hello".into(), ".".into()),
    ]
}

// 创建统计数据
fn make_test_stats() -> CollectStats {
    CollectStats {
        all_processed: 10,
        included: 2,
        excluded_by_ext: 5,
        excluded_by_size: 2,
        exclude_by_not_file: 1,
        exclude_by_name: 0
    }
}

// 测试
// 测试Normal输出
#[test]
fn test_normal_output_contains_content() {
    let files = make_test_files();
    let stats = make_test_stats();
    let config = OutputConfig { format: OutputFormat::Normal, pretty_json: false };
    let out = generate_output(&files, &stats, &config).unwrap();

    assert!(out.contains("--- src/main.rs ---"));
    assert!(out.contains("fn main() {}"));
    assert!(out.contains("--- readme.md ---"));
    assert!(out.contains("# Hello"));
}

// 测试Meta输出
#[test]
fn test_meta_output_includes_stats() {
    let files = make_test_files();
    let stats = make_test_stats();
    let config = OutputConfig { format: OutputFormat::Meta, pretty_json: false };
    let out = generate_output(&files, &stats, &config).unwrap();

    assert!(out.contains("扫描总数: 10"));
    assert!(out.contains("包含文件: 2"));
    assert!(out.contains("排除总数: 8")); 
}

// 测试JSON输出
#[test]
fn test_json_output_format() {
    let files = make_test_files();
    let stats = make_test_stats();
    let config = OutputConfig { format: OutputFormat::Json, pretty_json: false };
    let out = generate_output(&files, &stats, &config).unwrap();

    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["stats"]["all_processed"], 10);
    assert_eq!(v["files"].as_array().unwrap().len(), 2);
    assert_eq!(v["files"][0]["name"], "src/main.rs");
}

// 测试Markdown输出
#[test]
fn test_markdown_output_contains_anchors() {
    let files = make_test_files();
    let stats = make_test_stats();
    let config = OutputConfig { format: OutputFormat::Markdown, pretty_json: false };
    let out = generate_output(&files, &stats, &config).unwrap();

    assert!(out.contains("# 代码汇总"));
    assert!(out.contains("## 📊 统计信息"));
    assert!(out.contains(r#"<div id="file-0"></div>"#));
    assert!(out.contains("```rs"));
}