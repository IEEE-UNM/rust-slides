fn main() {
    let x = 69;
    let y = if x > 50 {
        true
    } else {
        false
    };
    println!("y: {}, check_50: {}", y, check_50(x));
}

fn check_50(x: i32) -> bool {
    if x < 50 {
        false
    } else {
        true
    }
}
