# File2TXT

将当前目录下的所有文本文件内容聚合到一个文件，方便喂给 AI 分析或代码审查。

## 安装
```bash
cargo install file2txt
```

## 快速开始
在任意项目目录下运行：
```bash
file2txt
```

## 命令行选项

| 参数 | 说明 | 示例 |
|------|------|------|
| `-o, --output` | 输出文件名（默认：`output.txt`） | `file2txt -o bundle.txt` |
| `-e, --ext` | 只处理指定扩展名（逗号分隔） | `file2txt -e rs,toml,md` |
| `-m, --max-size` | 最大文件大小，单位 KB（默认：1024） | `file2txt -m 500` |

## 使用示例

```bash
# 只收集 Rust 和 Markdown 文件
file2txt -e rs,md

# 限制文件大小，跳过大于 512KB 的
file2txt -m 512

# 自定义输出文件名
file2txt -o analysis.txt

# 指定排除自定义目录
file2txt -d node_modules

# 组合使用
file2txt -o allcode.txt -m 512 -e rs,toml
```

## 默认行为

- 递归遍历当前目录下的所有文件
- 自动排除 `.git`、`target`、`node_modules` 目录
- 只处理常见文本文件扩展名（除非用 `-e` 指定）
- 跳过大于 1MB 的文件（除非用 `-m` 调整）
- 读取出错的文件自动跳过，不影响继续执行

## 输出格式

```
--- ./src/main.rs ---
[文件内容]

--- ./src/lib.rs ---
[文件内容]
```