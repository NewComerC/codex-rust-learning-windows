fn main() {
    let title: Option<String> = None;       // 标题可能有值；这里明确表示没有
    let display_title = match title {      // 把两种可能都处理成一个 String
        Some(text) => text,                // 有标题，就取出里面的字符串
        None => String::from("未命名任务"), // 没有标题，就创建默认字符串
    };
    println!("{display_title}");           // 输出：未命名任务
}
