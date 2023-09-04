#![allow(dead_code)]
mod leetcode;
use crate::leetcode::find_median_from_data_stream::MedianFinder;

fn main() {
    let mut obj = MedianFinder::new();
    obj.add_num(10);
    println!("res = {}", obj.find_median());

    obj.add_num(2);
    println!("res = {}", obj.find_median());

    obj.add_num(12);
    println!("res = {}", obj.find_median());

    obj.add_num(8);
    println!("res = {}", obj.find_median());

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
