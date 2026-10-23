# 文章索引

本目录按「一篇文章一个目录」组织。后续文章增量新增目录，不改动已有文章目录；每个正式版本用 Git tag / Release 区分（见仓库根目录 `CONTRIBUTING.md` 的版本约定）。

| 序号 | 文章 | 对应目录 | 对应 tag | 状态 |
|---|---|---|---|---|
| 01 | 《Rust 基础一：从编译运行到读懂 Rust 代码》（Codex 源码学习第二篇） | [`01-rust-basics-compiling-and-reading/`](01-rust-basics-compiling-and-reading/) | `v0.1.0` | 已发布（第一版） |
| 02 | 《Rust 专题：理解所有权与借用》 | [`02-understanding-ownership-and-borrowing/`](02-understanding-ownership-and-borrowing/) | `v0.2.0` | 已发布（第一版） |

## 后续文章登记方式

1. 新建目录：`articles/02-<kebab-slug>/`，目录内用 `01-`、`02-`… 编号放该篇示例；附录（如源码阅读指引）放编号靠后的目录并在该篇说明中标为附录。
2. 在上表新增一行：序号、文章标题、对应目录、计划 tag。
3. 发布时打下一个 minor tag（如第二篇为 `v0.2.0`），并创建对应 GitHub Release，在 Release 说明中写清对应文章。
4. 不要移动或改名已发布文章的目录；确需调整时同步更新根 README 与本索引的全部相对链接。

## 当前文章内部结构（01）

| 子目录 | 内容 |
|---|---|
| `01-rustc-hello/` | 用 rustc 编译运行最小程序 |
| `02-cargo-basics/` | 用 Cargo 创建项目 |
| `03-variables-and-functions/` | 变量与函数 |
| `04-struct-impl-request/` | struct 与 impl |
| `05-enum-match-input/` | enum 与 match |
| `06-option-title/` | Option |
| `07-codex-source-reading/` | 附录：Codex 源码阅读指引（仅阅读，不单独运行） |


## 当前文章内部结构（02）

| 子目录 | 内容 |
|---|---|
| `01-create-string-value/` | 先创建一个值 |
| `02-move-assignment/` | 赋值发生移动 |
| `03-use-after-move-fail/` | 移动后再读取原变量（预期编译失败）（预期编译失败对照） |
| `04-clone-string/` | 显式 clone 复制文本 |
| `05-borrow-reference/` | 借用而不拿走 |
| `06-move-into-function-fail/` | 按值传参后再读取（预期编译失败）（预期编译失败对照） |
| `07-borrow-into-function/` | 按引用传参只是看看 |
| `08-i32-copy/` | 整数赋值是复制 |
| `09-borrow-within-scope/` | 引用在数据仍有效时使用 |
| `10-dangling-reference-fail/` | 数据销毁后继续用引用（预期编译失败）（预期编译失败对照） |
| `11-lifetime-elision/` | 单个输入引用可省略生命周期标注 |
| `12-missing-lifetime-fail/` | 两个输入引用缺少标注（预期编译失败）（预期编译失败对照） |
| `13-lifetime-annotation/` | 手写生命周期标注关联返回引用 |
| `14-shared-lifetime-choose/` | 两个输入共用生命周期并二选一返回 |
| `15-move-without-keyword/` | 普通赋值即发生移动 |
| `16-copy-i32-contrast/` | Copy：整数赋值后两边可用 |
| `17-move-string-contrast-fail/` | 非 Copy：String 移动后再读（预期编译失败）（预期编译失败对照） |
| `18-derive-copy-struct/` | derive(Copy, Clone) 的结构体 |
| `19-mutable-borrow/` | 可变借用修改原数据 |
| `20-borrow-conflict-fail/` | 共享借用未结束就可变修改（预期编译失败）（预期编译失败对照） |
| `21-borrow-then-mutate/` | 先用完共享借用再修改 |
| `22-show-string-ref/` | 参数 &String 只读借用 |
| `23-show-str-slice/` | 更常见的 &str 参数 |
| `24-request-full-flow/` | Request：创建、读取、修改与接管 |
| `25-consume-string-show/` | 按值接收后在函数内清理 |
| `26-self-receiver-forms/` | self / &self / &mut self 三种接收者 |
| `27-borrow-blocks-mut-fail/` | 返回引用未用完时可变借用（预期编译失败）（预期编译失败对照） |
| `28-return-ref-vs-owned/` | 返回 &str 借用 vs into_text 移交 |
| `29-return-local-ref-fail/` | 返回指向局部的引用（预期编译失败）（预期编译失败对照） |
| `30-return-owned-new-text/` | 返回新建的 String |
| `31-enum-match-borrow/` | 借用枚举时分支字段也借用 |
| `32-option-borrow-and-take/` | Option：先借用再取出 |
| `33-create-request-read-text/` | 创建请求后读取文本 |
| `34-appendix-a-clone/` | 附录 A：clone 复制文本 |
| `35-appendix-a-literal-clone-borrow/` | 附录 A：字面量、副本与借用 |
| `36-appendix-b-borrow-conflict-fail/` | 附录 B：扩容场景的借用冲突（预期编译失败）（预期编译失败对照） |
