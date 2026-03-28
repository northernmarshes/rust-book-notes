fn main() {
    // Variables are immutable by default
    let x = 5;
    println!("The value of x is: {x}");

    // Shadowing of a variable 
    let x = x + 1;
    {
        // Shadowing in an inner scope
        let x = x * 2;
        println!("The value of x is the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Constant - UPPERCASE is a naming convention
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");


}
