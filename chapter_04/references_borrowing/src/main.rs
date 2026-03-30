fn main() {
    let s1 = String::from("Hello");
    // This & is a reference that doesn't relocate 
    // the variable but it points at it.
    let len = calculate_length(&s1); 
    // Thanks to that we can reuse the variable
    // After passing it as a paremeter. The reference
    // does not own the value.
    println!("The length of '{s1}' is {len}.");
    
    // The opposite of referenceing is dereferencing
    // with the operator: *
    // The action of making a reference is called BORROWING
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    // We cannot modify references but &mut x is mutable
    let mut s = String::from("hello");
    change(&mut s);

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    // We cannot have more the one mutable reference to a value
}
