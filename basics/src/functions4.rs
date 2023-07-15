fn main() {
    // Blocks can be used as expressions
    let x = {
        let y = 69;
        y * y
    };
    println!("x: {x}");

    let x = {
        let y = 69;
        // Semicolon made it return ()
        y * y;
    };
    println!("x: {x:?}");
}
