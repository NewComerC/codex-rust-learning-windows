# 06-option-title · Option：明确表示有值或缺失

> 来源：配套文章《Rust 基础一：从编译运行到读懂 Rust 代码》（Codex 源码学习第二篇，初稿）中对应小节的完整示例，代码按文章原样放入，不另行改写核心代码。

## 学习目标
用 Option<String> 表达可能缺失的标题，并用 match 同时处理 Some 与 None。

## 涉及的 Rust 概念
Option<String>、Some/None、Option 与 String 是不同类型、Some("") 与 None 的区别

## 运行命令（Windows）

从仓库根目录执行：
```powershell
cd articles\01-rust-basics-compiling-and-reading\06-option-title
cargo run
```

## 完成标准 / 自测
输出「未命名任务」；能回答文章自检第 3 题：None 与空字符串不是一回事。

## 说明
- 文章中的教学示例说明「标为完整程序的代码，可以分别替换练习项目的 src/main.rs」；本仓为方便逐个跑通，把每个完整示例拆为独立目录。
- 若运行失败，先对照文章相应小节的注释逐行读，不要急着改代码。
