# 07-codex-source-reading · 附录：Codex 源码阅读指引（读懂类型并找到入口）

> 定位：附录：Codex 源码阅读指引（保留自初稿的源码阅读指引；定稿正文为三部分，本附录不属于正文第四部分）。文中源码为**摘录，不能单独复制成完整程序**，故本目录只放阅读指引，不放可运行的 src/main.rs 占位工程冒充源码。

## 学习目标
- 能按「字段 → 方法 → 属性」的顺序读懂 Codex 的输入类型定义。
- 能沿 Cargo 配置从 workspace 找到 CLI 程序入口，而不是逐个 .rs 文件猜。

## 涉及的 Rust 概念
pub 可见性、enum 变体携带数据、Vec<TextElement>、std::path::PathBuf、Option<ImageDetail>、属性 #[derive(...)] / #[serde(...)]、#[non_exhaustive]、package / crate / module / workspace、[[bin]] 入口配置、anyhow::Result<()> 入口签名（仅认出，不展开异步）。

## 阅读步骤（按文章）
1. 打开固定版本文件 `codex-rs/protocol/src/user_input.rs`，先读 UserInput 的 Text / LocalImage 两个变体：https://github.com/openai/codex/blob/5009a99dfc07830ddf7464c26f310a2b8532b55d/codex-rs/protocol/src/user_input.rs#L13
2. 认出属性行 `#[serde(tag = "type", rename_all = "snake_case")]` 的用途（序列化标签与命名），以及 #[non_exhaustive] 对外部 match 的影响。
3. 继续读同文件中 TextElement::new（byte_range + Option<String> placeholder → Self）：https://github.com/openai/codex/blob/5009a99dfc07830ddf7464c26f310a2b8532b55d/codex-rs/protocol/src/user_input.rs#L68
4. 打开 `codex-rs/cli/Cargo.toml`，从 [package] name = "codex-cli" 与 [[bin]] name = "codex" / path = "src/main.rs" 定位入口：https://github.com/openai/codex/blob/5009a99dfc07830ddf7464c26f310a2b8532b55d/codex-rs/cli/Cargo.toml#L1
5. 在 `codex-rs/cli/src/main.rs` 找到入口签名 `fn main() -> anyhow::Result<()>`，先定位、不展开 async/move/Box::pin：https://github.com/openai/codex/blob/5009a99dfc07830ddf7464c26f310a2b8532b55d/codex-rs/cli/src/main.rs#L1032

## 完成标准 / 自测（文章原文三问）
1. 为什么 `cargo run` 能直接运行，但 Rust 仍然需要编译？
2. 为什么给教学 Input 增加变体，describe 可能编译失败？
3. `Option<String>` 为 None 和保存空字符串是不是一回事？

能用自己的话答出三问（对照文章「阅读自检」节），即完成本专题。

## 平台说明（Windows）
本节以在线读 GitHub 固定链接为主，无平台专属命令；本地克隆 Codex 仓库后的路径写法按本平台习惯即可，文章未给出本地克隆命令，此处不编造。
