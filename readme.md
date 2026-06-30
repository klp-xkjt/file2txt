# File2TXT

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Release](https://img.shields.io/github/v/release/klp-xkjt/file2txt)](https://github.com/klp-xkjt/file2txt/releases)
[![GitHub repo](https://img.shields.io/badge/GitHub-klp--xkjt/file2txt-blue?logo=github)](https://github.com/klp-xkjt/file2txt)

将任意目录下的所有文本文件递归聚合为单个文件，支持过滤、多格式输出，方便 AI 分析与代码审查。
项目使用 Rust 编写。

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
| `-e, --extensions` | 只处理指定扩展名（逗号分隔） | `file2txt -e rs,toml,md` |
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
file2txt -n tests/test0.rs

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

## License

本项目基于 **MIT License** 开源。

### 说明

你可以：
- ✅ **商业使用** —— 可以用在商业项目中
- ✅ **修改源码** —— 自由修改、适配
- ✅ **分发代码** —— 可以分享给别人
- ✅ **私有使用** —— 个人或内部使用完全允许

你必须：
- 📌 **保留版权声明** —— 在分发或修改后的代码中保留原作者版权信息

你不能：
- ❌ **追究作者责任** —— 本软件按"原样"提供，作者不承担任何使用风险
- ❌ **使用作者名义推广** —— 未经许可，不得用作者名义做宣传

### License
```
MIT License

Copyright (c) 2026 klp-xkjt

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

见 [LICENSE](https://github.com/klp-xkjt/file2txt/blob/main/License) 文件。