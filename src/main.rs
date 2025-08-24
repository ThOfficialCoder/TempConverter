enum TempConversions {
    CelsiusFahrenheit,
    FahrenheitCelsius,
    FahrenheitKelvin,
    KelvinFahrenheit,
}

fn main() {
    let celsius = 100;
    let fahrenheit = 225;
    let kelvin: f32 = 50.0;
    
    convert_temp(TempConversions::CelsiusFahrenheit, celsius as f32);
    convert_temp(TempConversions::FahrenheitCelsius, fahrenheit as f32);
    convert_temp(TempConversions::FahrenheitKelvin, fahrenheit as f32);
    convert_temp(TempConversions::KelvinFahrenheit, kelvin);
}

fn convert_temp(num: TempConversions, value: f32) {
    match num {
        TempConversions::CelsiusFahrenheit => println!("Celsius in Fahrenheit is {}", celsius_fahrenheit(value)),
        TempConversions::FahrenheitCelsius => println!("Fahrenheit in Celsius is {}", fahrenheit_celsius(value)),
        TempConversions::FahrenheitKelvin => println!("Fahrenheit in Kelvin is {}", fahrenheit_kelvin(value)),
        TempConversions::KelvinFahrenheit => println!("Kelvin in Fahrenheit is {}", kelvin_fahrenheit(value)),
    }
} 

fn celsius_fahrenheit(celsius: f32) -> f32 {
    let result = (celsius * 9.0/5.0) + 32.0;
    return result;
}

fn fahrenheit_celsius(fahrenheitc: f32) -> f32 {
    let result = (fahrenheitc - 32.0) * 5.0/9.0;
    return result;
}

fn fahrenheit_kelvin(fahrenheitk: f32) -> f32 {
    let result = (fahrenheitk - 32 as f32) * 5.0/9.0 + 273.15;
    return result;
}

fn kelvin_fahrenheit(kelvin: f32) -> f32 {
    let result = (kelvin - 273.15) * 9.0/5.0 + 32 as f32;
    return result;
}