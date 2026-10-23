# 32-option-borrow-and-take · Option：先借用再取出

> 来源：配套文章《Rust 专题：理解所有权与借用》（Codex 源码学习专栏 · 所有权专题）第 4.2 对照 Option 中的借用与取出 节的完整示例，代码按文章原样放入，不另行改写核心代码。

## 学习目标
跑通并理解本示例所展示的所有权 / 借用规则；能对照文章解释输出。

## 涉及的 Rust 概念
match &title 与按值取出

## 运行命令（Windows）

从仓库根目录执行：
```powershell
cd articles\02-understanding-ownership-and-borrowing\32-option-borrow-and-take
rustc main.rs
.\main.exe
```

> 在 Windows PowerShell 从仓库根目录执行：第一行进入目录，第二行编译，第三行运行。

## 期望输出（源自文章）

```text
借用显示：项目分析
取出的标题：项目分析
```

## 说明
- 文章标明「每个代码块都是一个独立的完整程序，可以分别放进 main.rs」；本仓为方便逐个跑通，把每个完整示例拆为独立目录。
- 若运行失败，先对照文章相应小节的注释逐行读，不要急着改代码。
