use std::fmt::Display;

pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let code_trimmed = self.code.trim();
        if code_trimmed.len() <= 1 {
            return false;
        }

        match Luhn::luhn_checksum(code_trimmed) {
            Some(s) => {
                if s % 10 == 0 {
                    return true;
                } else {
                    return false;
                }
            }
            None => return false,
        }
    }

    fn luhn_checksum(code: &str) -> Option<u32> {
        let mut s = 0;
        for (i, val) in code
            .chars()
            .rev()
            .filter(|c| !c.is_whitespace())
            .enumerate()
        {
            if !val.is_digit(10) {
                return None;
            }

            let digit = val.to_digit(10).unwrap();
            if i % 2 != 0 {
                let b = digit * 2;
                if b > 9 {
                    s += b - 9;
                } else {
                    s += b;
                }
            } else {
                s += digit;
            }
        }
        Some(s)
    }
}

impl<T: Display> From<T> for Luhn {
    fn from(input: T) -> Self {
        Self {
            code: input.to_string(),
        }
    }
}
