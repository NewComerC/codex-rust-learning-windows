fn main() {
    let title = Some(String::from("项目分析"));

    let display = match &title {
        Some(text) => text.as_str(), // text 是 &String，返回 &str
        None => "未命名任务",
    };
    println!("借用显示：{display}"); // display 的最后一次使用

    let owned_title = match title {
        Some(text) => text, // text 是 String，按值取出
        None => String::from("未命名任务"),
    };
    println!("取出的标题：{owned_title}");
    // println!("{title:?}"); // 取消注释后编译失败：不能再把 title 作为完整值读取
}
