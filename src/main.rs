use crate::leetcode::scramble_string::Solution;

pub mod leetcode;

fn main() {
    let x = 30 * 30 * 30 * 30;

    println!("x = {x}");

    let s1 = String::from("great");
    let s2 = String::from("rgeat");

    // let s1 = String::from("hi");
    // let s2 = String::from("hi");

    let res = Solution::is_scramble(s1, s2);

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
