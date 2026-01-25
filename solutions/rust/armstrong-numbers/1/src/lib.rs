pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let arm: u32 = digits.iter().map(|d| d.pow(digits.len() as u32)).sum();
    arm == num
}
