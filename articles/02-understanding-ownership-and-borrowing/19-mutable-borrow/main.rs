fn add_hint(text: &mut String) {
    text.push_str("，先看目录");
}

fn main() {
    let mut text = String::from("解释这个项目");
    add_hint(&mut text);
    println!("{text}");
}
