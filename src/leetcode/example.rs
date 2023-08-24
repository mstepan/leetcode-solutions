pub struct Solution {}

/**
 * Leetcode title
 *
 * https://leetcode.com/problems/<problem-url>
 */
impl Solution {
    pub fn is_even(x: i32) -> bool {
        if x & 1 == 0 {
            return true;
        }
        return false;
    }

    pub fn is_odd(x: i32) -> bool {
        return !Solution::is_even(x);
    }
}
