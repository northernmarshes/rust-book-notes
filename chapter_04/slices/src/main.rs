fn main() {
    // Slice is a kind of reference
    // Function in rust typically don't
    let sentence = String::from("one two three");
    let word = first_word(&sentence);
    println!("{word}");

    // String Slice
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // If we match the end:
    let len = s.len();
    let slice = &s[7..len];
    let slice = &s[7..];

    let full_slice = &s[0..len];
    let full_slice = &s[..];

    println!("{hello}");
    println!("{world}");
    println!("{slice}");
    println!("{full_slice}");

    // The type of this variable is &str
    // It's a slice pointing to specific
    // Point of the binary. I's also immutable
    let n = "Hello, world!";
    println!("{n}");

    let a = [1, 2, 3, 4, 5, 6, 7,];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    // changes string to array of bytes
    let bytes = s.as_bytes(); 

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
