#[allow(dead_code)]
pub struct Solution {}

/**
* 1449. Form Largest Integer With Digits That Add up to Target

*
* https://leetcode.com/problems/form-largest-integer-with-digits-that-add-up-to-target/
*/
#[allow(dead_code)]
impl Solution {
    /**
     * Dynamic programmin top-wodn approach.
     * time: O(M)
     * space: O(M)
     */
    pub fn largest_number(cost: Vec<i32>, target: i32) -> String {
        let mut sol: Vec<Option<String>> = vec![None; (target + 1) as usize];
        sol[0] = Some("".to_string());

        for i in 1..sol.len() {
            let cur_target = i as usize;
            let mut max_cur: Option<String> = None;

            for (j, &cur_cost) in cost.iter().enumerate() {
                let digit = j + 1;

                let cur_cost = cur_cost as usize;

                if cur_target >= cur_cost {
                    if let Some(partial_str) = &sol[cur_target - cur_cost] {
                        let cur_str = digit.to_string() + partial_str;

                        if Solution::cmp_strings_as_numbers(&cur_str, &max_cur) > 0 {
                            max_cur = Some(cur_str);
                        }
                    }
                }
            }

            sol[i as usize] = max_cur;
        }

        match &sol[sol.len() - 1] {
            Some(val) => val.to_string(),
            None => "0".to_string(),
        }
    }

    #[allow(dead_code)]
    fn cmp_strings_as_numbers(first: &str, second: &Option<String>) -> i32 {
        match second {
            None => 1,
            Some(second_str) => {
                if first.len() > second_str.len() {
                    1
                } else if second_str.len() > first.len() {
                    -1
                } else if first > second_str {
                    1
                } else if first == second_str {
                    0
                } else {
                    -1
                }
            }
        }
    }
}
