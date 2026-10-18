# 01-rustc-hello · 用 rustc 编译运行最小程序

> 来源：配套文章《Rust 基础一：从编译运行到读懂 Rust 代码》（Codex 源码学习第二篇，初稿）中对应小节的完整示例，代码按文章原样放入，不另行改写核心代码。

## 学习目标
理解 Rust 先编译成原生程序、再运行的过程；认识 fn、main 入口与 println! 宏。

## 涉及的 Rust 概念
rustc 编译与执行、fn main 入口、println! 宏

## 运行命令（Windows）
```powershell
rustc main.rs
.\main.exe
```

> 文章原文：Windows 下生成的程序通常叫 `main.exe`，在 PowerShell 中运行 `.\main.exe`。

## 完成标准 / 自测
能说明改源码后为何要重新编译；能独立编译并运行、看到输出。

## 说明
- 文章中的教学示例说明「标为完整程序的代码，可以分别替换练习项目的 src/main.rs」；本仓为方便逐个跑通，把每个完整示例拆为独立目录。
- 若运行失败，先对照文章相应小节的注释逐行读，不要急着改代码。
