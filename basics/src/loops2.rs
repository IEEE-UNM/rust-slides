fn main() {
    let mut count1 = 0;
    let mut count2 = 0;
    'outer: loop {
        count1 += 1;
        loop {
            if count1 > 5 {
                break 'outer;
            }
            count2 += 1;
            println!("count1: {}, count2: {}", count1, count2);
            if count2 >= 5 {
                break;
            }
        }
        count2 = 0;
    }
}
