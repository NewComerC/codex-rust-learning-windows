struct Request {
    text: String,
    max_steps: u32,
}

impl Request {
    fn new(text: String) -> Self {
        Self { text, max_steps: 3 }
    }

    fn text(&self) -> &str {
        self.text.as_str()
    }

    fn set_max_steps(&mut self, max_steps: u32) {
        self.max_steps = max_steps;
    }
}

fn main() {
    let mut request = Request::new(String::from("解释这个项目"));
    let view = request.text();
    request.set_max_steps(8); // 错误：后面还要使用 view
    println!("{view}");
}
