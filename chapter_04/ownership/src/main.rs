fn main() {
    // Varbiable s is valid from the point 
    // it is declared to the moment it goes
    // out of scope, this type is known as
    // string literal

    let s_01 = "hello";

    // The more flexible type that is allocated
    // on the heap is known as String type, this
    // type can be mutable
    {
        let mut s_02 = String::from("hello");
        s_02.push_str(", world!");

        println!("The strings are {s_01} and {s_02}");
    }
    // When we hit } the drop() function 
    // releases the memory from the heap and the
    // allocated changes are lost
    
    println!("The first string is {s_01} and the second is no longer avalable");

    // Two integers pushed onto the stack,
    // because their size is known and that's why
    // the x is still available even after passing the
    // value to y
    let x = 5;
    let y = x;
    println!("The numers are {x} and {y}");
    {
        let mut s = String::from("hello");
        s = String::from("ahoy");
        // The original s goes out of scope and
        // rust drops it immediately
        println!("{s}, world");
    }
    {
        // Cloning method keeps the originally
        // allocated heap memory
        let s1 = String::from("Hello");
        let mut s2 = s1.clone();
        s2.push_str(", world!");
        println!("s1 = {s1}, s2 = {s2}");
    }

    // Returning ownership of parameters
    // {
    //     let s1 = String::from("hello");
    //     let (s2, len) = calculate_length(s1);
    //     println!("The length of'{s1}' is {len} ");
    // }

}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()returns the length of a string
    (s, length)
}
