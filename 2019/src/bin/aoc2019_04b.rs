use std::error::Error;

/*
    It is a six-digit number.
    The value is within the range given in your puzzle input.
    Two adjacent digits are the same (like 22 in 122345).
    Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
    the two adjacent matching digits are not part of a larger group of matching digits
*/
fn valid_password(password: u32) -> bool {
    if password < 100_000 || password > 999_999 {
        return false;
    }
    let password = password.to_string();
    let mut previous_digit = None;
    let mut seen_valid_duplicate = false;
    let mut digit_repetitions = 1u8;
    for digit in password.chars() {
        if let Some(previous_digit) = previous_digit {
            if digit < previous_digit {
                return false;
            }
            if digit == previous_digit {
                digit_repetitions += 1;
            } else {
                if digit_repetitions == 2 {
                    seen_valid_duplicate = true;
                }
                digit_repetitions = 1;
            }
        }
        previous_digit = Some(digit);
    }
    // Edge case: Two identical consecutive digits at the end of the password.
    seen_valid_duplicate = seen_valid_duplicate || digit_repetitions == 2;

    seen_valid_duplicate
}

#[test]
fn example1() {
    assert!(valid_password(112233));
}

#[test]
fn example2() {
    assert!(!valid_password(123444));
}

#[test]
fn example3() {
    assert!(valid_password(111122));
}

fn main() -> Result<(), Box<dyn Error>> {
    let min = 172851;
    let max = 675869;

    let mut valid_passwords = 0u32;

    for i in min..=max {
        if valid_password(i) {
            valid_passwords += 1;
        }
    }

    println!("Found {} valid passwords.", valid_passwords);

    Ok(())
}
