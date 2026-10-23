struct Request {
    text: String,
    max_steps: u32,
}

impl Request {
    fn new(text: String) -> Self {
        Self { text, max_steps: 3 }
    }

    fn is_ready(&self) -> bool {
        !self.text.is_empty() && self.max_steps > 0
    }

    fn set_max_steps(&mut self, max_steps: u32) {
        self.max_steps = max_steps;
    }

    fn text(&self) -> &str {
        self.text.as_str()
    }

    fn into_text(self) -> String {
        self.text // 将字段中的 String 移出，交给调用者
    }
}

fn main() {
    let text = String::from("解释这个项目");
    let mut request = Request::new(text);
    println!("可以开始：{}", request.is_ready());

    request.set_max_steps(5);
    let view = request.text();
    println!("借用文本：{view}"); // view 的最后一次使用

    request.set_max_steps(8);
    println!("最大步数：{}", request.max_steps);

    let text = request.into_text();
    println!("取出的文本：{text}");
    // println!("{}", request.is_ready()); // 取消注释后编译失败：request 已移动
}
