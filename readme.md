# File2TXT

将当前目录下的所有文本文件内容聚合到一个文件，方便喂给 AI 分析或代码审查。

## 安装

### 方式一：下载预编译版本（推荐）

1. 访问 [Releases](https://github.com/klp-xkjt/file2txt/releases) 页面
2. 根据你的操作系统下载对应文件：

| 操作系统 | 下载文件 |
|---------|---------|
| Windows (64位) | `file2txt-windows-x86_64.exe` |
| macOS (Intel) | `file2txt-apple-x86_64` |
| Linux (64位) | `file2txt-linux-x86_64` |

3. **（推荐）** 重命名为 `file2txt`（或 `file2txt.exe`）以便使用
4. 将文件放入 PATH 目录

### 方式二：通过 cargo 安装（需要 Rust 环境）

```bash
cargo install file2txt
```

### 方法三：从源码编译
```bash
git clone https://github.com/klp-xkjt/file2txt.git
cd file2txt
cargo build --release
# 可执行文件在 target/release/ 目录下
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
| `-m, --max-size` | 最大文件大小，单位 KB（默认：1024） | `file2txt -m 1024` |
| `-d, --exclude_dirs` | 指定排除哪些目录（命令运行的同级目录）(逗号分隔，例如 --exclude_dirs .git,node_modules,target) 不指定时默认排除 .git node_modules target | `file2txt -d target` |
| `-f, --format` | 指定输出文件格式：normal(默认), meta(带有元数据的), markdown(Markdown格式), json(Json格式) | `file2txt -f meta` |
| `-p,  --path` | 指定遍历目录，默认为当前目录| `file2txt -p D:\my_project` |
| `-t,  --to-path` | 指定输出目录，默认在遍历的目录（即 path 目录）| `file2txt -t D:\backup` |
| `-n,  --exclude-name` | 指定排除叫哪某个名字的文件 | `file2txt -n Cargo.lock` |

## 使用示例

```bash
# 只收集 Rust 和 Markdown 文件
file2txt -e rs,md

# 限制文件大小，跳过大于 512KB 的
file2txt -m 512

# 自定义输出文件名
file2txt -o analysis.txt

# 指定排除自定义目录，支持排除子目录
file2txt -d node_modules
file2txt -d src/temp

# 指定输出文件格式
file2txt -f markdown

# 指定遍历目录
file2txt -p E:\Rust\my_project

# 指定输出目录
file2txt -t D:\backup

# 指定排除文件，支持排除子目录文件
file2txt -n Cargo.lock
file2txt -d tests/test0.rs

# 组合使用
file2txt -o my_code_backup.md -m 512 -e rs,toml,md -f markdown -p E:\Rust\my_project -t D:\backup -n License
```

## 默认行为

- 递归遍历当前目录下的所有文件
- 自动排除 `.git`、`target`、`node_modules` 目录
- 只处理常见文本文件扩展名（除非用 `-e` 指定）
- 跳过大于 1MB 的文件（除非用 `-m` 调整）
- 输出格式默认为 normal
- 读取出错的文件自动跳过，不影响继续执行
- 输出的文件位置默认为指定遍历目录的位置

## 输出格式

### normal
```
--- ./src/main.rs ---
[文件内容]

--- ./src/lib.rs ---
[文件内容]
```

### meta

```
<!--
扫描总数: xxx
包含文件: xxx
排除总数: xxx
-->

// 大小: xxx 字节 | 行数: xxx | 类型: rs
--- ./src/main.rs ---
[文件内容]
```

### json
```
{
    "stats": {
        "all_processed": xxx,
        "included": xxx,
        "excluded_by_ext": xxx,
        "excluded_by_size": xxx,
        "exclude_by_not_file": xxx,
    },
    "files": [{
        "name": "main.rs",
        "content": "[文件内容]",
        "dir": "[文件目录]",
    }]
}
```

### markdown
```
# 代码汇总

## 📊 统计信息

- **扫描总数**: xxx
- **包含文件**: xxx
- **排除总数**: xxx

---

## 📑 目录

1. [src\main.rs](#file-0)
2. ...

---

<div id="file-0"></div>

## 📄 src\main.rs
[文件内容]
```