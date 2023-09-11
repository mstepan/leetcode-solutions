use std::cmp::max;

pub struct Solution {}

impl Solution {
    /**
     * 174. Dungeon Game
     * https://leetcode.com/problems/dungeon-game/
     *
     * Binary search for answer.
     *
     * time: O(lgM*N*N), where M = sum of all negative values, N = grid size
     *
     */
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let negative_path_value = Self::single_path_negative_sum(&dungeon);

        let mut lo = 1;
        let mut hi = i32::abs(negative_path_value) + 1;

        let mut min_hp_solution = hi;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;

            if Self::has_solution(mid, &dungeon) {
                min_hp_solution = mid;
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }

        min_hp_solution
    }

    fn single_path_negative_sum(dungeon: &Vec<Vec<i32>>) -> i32 {
        let mut diff_for_single_path = 0;

        // 0-row
        for &row_val in dungeon[0].iter() {
            if row_val < 0 {
                diff_for_single_path += row_val;
            }
        }

        // last col
        for row_idx in 1..dungeon.len() {
            let last_col_val = dungeon[row_idx][dungeon[0].len() - 1];

            if last_col_val < 0 {
                diff_for_single_path += last_col_val;
            }
        }

        diff_for_single_path
    }

    fn has_solution(hp_value: i32, dungeon: &Vec<Vec<i32>>) -> bool {
        let rows = dungeon.len();
        let cols = dungeon[0].len();

        let mut res = vec![vec![0; cols]; rows];

        res[0][0] = hp_value + dungeon[0][0];

        if res[0][0] <= 0 {
            res[0][0] = -1;
        }

        for row in 0..rows {
            for col in 0..cols {
                let mut cur_res = res[row][col];

                if row > 0 && res[row - 1][col] > 0 {
                    cur_res = res[row - 1][col] + dungeon[row][col]
                }

                if col > 0 && res[row][col - 1] > 0 {
                    cur_res = max(cur_res, res[row][col - 1] + dungeon[row][col])
                }

                if cur_res > 0 {
                    res[row][col] = cur_res;
                } else {
                    res[row][col] = -1;
                }
            }
        }

        res[rows - 1][cols - 1] > 0
    }
}
