fn main() {
    fibonacci(10);
}

fn fibonacci(number: i32) {
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 1;
    for _ in 0..number {
        println!("A is {a} and B is {b}");
        a = b + c;
        c = b;
        b = a;
    };
}
