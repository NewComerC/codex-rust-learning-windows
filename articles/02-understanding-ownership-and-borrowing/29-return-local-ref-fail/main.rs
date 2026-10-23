struct Request {
    text: String,
}

impl Request {
    fn bad_text(&self) -> &str {
        let local = String::from("临时文本");
        local.as_str() // 错误：不能返回指向局部 String 内容的引用
    }
}

fn main() {
    let request = Request {
        text: String::from("解释这个项目"),
    };
    println!("原有文本：{}", request.text);
    let view = request.bad_text();
    println!("{view}");
}
