fn main() {
    let view;

    {
        let message = String::from("hello");
        view = &message;
        println!("{view}");
    } // message 持有的 String 在这里销毁
}
