use crate::leetcode::example::Solution;

pub mod leetcode;

fn main() {
    Solution::print_me(133);

    println!("main done...");
}

#[allow(dead_code)]
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
