use core::panic;

#[allow(dead_code)]
pub struct Solution {}

/**
* 20. Valid Parentheses
*
* https://leetcode.com/problems/valid-parentheses/
*/
#[allow(dead_code)]
impl Solution {
    pub fn is_valid(s: &str) -> bool {
        let mut stack_parenthesis: Vec<char> = vec![];

        for ch in s.chars() {
            if ch == '[' || ch == '{' || ch == '(' {
                stack_parenthesis.push(ch);
            } else if ch == ']' || ch == '}' || ch == ')' {
                if stack_parenthesis.is_empty() {
                    return false;
                }
                let char_to_match = stack_parenthesis.pop().unwrap();

                if !Solution::is_matched(char_to_match, ch) {
                    return false;
                }
            } else {
                panic!("Unknown character detected");
            }
        }

        stack_parenthesis.is_empty()
    }

    fn is_matched(open_par: char, close_par: char) -> bool {
        if open_par == '(' && close_par == ')' {
            return true;
        }

        if open_par == '[' && close_par == ']' {
            return true;
        }

        if open_par == '{' && close_par == '}' {
            return true;
        }

        false
    }
}
