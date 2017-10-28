fn main() {
    println!("{}", fahrenheit_to_celsius(65.0));
    println!("{}", fahrenheit_to_celsius(32.0));
    println!("{}", fahrenheit_to_celsius(0.0));
    println!("{}", fahrenheit_to_celsius(70.0));
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}
