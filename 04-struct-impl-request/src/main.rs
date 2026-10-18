struct Request {
    text: String,                   // 保存文本
    max_steps: u32,                  // 保存最大步数
}

impl Request {                      // 为 Request 定义关联函数和方法
    fn new(text: String) -> Self {   // 没有 self 参数；Self 在这里就是 Request
        Self {                      // 构造一个 Request 值
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
    let request = Request::new(text);       // 调用 new，用这个 String 创建请求
    let ready = request.is_ready();         // 调用实例方法；Rust 自动借用 request
    println!("可以开始：{ready}");          // 输出：可以开始：true
}
