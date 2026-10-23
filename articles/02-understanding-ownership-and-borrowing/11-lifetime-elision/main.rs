fn borrow_text(source: &String) -> &String {
    source
}

fn main() {
    let message = String::from("hello");
    let view = borrow_text(&message);
    println!("{view}");
}
