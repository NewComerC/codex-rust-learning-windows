struct Request {
    text: String,
}

impl Request {
    fn new_text(&self) -> String {
        let local = String::from("临时文本");
        local // 将 String 作为返回值移交给调用者
    }
}

fn main() {
    let request = Request {
        text: String::from("解释这个项目"),
    };
    let text = request.new_text();
    println!("新建文本：{text}");
    println!("原有文本：{}", request.text);
}
