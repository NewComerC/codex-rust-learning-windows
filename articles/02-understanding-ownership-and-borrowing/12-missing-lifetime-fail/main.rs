fn first_text(first: &String, _second: &String) -> &String {
    first
}

fn main() {
    let message = String::from("hello");
    let other = String::from("world");
    let view = first_text(&message, &other);
    println!("{view}");
}
