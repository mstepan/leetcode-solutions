pub struct Solution {}

/**
* 58. Length of Last Word

*
* https://leetcode.com/problems/length-of-last-word/
*/
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut i: i32 = (s.len() - 1) as i32;

        while i >= 0 && Self::char_at(&s, i) == ' ' {
            i -= 1;
        }

        if i < 0 {
            return 0;
        }

        let mut j = i - 1;

        while j >= 0 && Self::char_at(&s, j) != ' ' {
            j -= 1;
        }

        i - j
    }

    fn char_at(s: &str, index: i32) -> char {
        s.chars().nth(index as usize).unwrap()
    }
}
