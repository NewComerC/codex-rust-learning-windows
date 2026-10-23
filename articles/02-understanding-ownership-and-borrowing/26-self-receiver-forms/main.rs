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

    fn into_text(self) -> String {
        self.text
    }
}

fn main() {
    let mut request = Request::new(String::from("解释这个项目"));
    println!("可以开始：{}", request.is_ready());

    request.set_max_steps(5);
    println!("最大步数：{}", request.max_steps);

    let text = request.into_text();
    println!("取出的文本：{text}");
}
