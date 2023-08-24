use crate::leetcode::remove_duplicates_from_sorted_array::Solution;

pub mod leetcode;

fn main() {
    let mut data = vec![1, 2, 2, 3, 4, 5, 5, 7, 7, 7];

    let unique_elems_count = Solution::remove_duplicates(&mut data);

    print_vector(&data);
    println!("unique_elems_count = {unique_elems_count}");

    println!("main done...");
}

fn print_vector(data: &Vec<i32>) {
    for val in data {
        print!("{val}, ");
    }
    println!();
}
