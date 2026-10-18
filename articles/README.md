# 文章索引

本目录按「一篇文章一个目录」组织。后续文章增量新增目录，不改动已有文章目录；每个正式版本用 Git tag / Release 区分（见仓库根目录 `CONTRIBUTING.md` 的版本约定）。

| 序号 | 文章 | 对应目录 | 对应 tag | 状态 |
|---|---|---|---|---|
| 01 | 《Rust 基础一：从编译运行到读懂 Rust 代码》（Codex 源码学习第二篇） | [`01-rust-basics-compiling-and-reading/`](01-rust-basics-compiling-and-reading/) | `v0.1.0` | 已发布（第一版） |

## 后续文章登记方式

1. 新建目录：`articles/02-<kebab-slug>/`，目录内用 `01-`、`02-`… 编号放该篇示例；附录（如源码阅读指引）放编号靠后的目录并在该篇说明中标为附录。
2. 在上表新增一行：序号、文章标题、对应目录、计划 tag。
3. 发布时打下一个 minor tag（如第二篇为 `v0.2.0`），并创建对应 GitHub Release，在 Release 说明中写清对应文章。
4. 不要移动或改名已发布文章的目录；确需调整时同步更新根 README 与本索引的全部相对链接。

## 当前文章内部结构（01）

| 子目录 | 内容 |
|---|---|
| `01-rustc-hello/` | 用 rustc 编译运行最小程序 |
| `02-cargo-basics/` | 用 Cargo 创建项目 |
| `03-variables-and-functions/` | 变量与函数 |
| `04-struct-impl-request/` | struct 与 impl |
| `05-enum-match-input/` | enum 与 match |
| `06-option-title/` | Option |
| `07-codex-source-reading/` | 附录：Codex 源码阅读指引（仅阅读，不单独运行） |
