//Arrays - fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [2, 1, 24, 3, 5];

    //Re-assign value
    numbers[1] = 10;

    //Get all val
    println!("Array: {:?}", numbers);

    //Get single val
    println!("Array Single: {:?}", numbers[0]);

    //Get array lenth
    println!("Array lenth: {}", numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes",mem::size_of_val(&numbers));

    let slice = &numbers;

    println!("Slice: {:?}",slice);
    
    println!("Half Slice: {:?}",&numbers[0..2]);
}
