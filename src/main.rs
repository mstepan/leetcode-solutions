use crate::leetcode::merge_sorted_array::Solution;

pub mod leetcode;

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let mut nums2 = vec![2, 5, 6];
    let n = 3;

    Solution::merge(&mut nums1, m, &mut nums2, n);

    print_vector(&nums1);

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
