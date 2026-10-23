fn show_text(text: &str) {
    println!("{text}");
}

fn main() {
    let text = String::from("解释这个项目");

    show_text(text.as_str()); // 显式取得整个 String 的字符串切片
    show_text(&text);         // 这里也可以把 &String 转换为所需的 &str
    show_text("先查看目录"); // 字符串字面量可以直接传入

    println!("仍由调用方持有：{text}");
}
