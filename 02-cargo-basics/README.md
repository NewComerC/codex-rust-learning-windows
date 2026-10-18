# 02-cargo-basics · 用 Cargo 创建项目 rust_reading

> 来源：配套文章《Rust 基础一：从编译运行到读懂 Rust 代码》（Codex 源码学习第二篇，初稿）中对应小节的完整示例，代码按文章原样放入，不另行改写核心代码。

## 学习目标
分清 rustc / Cargo / rustup 各自职责；用 Cargo 创建项目并 cargo run 跑通。

## 涉及的 Rust 概念
Cargo 项目结构、Cargo.toml 的 [package]/[dependencies]、edition、cargo check/build/run/test、target/debug 与 target/release

## 运行命令（Windows）
```powershell
cd 02-cargo-basics
cargo run
```

## 完成标准 / 自测
能说清三个工具分工；cargo run 成功；知道 cargo run≠解释执行。

## 说明
- 文章中的教学示例说明「标为完整程序的代码，可以分别替换练习项目的 src/main.rs」；本仓为方便逐个跑通，把每个完整示例拆为独立目录。
- 若运行失败，先对照文章相应小节的注释逐行读，不要急着改代码。
