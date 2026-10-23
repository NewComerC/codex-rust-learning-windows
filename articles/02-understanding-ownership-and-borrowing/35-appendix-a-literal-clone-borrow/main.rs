fn main() {
    let literal = "hello";
    let a = String::from(literal);
    let mut b = a.clone();
    b.push_str("!");
    let string_ref = &a;
    let view = a.as_str();

    println!("literal = {literal}");
    println!("a = {a}, b = {b}, view = {view}");
    println!("string_ref = {string_ref}");
}
