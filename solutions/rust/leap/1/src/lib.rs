pub fn is_leap_year(year: u64) -> bool {
    let divisible_by_4: bool = year % 4 == 0;
    let divisible_by_100: bool = year % 100 == 0;
    let divisible_by_400: bool = year % 400 == 0;

    if (400..).contains(&year) {
        divisible_by_400 || (!divisible_by_100 && divisible_by_4)
    } else if (100..400).contains(&year) {
        !divisible_by_100 && divisible_by_4
    } else if (0..100).contains(&year) {
        divisible_by_4
    } else {
        false
    }
}
