pub fn run(){

    //Primitive str = Immutable fixed-length string somewhere in memory
    //String = Growable, heap-allocated data structure - Use when you need to modify or own string data


    //String slice
    let msg = "Hello world"; 

    println!("String Slice : {:?}", (&msg[0..3], &msg.len()));

    let mut msg2 = String::from("Hello world");
    msg2.push('!');
    println!("String Grawble : {:?}", (&msg2, &msg2.len()));

    //Capacity in bits
    println!("Capacity: {}",msg2.capacity());
    
    //Is Empaty
    println!("Is Empaty: {}",msg2.is_empty());
    
    
    //Contains
    println!("Contains: 'World' {}",msg2.contains("world"));

    //Replace
    println!("Repace: '{}' ",msg2.replace("world","there"));

    //Loop through string by whitespace

    for word in msg2.split_whitespace() {
        println!("{}",word);
    }

    //Create String with capacity
    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');
    println!("{}",s);

    // Assertion Testing

    assert_eq!(12,msg2.len());
}