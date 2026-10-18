struct Request {
    text: String,                   // 保存文本
    max_steps: u32,                  // 保存最大步数
}

impl Request {                      // 为 Request 定义关联函数和方法
    fn new(text: String) -> Self {   // 无 self 参数的关联函数，返回一个 Request
        Self {                      // Self 就是 Request；这一步实际构造值
            text,                   // text: text 的简写，左边是字段，右边是参数
            max_steps: 3,           // 给这个字段设置初始值 3
        }                           // 末尾不加分号，把这个值返回
    }

    fn is_ready(&self) -> bool {     // 借用当前 Request 读取信息，返回 true 或 false
        !self.text.is_empty() && self.max_steps > 0 // 文本非空，并且最大步数大于 0
    }
}

fn main() {
    let text = String::from("解释这个项目"); // 调用 from，把字面量转换成 String
    let request = Request::new(text);       // 通过类型找到 new，创建一个请求
    let ready = request.is_ready();         // 检查这个已有请求；自动把它借给 self
    println!("可以开始：{ready}");          // 输出：可以开始：true
}
