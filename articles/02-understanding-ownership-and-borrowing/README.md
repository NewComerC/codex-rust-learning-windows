# 02 · 理解所有权与借用

> 对应文章《Rust 专题：理解所有权与借用》；目录 slug：`02-understanding-ownership-and-borrowing`（slug 按文章题「理解所有权与借用」拟为 understanding-ownership-and-borrowing）。对应 tag：`v0.2.0`。

文章写明：下面每个代码块都是一个独立的完整程序，可以分别放进 `main.rs`。本目录按文章顺序编号，共 36 个示例（含可运行与预期编译失败对照）。

| 子目录 | 内容 | 类型 |
|---|---|---|
| `01-create-string-value/` | 先创建一个值 | 可运行 |
| `02-move-assignment/` | 赋值发生移动 | 可运行 |
| `03-use-after-move-fail/` | 移动后再读取原变量（预期编译失败） | 预期编译失败（对照） |
| `04-clone-string/` | 显式 clone 复制文本 | 可运行 |
| `05-borrow-reference/` | 借用而不拿走 | 可运行 |
| `06-move-into-function-fail/` | 按值传参后再读取（预期编译失败） | 预期编译失败（对照） |
| `07-borrow-into-function/` | 按引用传参只是看看 | 可运行 |
| `08-i32-copy/` | 整数赋值是复制 | 可运行 |
| `09-borrow-within-scope/` | 引用在数据仍有效时使用 | 可运行 |
| `10-dangling-reference-fail/` | 数据销毁后继续用引用（预期编译失败） | 预期编译失败（对照） |
| `11-lifetime-elision/` | 单个输入引用可省略生命周期标注 | 可运行 |
| `12-missing-lifetime-fail/` | 两个输入引用缺少标注（预期编译失败） | 预期编译失败（对照） |
| `13-lifetime-annotation/` | 手写生命周期标注关联返回引用 | 可运行 |
| `14-shared-lifetime-choose/` | 两个输入共用生命周期并二选一返回 | 可运行 |
| `15-move-without-keyword/` | 普通赋值即发生移动 | 可运行 |
| `16-copy-i32-contrast/` | Copy：整数赋值后两边可用 | 可运行 |
| `17-move-string-contrast-fail/` | 非 Copy：String 移动后再读（预期编译失败） | 预期编译失败（对照） |
| `18-derive-copy-struct/` | derive(Copy, Clone) 的结构体 | 可运行 |
| `19-mutable-borrow/` | 可变借用修改原数据 | 可运行 |
| `20-borrow-conflict-fail/` | 共享借用未结束就可变修改（预期编译失败） | 预期编译失败（对照） |
| `21-borrow-then-mutate/` | 先用完共享借用再修改 | 可运行 |
| `22-show-string-ref/` | 参数 &String 只读借用 | 可运行 |
| `23-show-str-slice/` | 更常见的 &str 参数 | 可运行 |
| `24-request-full-flow/` | Request：创建、读取、修改与接管 | 可运行 |
| `25-consume-string-show/` | 按值接收后在函数内清理 | 可运行 |
| `26-self-receiver-forms/` | self / &self / &mut self 三种接收者 | 可运行 |
| `27-borrow-blocks-mut-fail/` | 返回引用未用完时可变借用（预期编译失败） | 预期编译失败（对照） |
| `28-return-ref-vs-owned/` | 返回 &str 借用 vs into_text 移交 | 可运行 |
| `29-return-local-ref-fail/` | 返回指向局部的引用（预期编译失败） | 预期编译失败（对照） |
| `30-return-owned-new-text/` | 返回新建的 String | 可运行 |
| `31-enum-match-borrow/` | 借用枚举时分支字段也借用 | 可运行 |
| `32-option-borrow-and-take/` | Option：先借用再取出 | 可运行 |
| `33-create-request-read-text/` | 创建请求后读取文本 | 可运行 |
| `34-appendix-a-clone/` | 附录 A：clone 复制文本 | 可运行 |
| `35-appendix-a-literal-clone-borrow/` | 附录 A：字面量、副本与借用 | 可运行 |
| `36-appendix-b-borrow-conflict-fail/` | 附录 B：扩容场景的借用冲突（预期编译失败） | 预期编译失败（对照） |

## 怎么跑

- **可运行**：进入子目录后 `rustc main.rs`，再运行生成的二进制（见各子 README 的平台命令）。
- **预期编译失败**：同样执行 `rustc main.rs`，对照文章理解报错；不要改代码“修通”。

代码与文章一致，未另行改写教学逻辑。
