pub struct Solution {}

/**
 * 26. Remove Duplicates from Sorted Array
 *
 * https://leetcode.com/problems/remove-duplicates-from-sorted-array/
 */
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut last: usize = 0;

        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[last + 1] = nums[i];
                last += 1;
            }
        }

        nums.truncate(last + 1);

        nums.len() as i32
    }
}
