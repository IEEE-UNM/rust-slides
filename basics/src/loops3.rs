fn main() {
    let mut count1 = 0;
    let x = loop {
        count1 += 1;
        println!("Count is {}", count1);
        if count1 == 5 {
            break count1;
        }
    };
    println!("x: {}", x);
}
