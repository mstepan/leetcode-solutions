use std::cmp::max;
pub struct Solution {}

impl Solution {
    /**
     * 164. Maximum Gap
     * https://leetcode.com/problems/maximum-gap/
     *
     * Use radix sorting by LSD (least significant digit).
     *
     * N = data.len()
     * D = total decimal digits count,according to description D <= 10
     *
     * time: O(N * D) = O(N*10)
     * space: O(N)
     */
    pub fn maximum_gap(data: Vec<i32>) -> i32 {
        if data.len() < 2 {
            return 0;
        }

        let mut max_digits_cnt = 0;

        for val in data.iter() {
            max_digits_cnt = max(max_digits_cnt, Self::count_decimal_digits(val));
        }

        let mut res = data;

        for idx in 0..max_digits_cnt {
            res = Self::sort_by_digit_at_position(&res, idx)
        }

        let mut max_gap = 0;

        for i in 1..res.len() {
            max_gap = max(max_gap, res[i] - res[i - 1]);
        }

        max_gap
    }

    fn count_decimal_digits(val: &i32) -> u8 {
        let mut cur = *val;

        let mut digits_cnt = 0;

        while cur != 0 {
            cur /= 10;
            digits_cnt += 1;
        }

        digits_cnt
    }

    fn sort_by_digit_at_position(data: &Vec<i32>, idx: u8) -> Vec<i32> {
        let mut buckets = [0; 10];

        for val in data.iter() {
            let cur_digit = Self::extract_decimal_digit(val, idx);
            buckets[cur_digit as usize] += 1;
        }

        Self::prefix_sum(&mut buckets);

        let mut res = vec![0; data.len()];

        for &val in data.iter().rev() {
            let digit = Self::extract_decimal_digit(&val, idx) as usize;
            buckets[digit] -= 1;
            let pos = buckets[digit];
            res[pos as usize] = val;
        }
        res
    }

    fn prefix_sum(arr: &mut [u32]) {
        for i in 1..arr.len() {
            arr[i] += arr[i - 1];
        }
    }

    fn extract_decimal_digit(val: &i32, idx: u8) -> u8 {
        ((val / i32::pow(10, idx as u32)) % 10) as u8
    }
}
