fn remaining(used: u32) -> u32 {
    if used < 3 {
        3 - used
    } else {
        0
    }
}

fn main() {
    for used in [0, 1, 3] {
        let left = remaining(used);
        println!("已用 {used} 次，剩余 {left} 次");
    }
}
