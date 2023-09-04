use std::cmp::max;

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    /*
    1458. Max Dot Product of Two Subsequences
    https://leetcode.com/problems/max-dot-product-of-two-subsequences/
    Uses top down dynamic programmign approach.
    time: O(N*M)
    space: O(N*M), but cane be reduced to O(min(N, M))
     */
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let rows = nums1.len();
        let cols = nums2.len();

        let mut opt: Vec<Vec<i32>> = vec![vec![0; cols]; rows];

        let mut max_so_far = nums1[0] * nums2[0];

        // fill [0][0] row and col value
        opt[0][0] = nums1[0] * nums2[0];

        // fill 0 row
        for (col, v2) in nums2.iter().enumerate().skip(1) {
            opt[0][col] = max(opt[0][col - 1], nums1[0] * v2);
        }

        // fill 0 col
        for (row, v1) in nums1.iter().enumerate().skip(1) {
            opt[row][0] = max(opt[row - 1][0], v1 * nums2[0]);
        }

        for (i, v1) in nums1.iter().enumerate().skip(1) {
            for (j, v2) in nums2.iter().enumerate().skip(1) {
                let mut best_cur = max(v1 * v2 + opt[i - 1][j - 1], v1 * v2);

                best_cur = max(best_cur, opt[i - 1][j]);

                best_cur = max(best_cur, opt[i][j - 1]);

                opt[i][j] = best_cur;

                max_so_far = max(max_so_far, best_cur);
            }
        }

        opt[rows - 1][cols - 1]
    }
}
