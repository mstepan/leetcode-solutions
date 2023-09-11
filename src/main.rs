#![allow(dead_code)]
mod leetcode;
use crate::leetcode::hard::dungeon_game::Solution;

fn main() {
    //let dungeon = vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]];

    let dungeon = vec![vec![2], vec![1]];

    let min_hp = Solution::calculate_minimum_hp(dungeon);

    println!("min_hp = {min_hp}");

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
