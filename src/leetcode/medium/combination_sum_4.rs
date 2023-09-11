pub struct Solution {}

impl Solution {
    /**
     * 377. Combination Sum IV
     * https://leetcode.com/problems/combination-sum-iv/
     *
     * time: O(N*M)
     * space: O(N)
     *
     * N = target+1
     * M = nums.len()
     */
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut sol = vec![0; (target + 1) as usize];
        sol[0] = 1;

        for i in 1..sol.len() {
            let cur_target = i;
            let mut cur_sol = 0;

            for val in nums.iter() {
                let val = *val as usize;

                if cur_target >= val {
                    cur_sol += sol[cur_target - val];
                }
            }

            sol[i] = cur_sol;
        }

        sol[sol.len() - 1]
    }
}
