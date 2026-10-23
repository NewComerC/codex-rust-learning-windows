# 23-show-str-slice · 更常见的 &str 参数

> 来源：配套文章《Rust 专题：理解所有权与借用》（Codex 源码学习专栏 · 所有权专题）第 2.4 String 拥有文本而字符串切片借用文本 节的完整示例，代码按文章原样放入，不另行改写核心代码。

## 学习目标
跑通并理解本示例所展示的所有权 / 借用规则；能对照文章解释输出。

## 涉及的 Rust 概念
&str / as_str / 字面量

## 运行命令（Windows）

从仓库根目录执行：
```powershell
cd articles\02-understanding-ownership-and-borrowing\23-show-str-slice
rustc main.rs
.\main.exe
```

> 在 Windows PowerShell 从仓库根目录执行：第一行进入目录，第二行编译，第三行运行。

## 期望输出（源自文章）

```text
解释这个项目
解释这个项目
先查看目录
仍由调用方持有：解释这个项目
```

## 说明
- 文章标明「每个代码块都是一个独立的完整程序，可以分别放进 main.rs」；本仓为方便逐个跑通，把每个完整示例拆为独立目录。
- 若运行失败，先对照文章相应小节的注释逐行读，不要急着改代码。
