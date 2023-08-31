#[allow(dead_code)]
pub struct Solution {}

/**
* 88. Merge Sorted Array

*
* https://leetcode.com/problems/merge-sorted-array/
*/
#[allow(dead_code)]
impl Solution {
    pub fn merge(res: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
        let mut res_idx = n + m - 1;

        let mut i = m - 1;
        let mut j = n - 1;

        while res_idx >= 0 {
            if i < 0 {
                res[res_idx as usize] = nums2[j as usize];
                j -= 1;
            } else if j < 0 || res[i as usize] >= nums2[j as usize] {
                res[res_idx as usize] = res[i as usize];
                i -= 1;
            } else {
                res[res_idx as usize] = nums2[j as usize];
                j -= 1;
            }

            res_idx -= 1;
        }
    }
}
