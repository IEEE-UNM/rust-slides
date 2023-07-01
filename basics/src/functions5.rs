fn main() {
    println!("Number from function is: {}", get_number(false));
    println!("Big Number from function is: {}", get_number(true));
}
fn get_number(big: bool) -> u32 {
    if big {
        return 420;
    }
    // Last Expression
    60 + 9
}
