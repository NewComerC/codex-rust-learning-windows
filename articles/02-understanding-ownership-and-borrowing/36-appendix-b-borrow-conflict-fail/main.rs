fn main() {
    let mut text = String::from("解释这个项目");
    let view = &text;

    text.push_str("，先看目录"); // 修改需要可变借用
    println!("{view}");        // 前面的共享引用还要继续使用
}
