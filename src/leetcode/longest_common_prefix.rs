#[allow(dead_code)]
pub struct Solution {}

/**
 * 14. Longest Common Prefix
 *
 * https://leetcode.com/problems/longest-common-prefix/
 */
#[allow(dead_code)]
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::from("");
        }

        let mut longest_prefix = Solution::find_shortest(&strs);

        for val in strs.iter() {
            longest_prefix = Solution::find_longest_prefix(longest_prefix, val);
        }

        longest_prefix.to_string()
    }

    fn find_shortest(strs: &[String]) -> String {
        let mut shortest_str = &strs[0];

        for val in strs.iter() {
            if val.len() < shortest_str.len() {
                shortest_str = val;
            }
        }

        shortest_str.clone()
    }

    fn find_longest_prefix(cur_prefix: String, val: &str) -> String {
        let mut last_idx: i32 = -1;

        for (i, ch) in cur_prefix.chars().enumerate() {
            if i == val.len() || ch != val.chars().nth(i).unwrap() {
                break;
            }
            last_idx = i as i32;
        }

        if last_idx == -1 {
            return String::from("");
        }

        let last_idx: usize = last_idx as usize;

        cur_prefix[0..last_idx + 1].to_string()
    }
}
