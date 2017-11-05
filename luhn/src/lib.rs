pub fn is_valid(num_string: &str) -> bool {
    let digits: Vec<u32> = num_string.replace(" ", "").chars().map(|d| d.to_digit(10).unwrap()).collect();
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
