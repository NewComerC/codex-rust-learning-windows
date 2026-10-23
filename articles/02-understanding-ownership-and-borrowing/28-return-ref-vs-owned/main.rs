struct Request {
    text: String,
}

impl Request {
    fn text(&self) -> &str {
        self.text.as_str()
    }

    fn into_text(self) -> String {
        self.text
    }
}

fn main() {
    let request = Request {
        text: String::from("解释这个项目"),
    };

    let view = request.text();
    println!("借用文本：{view}"); // 之后不再使用 view

    let text = request.into_text();
    println!("取出的文本：{text}");
}
