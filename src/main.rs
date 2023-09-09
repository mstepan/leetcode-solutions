#![allow(dead_code)]
mod leetcode;
use crate::leetcode::combination_sum_4::Solution;

fn main() {
    let nums = vec![1, 2, 3];
    let target = 4;

    let res = Solution::combination_sum4(nums, target);

    println!("res = {res}");

    println!("main done...");
}

fn print_vector(data: &Vec<i32>) {
    let mut data_str = String::with_capacity(data.len() * 4);

    data_str.push('[');

    for (i, val) in data.iter().enumerate() {
        if i == 0 {
            data_str.push_str(&format!("{}", val));
        } else {
            data_str.push_str(&format!(", {}", val));
        }
    }

    data_str.push(']');

    println!("{data_str}");
}
