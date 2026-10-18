enum Input {
    Text { text: String },       // 文本输入，携带一个 String 字段
    LocalImage { path: String }, // 本地图片输入，携带路径；教学例子先用 String
}

fn describe(input: Input) -> String { // 接收一个 Input，返回描述字符串
    match input {                   // 判断是哪一种输入，并取出它携带的数据
        Input::Text { text } => format!("文本：{text}"), // 匹配 Text，把字段绑定为 text
        Input::LocalImage { path } => format!("本地图片：{path}"), // 匹配图片，取出 path
        // => 左侧是匹配模式，右侧产生结果；format! 返回字符串，不直接打印
    }
}

fn main() {
    let input = Input::Text {              // 这里是构造：选择 Input 的 Text 形态
        text: String::from("看看这个项目"), // 给这个形态的 text 字段赋值
    };
    println!("{}", describe(input));       // 调用 describe，再打印返回的字符串
}
