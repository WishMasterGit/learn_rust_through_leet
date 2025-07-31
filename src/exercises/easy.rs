use crate::util::*;
use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Result<Vec<i32>> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {
            return Result {
                status: Status::Success,
                data: vec![j, i as i32],
            };
        }
        map.insert(num, i as i32);
    }
    Result::fail()
}

pub fn is_palindrome(mut number: i32) -> Result<bool> {
    if number < 0 || (number != 0 && number % 10 == 0) {
        return Result::fail();
    }

    let mut reverted_number = 0;
    while number > reverted_number {
        reverted_number = reverted_number * 10 + number % 10;
        number /= 10;
    }
    if number == reverted_number || number == reverted_number / 10 {
        return Result::success(true);
    };
    return Result::fail();
}

pub fn roman_to_int(roman_number: String) -> Result<i32> {
    let roman_digits = HashMap::from([
        ("I", 1),
        ("V", 5),
        ("X", 10),
        ("L", 50),
        ("C", 100),
        ("D", 500),
        ("M", 1000),
        ("IV", 4),
        ("IX", 9),
        ("XL", 40),
        ("XC", 90),
        ("CD", 400),
        ("CM", 900),
    ]);
    let mut output = 0;
    let rev_chars = roman_number.chars().rev().collect::<Vec<char>>();
    let size = rev_chars.len();
    let mut iterator = rev_chars.iter().enumerate();
    while let Some((index, &number)) = iterator.next() {
        if index + 1 < size {
            let short_form = format!("{}{}", rev_chars[index + 1], rev_chars[index]);
            if let Some(&value) = roman_digits.get(short_form.as_str()) {
                output += &value;
                iterator.next();
            } else if let Some(&value) = roman_digits.get(number.to_string().as_str()) {
                output += value;
            }
        } else if let Some(&value) = roman_digits.get(number.to_string().as_str()) {
            output += value;
        }
    }
    Result::success(output)
}

pub fn roman_to_int_v2(roman_number: String) -> Result<i32> {
    let mut output = 0;
    let mut prev_increment = 0;
    for roman_digit in roman_number.chars().rev() {
        let increment = match roman_digit {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };
        if increment < prev_increment {
            output -= increment;
        } else {
            output += increment
        }
        prev_increment = increment
    }
    Result::success(output)
}

pub fn longest_common_prefix(words: Vec<String>) -> Result<String> {
    let mut max_len = 300;
    let mut ouptut = "";
    for str in words {
        if max_len > str.len() {
            max_len = str.len()
        }
    }

    Result::fail()
}
