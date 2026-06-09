use file2txt::*;
use std::fs;
use walkdir::WalkDir;

#[test]
fn test_should_skip_dir_pure() {
    let config = FilterConfig::default();

    let tmp = std::env::temp_dir().join("file2txt_test_skip");
    let _ = fs::remove_dir_all(&tmp); // 清理上次残留
    fs::create_dir_all(tmp.join(".git/hooks")).unwrap();
    fs::create_dir_all(tmp.join("target")).unwrap();
    fs::create_dir_all(tmp.join("node_modules")).unwrap();
    fs::create_dir_all(tmp.join("src")).unwrap();

    // 这些应该被跳过
    assert!(config.should_skip_dir(&tmp.join(".git")));
    assert!(config.should_skip_dir(&tmp.join(".git/hooks")));
    assert!(config.should_skip_dir(&tmp.join("target")));
    assert!(config.should_skip_dir(&tmp.join("node_modules")));
    // 普通目录不跳过
    assert!(!config.should_skip_dir(&tmp.join("src")));

    // 清理
    fs::remove_dir_all(&tmp).unwrap();
}

#[test]
fn test_decide_using_current_project() {
    let config = FilterConfig::default();

    let entry = WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .find(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
        .expect("当前项目没有 .rs 文件，换个路径");
    // 应该 Keep
    assert!(matches!(config.decide(&entry), FilterDecision::Keep));
}
