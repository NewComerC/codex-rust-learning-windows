# Codex 源码学习 · Rust 基础专题（Windows 版）

> 配套文章：《Rust 基础一：从编译运行到读懂 Rust 代码》（Codex 源码学习第二篇）——从编译运行到读懂 Rust 代码。
> 另一平台版本：[macOS 版 codex-rust-learning-macos](https://github.com/NewComerC/codex-rust-learning-macos)（学习路线与项目结构一致，仅平台指引不同）。
> 本仓默认读者可以正常访问 GitHub 与 Rust 官方站点。

## 这是什么

这是 Codex 源码学习专题的 Rust 基础配套代码。目标不是系统学完 Rust，而是先跑通程序、读懂 Codex 源码里最先会遇到的写法：编译与 Cargo、变量与函数、struct/impl、enum/match、Option，以及沿 Cargo 配置找到程序入口。

## 适合人群与前置要求

- 适合：有编程经验、但没有 Rust 基础，准备读 Codex 源码的读者（文章阅读前提，原文表述）。
- 默认你理解变量、函数、条件和循环；不需要提前了解 Cargo、所有权或 Rust 工程结构。
- 前置：能访问 GitHub；按下面指引安装 Rust 工具链，或先用 [Rust Playground](https://play.rust-lang.org/) 在线运行小例子（文章给出的备选方式）。

- **Windows 实测环境说明**：本仓 6 个示例已在 Windows（AMD64/x86_64）实机跑通，本次实测使用 rustup 的 GNU 工具链 `stable-x86_64-pc-windows-gnu`（rustc/cargo 1.99.1），示例无第三方依赖，`rustc`/`cargo run` 均可直接编译运行。若按 Rust 官方安装器的默认 MSVC 工具链（`x86_64-pc-windows-msvc`）安装，通常还需配合 Visual Studio Build Tools 的 C++ 构建组件；两种工具链下本仓的运行命令（`rustc main.rs`、`.\main.exe`、`cargo run`）与预期打印一致，未安装 MSVC 组件时可优先选择 GNU 工具链完成本篇练习。

## 学习路线（按文章四部分）

1. **Rust 语言全景**：编译执行、类型与抽象、所有权与资源、Option/Result、并发与安全——先建立整体认识（以读文章为主）。
2. **开发工具与 Cargo**：`01-rustc-hello` → `02-cargo-basics`，跑通第一个程序。
3. **Rust 语法基础**：`03-variables-and-functions` → `04-struct-impl-request` → `05-enum-match-input` → `06-option-title`。
4. **回到 Codex 源码**：`07-codex-source-reading`，读真实类型 `UserInput` 并沿 Cargo 配置找到 CLI 入口。

## 项目列表

| 目录 | 主题 | 关键概念 |
|---|---|---|
| `01-rustc-hello` | 用 rustc 编译运行最小程序 | rustc 编译与执行、fn main 入口、println! 宏 |
| `02-cargo-basics` | 用 Cargo 创建项目 rust_reading | Cargo 项目结构、Cargo.toml 的 [package]/[dependencies]、edition、cargo check/build/run/test、target/debug 与 target/release |
| `03-variables-and-functions` | 变量与函数：类型、可变性与返回值 | let 与类型标注 u32、类型推断、mut、shadowing、if 表达式、尾表达式返回值、() 单元值 |
| `04-struct-impl-request` | struct 与 impl：组织数据和方法 | struct 字段、impl、Self/self、&self 共享借用、String::from 关联函数调用、字段简写 |
| `05-enum-match-input` | enum 与 match：表达和处理不同形态 | enum 变体、Input::Text / Input::LocalImage 构造、match 模式绑定、format! 与 println! 区别、穷尽匹配 |
| `06-option-title` | Option：明确表示有值或缺失 | Option<String>、Some/None、Option 与 String 是不同类型、Some("") 与 None 的区别 |
| `07-codex-source-reading` | 回到 Codex 源码：读类型并找到入口 | pub、Vec<T>、PathBuf、Option、属性 #[...]、package/crate/module/workspace、[[bin]] 入口定位 |

每个项目目录内的 README 写有：学习目标、涉及概念、运行命令、完成标准/自测问题（均源自文章）。

## 快速开始

### Windows 安装与跑通（PowerShell）

> 文章原文只说：需要安装时按 [Rust 官方安装说明](https://www.rust-lang.org/tools/install) 配置工具链，并给出 Windows 下 `main.exe` / PowerShell 运行方式。下面的安装命令是 **Rust 官方通用方式（非文章原文命令）**，请以官方页面为准：

1. 从官方安装页下载并运行 `rustup-init.exe`，按提示完成安装（官方通用方式，标注非文章原文）。
2. 重新打开 PowerShell，验证（通用 Rust 常用命令，文章未逐字给出）：

```powershell
rustc --version
cargo --version
```

3. 跑通第一个项目：

```powershell
cd 01-rustc-hello
rustc main.rs
.\main.exe

cd ..\02-cargo-basics
cargo run
```

路径与产物写法为 Windows 风格：可执行文件通常带 `.exe`，如 `target\debug\rust_reading.exe`（文章原文提到 Windows 文件名通常带 `.exe`）。

后续常用命令（文章原文给出的对照表）：

| 命令 | 做什么 |
|---|---|
| `cargo check` | 检查代码及依赖，通常较快（不生成最终可执行文件） |
| `cargo build` | 构建程序，默认开发配置 |
| `cargo run` | 按需构建并运行 |
| `cargo test` | 构建并运行测试 |
| `cargo build --release` | 使用 release 配置构建，默认更高优化 |

## 关于 Cargo.lock 的选择

各示例均为**二进制项目**。本仓 `.gitignore` 忽略 `**/target/`，但**不忽略 Cargo.lock**：二进制项目保留 Cargo.lock 更利于复现同一依赖解析结果。各示例当前无第三方依赖，首次 `cargo run` 后会生成各自的 Cargo.lock。

## 与 Codex 源码学习的关系

本专题是阅读 Codex 源码前的基础准备：先在本仓跑通并读懂教学示例，再按 `07-codex-source-reading/README.md` 的指引去读固定版本（commit `5009a99dfc07830ddf7464c26f310a2b8532b55d`）的 Codex 源码。专栏链接待补充（占位，不在此编造）。

## 代码来源说明

- `01`–`06` 的代码为文章中标注为完整程序的教学示例，按文章原样放入（含文章注释）。
- `07` 中的源码为摘录，不能单独运行，只作阅读练习，详见该目录 README 的来源链接。

## License

MIT，见 [LICENSE](LICENSE)。署名年份/姓名如需更换，请直接修改该文件。
