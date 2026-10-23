fn first_text<'borrow>(first: &'borrow String, _second: &String) -> &'borrow String {
    first
}

fn main() {
    let message = String::from("hello");
    let other = String::from("world");
    let view = first_text(&message, &other);
    println!("{view}");
}
