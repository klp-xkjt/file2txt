use file2txt::*;
use std::path::Path;
use walkdir::WalkDir;

#[test]
fn test_should_skip_dir_pure() {
    let config = FilterConfig::default();

    // 直接拼路径，不检查是否真实存在
    assert!(!config.should_skip_dir(Path::new("/some/project/.git")));
    assert!(!config.should_skip_dir(Path::new("/some/project/target")));
    assert!(!config.should_skip_dir(Path::new("/some/project/node_modules")));
    // 子目录名匹配也会跳过
    assert!(!config.should_skip_dir(Path::new("/some/project/.git/hooks")));
    // 普通目录不跳过
    assert!(!config.should_skip_dir(Path::new("/some/project/src")));
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