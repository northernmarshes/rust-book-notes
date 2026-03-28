fn main() {
    println!("Hello, world!");
    another_function(5, 'h');
    
    // Statements don't return any value, 
    // Expressions evaluate to a return value
    
}
fn another_function(value: i32, unit_label: char) {
    print!("The measurement is {value}{unit_label}");


}
