struct Request {
    text: String,
}

impl Request {
    fn new(text: String) -> Self {
        Self { text }
    }

    fn text(&self) -> &str {
        self.text.as_str()
    }
}

fn main() {
    let text = String::from("解释这个项目");
    let request = Request::new(text);
    println!("请求文本：{}", request.text());
}
