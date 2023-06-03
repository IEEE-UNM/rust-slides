fn main() {
    let x: Option<i32> = Some(69);
    match x {
        None => None,
        Some(i) => Some(i + 1),
    };
}
