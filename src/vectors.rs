//Vectors - Resizable arrays

use std::mem;

pub fn run() {
    //Create vector
    let mut numbers: Vec<i32> = vec![2, 1, 24, 3, 5];

    //Re-assign value
    numbers[1] = 10;

    //Add on to vector
    numbers.push(5);
    numbers.push(6);

    //Get all val
    println!("Vector: {:?}", numbers);

    numbers.sort();
    numbers.dedup();

    //Get single val
    println!("Vector Single: {:?}", numbers[0]);

    //Get array lenth
    println!("Vector lenth: {}", numbers.len());

    //Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    let slice = &numbers;

    println!("Slice: {:?}", slice);

    println!("Half Slice: {:?}", &numbers[0..2]);
}
