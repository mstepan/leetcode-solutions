pub struct Solution {}

/**
 * 9. Palindrome Number (Easy)
 *
 * https://leetcode.com/problems/palindrome-number/
 */
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut val = x;

        let mut digits: Vec<u32> = vec![];

        while val > 0 {
            let single_digit = (val % 10) as u32;

            digits.push(single_digit);

            val /= 10;
        }

        Solution::is_vector_palindrome(&digits)
    }

    fn is_vector_palindrome(vec: &Vec<u32>) -> bool {
        if vec.is_empty() {
            return true;
        }

        let mut left = 0;
        let mut right = vec.len() - 1;

        while left < right {
            if vec[left] != vec[right] {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true
    }
}
