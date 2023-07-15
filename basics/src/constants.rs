const NICE_NUMBER: i32 = 69;

fn main() {
    println!("{}", NICE_NUMBER);
    print_nice_number();

    // Error
    // NICE_NUMBER = 420;
}

fn print_nice_number() {
    println!("NICE_NUMBER in print_nice_number: {}", NICE_NUMBER);
}
