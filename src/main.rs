fn main() {
    let result = celsius_fahrenheit(25);
    println!("Result of conversion is {}F", result);
}

fn celsius_fahrenheit(number: u32) -> u32 {
    let celsius = (number * 9/5) + 32;
    return celsius;
}
