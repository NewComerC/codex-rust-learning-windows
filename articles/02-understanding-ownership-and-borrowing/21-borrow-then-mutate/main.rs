fn main() {
    let mut text = String::from("解释这个项目");
    let view = &text;

    println!("修改前：{view}"); // view 的最后一次使用
    text.push_str("，先看目录");
    println!("修改后：{text}");
}
