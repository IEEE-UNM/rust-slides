fn main() {
    let mut x = 0;
    loop {
        x += 1;
        println!("Hello {}.", x);
        if x == 5 {
            break;
        }
    }
}
