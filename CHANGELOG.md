# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- 并行读取文件（`rayon`），提升大项目扫描性能
- 自定义错误类型 `File2txtError`（`thiserror`）
- 应用层错误处理（`anyhow`）
- 彩色终端输出（`colored`）
- `--debug` 模式，命令行输出调试内容
- `--debug-output` 模式，输出调试文件
- `group_by_top_dir` 和 `get_top_dir_group` 函数
- `FileGroup` 数据结构

### Changed
- 所有 `Box<dyn Error>` 替换为 `File2txtError`（库）或 `anyhow::Result`（应用）
- 统计信息输出增加颜色区分（绿色/红色/紫色/青色）

### Deprecated
- `collect_files`（使用 `collect_files_in` 替代）
- `write_bundle`（使用 `generate_output` 替代）

### Fixed
- 测试用例使用真实临时目录，不再依赖假路径字符串
- `test_should_skip_dir_pure` 现在可独立运行且不污染项目目录

---

## [0.1.6] - 2026-06-01

### Added
- `exclude_names` 字段支持按文件名黑名单排除
- `FilterDecision::ExcludeName` 枚举变体
- `tests/test_filter.rs` 和 `tests/test_output.rs` 单元测试
- `should_skip_dir` 方法支持带路径的排除（如 `src/temp`）

### Changed
- `should_skip_dir` 支持 `ends_with` 匹配子目录路径
- `collect_files_in` 使用 `filter_entry` 跳过目录，而非在 `decide` 里判断

### Fixed
- 目录排除逻辑现在能正确跳过 `./.git/hooks` 等深层子目录

---

## [0.1.5] - 2026-05-31

### Added
- `-p, --path` 参数：指定遍历目录（默认 `.`）
- `-t, --to-path` 参数：指定输出目录（默认与 `--path` 相同）
- GitHub Actions 自动构建和发布 workflow（`.github/workflows/release.yml`）
- README 增加多种安装方式说明

### Changed
- `collect_files` 标记为 deprecated，改用 `collect_files_in`
- 输出路径支持跨目录（`to_path` + `output` 拼接）

---

## [0.1.4] - 2026-05-23

### Added
- 多格式输出支持：
  - `normal`：默认纯文本格式
  - `meta`：带元信息（大小、行数、类型）
  - `markdown`：Markdown 格式，带目录锚点
  - `json`：结构化 JSON 输出
- `serde` 和 `serde_json` 依赖
- `OutputConfig` 和 `OutputFormat` 配置结构
- `generate_output` 统一输出生成入口

### Changed
- `write_bundle` 不再被 main.rs 使用（保留但未标记 deprecated）
- `main.rs` 输出逻辑改用 `generate_output`

---

## [0.1.2] - 2026-05-16

### Added
- `FilterDecision` 枚举（替代 `bool` 返回值）
- `CollectStats` 统计信息结构
- 统计输出：扫描总数、包含文件、排除总数及各类原因细分
- `-d, --exclude_dirs` 参数：支持自定义排除目录
- `-e` 短参数改为 `-e`（原为 `-ext`）

### Changed
- `should_process` 重命名为 `decide`，返回 `FilterDecision`
- `collect_files` 现在返回 `(Vec<File>, CollectStats)` 元组
- `FilterConfig::default()` 增加 `node_modules` 默认排除

---

## [0.1.1] - 2026-05-16

### Added
- `clap` CLI 框架，支持命令行参数：
  - `-o, --output`：指定输出文件名
  - `-e, --ext`：指定扩展名白名单（逗号分隔）
  - `-m, --max-size`：指定最大文件大小（KB）
- `DEFAULT_EXTENSIONS` 导出为 `pub const`，供 CLI 使用
- README.md 文档

### Changed
- `Cargo.toml` 增加 `description`、`license`、`repository`、`readme`、`authors`
- `main.rs` 使用 `clap` 解析参数，不再硬编码

---

## [0.1.0] - 2026-05-15

### Added
- 首个可用版本
- 递归遍历当前目录下所有文件
- 内置常用文本扩展名白名单（约 100+ 种）
- 自动排除 `.git`、`target`、`node_modules` 目录
- 默认文件大小限制 1MB
- 输出格式：`--- filename ---` + 内容
- 依赖：`walkdir = "2"`

---

## 版本标记说明

- `Added`：新增功能
- `Changed`：功能变更（非破坏性）
- `Deprecated`：标记为废弃，将在未来移除
- `Fixed`：问题修复
- `Removed`：移除功能
- `Security`：安全相关修复