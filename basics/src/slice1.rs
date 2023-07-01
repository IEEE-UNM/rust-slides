fn main() {
    // General Slices
    let array: [u32; 4] = [21, 69, 420, 69420];
    let first_two: &[u32] = &array[0..=1];
    let second_to_last: &[u32] = &array[1..];
    // Note that 3 is not inclusive
    let first_to_third: &[u32] = &array[..3];
    let entire_array: &[u32] = &array[..];

    // See More Format Options in Apendix about what '{:?}' does
    println!("First Two: {:?}", first_two);
    println!("Second to Last: {:?}", second_to_last);
    println!("First to Third: {:?}", first_to_third);
    println!("Entire Array: {:?}", entire_array);
}
