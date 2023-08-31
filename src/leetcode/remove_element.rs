#[allow(dead_code)]
pub struct Solution {}

/**
* 27. Remove Element

*
* https://leetcode.com/problems/remove-element/
*/
#[allow(dead_code)]
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut last = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[last] = nums[i];
                last += 1;
            }
        }

        nums.truncate(last);

        last as i32
    }
}
