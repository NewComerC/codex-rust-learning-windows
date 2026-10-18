# 04-struct-impl-request · struct 与 impl：组织数据和方法

> 来源：配套文章《Rust 基础一：从编译运行到读懂 Rust 代码》（Codex 源码学习第二篇，初稿）中对应小节的完整示例，代码按文章原样放入，不另行改写核心代码。

## 学习目标
用 struct 定义数据、impl 定义操作；分清关联函数 new 与实例方法 is_ready(&self)。

## 涉及的 Rust 概念
struct 字段、impl、Self/self、&self 共享借用、String::from 关联函数调用、字段简写

## 运行命令（Windows）

从仓库根目录执行：
```powershell
cd articles\01-rust-basics-compiling-and-reading\04-struct-impl-request
cargo run
```

## 完成标准 / 自测
输出「可以开始：true」；能区分 Request::new 与 request.is_ready()；能解释 &self 只读借用当前值。

## 说明
- 文章中的教学示例说明「标为完整程序的代码，可以分别替换练习项目的 src/main.rs」；本仓为方便逐个跑通，把每个完整示例拆为独立目录。
- 若运行失败，先对照文章相应小节的注释逐行读，不要急着改代码。
