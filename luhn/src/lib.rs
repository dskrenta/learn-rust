pub fn is_valid(num_string: &str) -> bool {
    let digits: Vec<u32> = num_string.chars().filter(|c| c.is_digit(10)).map(|d| d.to_digit(10).unwrap()).collect();
    if digits.len() < 2 {
        return false;
    }
    println!("{:?}, {}", digits, digits.len());
    let sum: u32 = digits.iter().rev().enumerate().map(|(i, digit)| {
        if (i + 2) % 2 == 0 {
            if digit * 2 > 9 {
                return (digit * 2) - 9
            }
            return digit * 2;
        }
        *digit
    }).sum();
    if sum % 10 == 0 {
        return true;
    }
    false
}
