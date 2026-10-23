enum Input {
    Text { text: String },
    LocalImage { path: String },
}

fn describe(input: &Input) -> String {
    match input {
        Input::Text { text } => format!("文本：{text}"),
        Input::LocalImage { path } => format!("本地图片：{path}"),
    }
}

fn main() {
    let input = Input::Text {
        text: String::from("看看这个项目"),
    };
    println!("{}", describe(&input));
    println!("{}", describe(&input)); // 原输入仍在，可以再次借用
}
