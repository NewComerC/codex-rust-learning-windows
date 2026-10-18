# 贡献指南

欢迎提 Issue 和 Pull Request。本仓库是 Codex 源码学习专题的 Rust 基础配套代码，按文章组织、按 tag 发布版本。

## 目录与文章的对应关系

- 一篇文章一个目录：`articles/<序号>-<kebab-slug>/`，例如第一篇为 `articles/01-rust-basics-compiling-and-reading/`。
- 文章索引见 [`articles/README.md`](articles/README.md)：序号 / 标题 / 对应目录 / 对应 tag / 状态。
- 后续文章只增量新增 `articles/02-…`、`articles/03-…`，不移动已发布目录。

## 版本与 tag 约定

- 每篇文章对应一个 minor 版本：第一篇 `v0.1.0`，第二篇 `v0.2.0`，依次类推。
- 同一篇文章的修正（错别字、命令或说明修正、示例小修复）用 patch 版本，如 `v0.1.1`。
- 每个正式 tag 都应有对应的 GitHub Release，说明中写清对应文章、变更内容与平台说明。
- 查看历史版本：仓库的 Tags / Releases 页面。

## 如何提 Issue

请优先使用 Issue 模板（不要空白提交）：

- **Bug 报告**：示例跑不通、输出与预期不一致、命令或文档错误。
- **运行求助**：卡在某一步、需要帮助排查环境或步骤。
- **内容建议**：想学的主题、后续文章建议、期望的讲解形式。

提 Issue 时请尽量写清：所属文章/示例目录、操作系统、`rustc --version` 与 `cargo --version` 的输出、复现步骤、实际与期望输出、报错原文。请勿在公开 Issue 中粘贴任何 token、密钥、`.env` 内容或其他凭据（见 `SECURITY.md`）。

## 如何提 Pull Request

1. Fork 本仓库并新建分支。
2. 只改与说明相关的内容；示例代码应与配套文章保持一致，如需改代码请在 PR 中说明与文章的对应关系。
3. 本地跑通受影响的示例（见下），在 PR 模板中填自测命令与输出，并勾选测试平台。
4. 一个 PR 聚焦一件事；文档、代码、目录结构变更请分开说明。

## 本地跑通命令（以第一篇 02、04 示例为例）

macOS（终端）：

```bash
cd articles/01-rust-basics-compiling-and-reading/02-cargo-basics
cargo run
cd ../04-struct-impl-request
cargo run
```

Windows（PowerShell）：

```powershell
cd articles\01-rust-basics-compiling-and-reading\02-cargo-basics
cargo run
cd ..\04-struct-impl-request
cargo run
```

预期输出：02 为 `开始阅读 Codex`，04 为 `可以开始：true`。其余示例同理进入对应目录执行 `cargo run`（01 用 `rustc` 直编，见其目录内 README）。

## 行为准则（简版）

- 友善、具体、就事论事；给复现信息，不给人身评价。
- 尊重不同基础的提问者；能给最小可复现示例时优先给示例。
- 不贴凭据、不贴他人隐私信息。
- 本项目参考 Contributor Covenant 的通用原则：https://www.contributor-covenant.org/
