use crate::leetcode::search_insert_position::Solution;

pub mod leetcode;

fn main() {
    let data: Vec<i32> = vec![1, 5, 8, 10, 25, 34, 40, 45];
    let target = 1;

    let index = Solution::search_insert(data, target);

    println!("index = {index}");

    println!("main done...");
}

fn print_vector(data: &Vec<i32>) {
    let mut data_str = String::with_capacity(data.len() * 4);

    data_str.push_str("[");

    for (i, val) in data.iter().enumerate() {
        if i == 0 {
            data_str.push_str(&format!("{}", val));
        } else {
            data_str.push_str(&format!(", {}", val));
        }
    }

    data_str.push_str("]");

    println!("{data_str}");
}
