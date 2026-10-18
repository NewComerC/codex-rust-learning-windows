# 05-enum-match-input · enum 与 match：表达和处理不同形态

> 来源：配套文章《Rust 基础一：从编译运行到读懂 Rust 代码》（Codex 源码学习第二篇，初稿）中对应小节的完整示例，代码按文章原样放入，不另行改写核心代码。

## 学习目标
用 enum 变体携带不同数据，用 match 识别形态并取出字段。

## 涉及的 Rust 概念
enum 变体、Input::Text / Input::LocalImage 构造、match 模式绑定、format! 与 println! 区别、穷尽匹配

## 运行命令（Windows）
```powershell
cd 05-enum-match-input
cargo run
```

## 完成标准 / 自测
能运行并输出文本描述；能回答：给 Input 新增变体而 describe 不改，为何可能编译失败（文章自检第 2 题）。

## 说明
- 文章中的教学示例说明「标为完整程序的代码，可以分别替换练习项目的 src/main.rs」；本仓为方便逐个跑通，把每个完整示例拆为独立目录。
- 若运行失败，先对照文章相应小节的注释逐行读，不要急着改代码。
