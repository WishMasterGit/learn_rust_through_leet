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
    Result::fail()
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

// roman_to_int_v2 is a more efficient version of roman_to_int
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
    let mut min_word_len = 300;
    let mut ouptut: String = String::new();
    for str in words.iter() {
        if min_word_len > str.len() {
            min_word_len = str.len()
        }
    }
    for i in 0..min_word_len {
        let mut current_char = ' ';
        let mut counter = 0;
        for word in words.iter() {
            if current_char == ' ' {
                current_char = word.chars().nth(i).unwrap();
                counter += 1;
            } else if current_char == word.chars().nth(i).unwrap() {
                counter += 1;
            } else if current_char != word.chars().nth(i).unwrap() {
                if !ouptut.is_empty() {
                    return Result::success(ouptut);
                }
                return Result::fail();
            }
        }
        if counter == words.len() {
            ouptut.push(current_char);
        }
    }
    if !ouptut.is_empty() {
        return Result::success(ouptut);
    }
    Result::fail()
}

pub fn longest_common_prefix_v2(words: Vec<String>) -> Result<String> {
    let min_word_len = words.iter().map(|w| w.len()).min().unwrap();
    let mut output = String::new();
    for i in 0..min_word_len {
        let current_char = words[0].chars().nth(i).unwrap();
        if words
            .iter()
            .all(|word| word.chars().nth(i).unwrap() == current_char)
        {
            output.push(current_char);
        } else {
            break;
        }
    }
    if !output.is_empty() {
        return Result::success(output);
    }
    Result::fail()
}

pub fn is_valid_parentheses(s: String) -> bool {
    let close_dict = HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);
    let mut validator = Vec::new();
    for c in s.chars() {
        if close_dict.contains_key(&c) {
            validator.push(close_dict[&c]);
        } else {
            let closing = validator.pop().unwrap();
            if closing == c {
                continue;
            } else {
                return false;
            }
        }
    }
    if validator.is_empty() {
        return true;
    }
    false
}

pub fn is_valid_parentheses_v2(s: String) -> bool {
    let close_dict = HashMap::from([(')', '('), (']', '['), ('}', '{')]);
    let mut validator = Vec::new();
    for c in s.chars() {
        if close_dict.contains_key(&c) {
            if validator.is_empty() || validator.pop().unwrap() != close_dict[&c] {
                return false;
            }
        } else {
            validator.push(c);
        }
    }
    if validator.is_empty() {
        return true;
    }
    false
}
