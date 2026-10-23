# 36-appendix-b-borrow-conflict-fail · 附录 B：扩容场景的借用冲突（预期编译失败）

> 来源：配套文章《Rust 专题：理解所有权与借用》（Codex 源码学习专栏 · 所有权专题）第 附录 B.2 谁约束这块内存的使用 节的完整示例，代码按文章原样放入，不另行改写核心代码。

## 定位：阅读 / 对照用（预期编译失败，不可当作“修通”目标）

本目录存放文章中**故意写出来会编译失败**的对照程序，用来观察编译器报错。请对照文章阅读，**不要为了让它跑通而改教学意图**。

## 涉及的 Rust 概念
与 2.3 对照的 SDS 讨论示例

## 观察命令（Windows）

从仓库根目录执行：
```powershell
cd articles\02-understanding-ownership-and-borrowing\36-appendix-b-borrow-conflict-fail
rustc main.rs
```

> 预期 `rustc` 报错退出；不要强行改代码去“修通”，对照文章该小节理解报错即可。

## 文章给出的核心报错（类型）

```text
cannot borrow as mutable because it is also borrowed as immutable
```

（具体行号、辅助说明可能随 rustc 版本略有差异，核心错误码/关键词应一致。）

## 说明
- 文章标明本程序「预期编译失败」；本仓保留原文，便于读者本地复现报错。
- 合法对照版本见同篇相邻的可运行示例目录。
