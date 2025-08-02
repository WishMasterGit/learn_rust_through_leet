#[cfg(test)]
mod excercise_tests {
    use crate::exercises::*;
    use crate::util::*;
    #[test]
    fn test_two_sum() {
        let result_a = two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result_a.status, Status::Success);
        assert_eq!(result_a.data, vec![0, 1]);

        let result_b = two_sum(vec![3, 2, 4], 6);
        assert_eq!(result_b.status, Status::Success);
        assert_eq!(result_b.data, vec![1, 2]);

        let result_b = two_sum(vec![3, 3], 6);
        assert_eq!(result_b.status, Status::Success);
        assert_eq!(result_b.data, vec![0, 1]);
    }

    #[test]
    fn test_result_new() {
        let result = Result::new(Status::Success, 42);
        assert_eq!(result.status, Status::Success);
        assert_eq!(result.data, 42);
    }

    #[test]
    fn test_result_fail() {
        let result: Result<i32> = Result::fail();
        assert_eq!(result.status, Status::Fail);
        assert_eq!(result.data, 0); // Default for i32 is 0
    }

    #[test]
    fn test_split_to_digits_single_digit() {
        let digits = split_to_digits(7);
        assert_eq!(digits, vec![7]);
    }

    #[test]
    fn test_split_to_digits_multiple_digits() {
        let digits = split_to_digits(12345);
        assert_eq!(digits, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_split_to_digits_zero() {
        let digits = split_to_digits(0);
        assert_eq!(digits, vec![0]);
    }

    #[test]
    fn test_split_to_digits_with_trailing_zeros() {
        let digits = split_to_digits(1000);
        assert_eq!(digits, vec![1, 0, 0, 0]);
    }
    #[test]
    fn test_is_palindrome() {
        let num = 121;
        let result = is_palindrome(num);
        assert!(result.data);

        let num = -121;
        let result = is_palindrome(num);
        assert!(!result.data); // Negative numbers are not palindromes.

        let num = 10;
        let result = is_palindrome(num);
        assert!(!result.data);

        let num = 0;
        let result = is_palindrome(num);
        assert!(result.data);
    }
    #[test]
    fn test_roman_to_int() {
        assert_eq!(roman_to_int("III".to_string()).data, 3);
        assert_eq!(roman_to_int("IV".to_string()).data, 4);
        assert_eq!(roman_to_int("IX".to_string()).data, 9);
        assert_eq!(roman_to_int("LVIII".to_string()).data, 58);
        assert_eq!(roman_to_int("MCMXCIV".to_string()).data, 1994);
        assert_eq!(roman_to_int("MMMCMXCIX".to_string()).data, 3999);
    }

    #[test]
    fn test_roman_to_int_v2() {
        assert_eq!(roman_to_int_v2("III".to_string()).data, 3);
        assert_eq!(roman_to_int_v2("IV".to_string()).data, 4);
        assert_eq!(roman_to_int_v2("IX".to_string()).data, 9);
        assert_eq!(roman_to_int_v2("LVIII".to_string()).data, 58);
        assert_eq!(roman_to_int_v2("MCMXCIV".to_string()).data, 1994);
        assert_eq!(roman_to_int_v2("MMMCMXCIX".to_string()).data, 3999);
    }
    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ])
            .data,
            "fl"
        );
        assert_eq!(
            longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ])
            .data,
            ""
        );
        assert_eq!(
            longest_common_prefix(vec!["aca".to_string(), "cba".to_string()]).data,
            ""
        );
        assert_eq!(longest_common_prefix(vec!["a".to_string()]).data, "a");
    }
}
