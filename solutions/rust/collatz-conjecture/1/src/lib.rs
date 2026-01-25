const LIMIT: u64 = u64::MAX / 3 - 1;

pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut steps = 0u64;
    let mut result = n;
    while steps != u64::MAX {
        if result == 1 {
            return Some(steps);
        }
        if result % 2 == 0 {
            result /= 2;
        } else if result <= LIMIT {
            result = result * 3 + 1;
        } else {
            return None;
        }
        steps += 1;
    }
    None
}
