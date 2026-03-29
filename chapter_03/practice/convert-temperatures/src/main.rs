fn main() {
    let converted: f32 = convert_temp(52.0);
    println!("The celcius temperature is {converted}");
}

fn convert_temp(input: f32) -> f32 {
    let fahrenheit: f32 = input;
    let celsius: f32 = (fahrenheit - 32.0) / 1.8;
    celsius
}
