pub fn run() {
    // Printing to the Console
    println!("Hello, world!");

    // Basic Formatting
    println!("Hello, {}!", "Alice");
    println!("The answer is: {}", 42);

    // Positional Arguments

    println!(
        "{0} is from {1} and likes to {2}",
        "Lucia", "Mexico", "drink"
    );

    // Named Arguments

    println!(
        "{name} likes to play {activity}",
        name = "Bob",
        activity = "chess"
    );

    //Placeholder Traits

    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Debug trait for tuples
    println!("{:?}", (3, true, "hello"));

    //Basic math

    print!("{n1} + {n2} = {r}", n1 = 5, n2 = 10, r = 5 + 10);
}
