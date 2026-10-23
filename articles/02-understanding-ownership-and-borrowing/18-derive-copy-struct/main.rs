#[derive(Copy, Clone)]
struct Count {
    value: u32,
}

fn main() {
    let a = Count { value: 5 };
    let b = a;
    println!("a = {}, b = {}", a.value, b.value);
}
