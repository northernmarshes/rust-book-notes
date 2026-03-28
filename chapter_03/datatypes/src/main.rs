fn main() {
    // Scalar types
    // Integers:
    
    // Signed
    let number: i32 = "42".parse().expect("Not a number!");
    println!("The numer is {number}");

    //Unsigned
    let unsigned_number: i32 = "52".parse().expect("Not a number!");
    println!("The unsigned numer is {unsigned_number}");

    // Floats:
    let x = 2.0;
    let y: f32 = 3.0;
    
    // Numeric operations:
    // Sum
    let sum = x + y;
    println!("The sum is {sum}");

    // Substraction
    let difference = y - x;
    println!("The difference is {difference}");

    // Multiplication
    let product = number * unsigned_number;
    println!("The product is {product}");

    // Division
    let quotinent = 56.7 / 32.2;
    let truncated = -5 / 3;
    println!("The quotinent is {quotinent}");
    println!("The truncated is {truncated}");

    // Reminder
    let reminder = 43 % 5;
    println!("The reminder is {reminder}");

    // Boolean
    let t = true;
    let f: bool = false;
    println!("The boolean is {t}!");

    // Chars

    let c = 'z';
    let z: char = 'u';
    println!("Chars are: {c}, {z}");

    // Compound type variables:

    // Tuple
    let typ: (i32, f64, u8) = (500, 6.4, 1);
    let first_value = typ.0;
    println!("One of the tuple values is {first_value}.");

    // Arrays are collections of single type with fixed lengh:

    let numbers = [1, 2, 3, 4, 5];
    
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let third_month = months[2];
    println!("{third_month}");

    let fixed_type_numbers: [i32, 5] = [1, 2, 3, 4, 5];
    let array_4 = [3; 5];
}

