pub struct Solution {}

/**
* 28. Find the Index of the First Occurrence in a String

*
* https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/
*/
impl Solution {
    /**
     * N = haystack.len
     * M = needle.len
     * Brute-force serach with time: O(N*M) and space O(1)
     */
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() > haystack.len() {
            return -1;
        }

        if needle.len() == haystack.len() && haystack == needle {
            return 0;
        }

        let last = haystack.len() - needle.len() + 1;

        for i in 0..last {
            let mut j = 0;

            while j < needle.len() {
                if Solution::char_at(&needle, j) != Self::char_at(&haystack, i + j) {
                    break;
                }

                j += 1;
            }

            if j == needle.len() {
                return i as i32;
            }
        }

        -1
    }

    fn char_at(value: &str, index: usize) -> char {
        return value.chars().nth(index).unwrap();
    }
}
