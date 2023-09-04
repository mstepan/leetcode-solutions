mod leetcode;
use crate::leetcode::max_dot_product_of_two_subsequences::Solution;

fn main() {
    let v1 = vec![-3, -8, 3, -10, 1, 3, 9];
    let v2 = vec![9, 2, 3, 7, -9, 1, -8, 5, -1, -1];

    let res = Solution::max_dot_product(v1, v2);

    println!("res = {res}");

    println!("main done...");
}

#[allow(dead_code)]
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
