mod leetcode;
use crate::leetcode::form_largest_integer_with_digits_that_add_up_to_target::Solution;

fn main() {
    let cost = vec![4, 3, 2, 5, 6, 7, 2, 5, 5];
    let target = 9;

    let largest_str = Solution::largest_number(cost, target);

    println!("largest_str = {largest_str}");

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
