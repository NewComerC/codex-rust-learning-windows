fn choose_text<'borrow>(
    first: &'borrow String,
    second: &'borrow String,
    use_first: bool,
) -> &'borrow String {
    if use_first {
        first
    } else {
        second
    }
}

fn main() {
    let message = String::from("hello");
    let other = String::from("world");
    println!("{}", choose_text(&message, &other, true));
    println!("{}", choose_text(&message, &other, false));
}
