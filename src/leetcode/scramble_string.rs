use std::collections::HashMap;

#[allow(dead_code)]
pub struct Solution {}

/**
 * 87. Scramble String
 *
 * https://leetcode.com/problems/scramble-string/
 */
#[allow(dead_code)]
impl Solution {
    /**
     * Dynamic programming solution with time O(N^4)
     * N = 30, 30^4 = 810K
     */
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let mut cache: HashMap<(usize, usize, usize, usize), bool> = HashMap::new();
        Solution::has_solution(&s1, 0, s1.len() - 1, &s2, 0, s2.len() - 1, &mut cache)
    }

    fn has_solution(
        s1: &str,
        i1: usize,
        j1: usize,
        s2: &str,
        i2: usize,
        j2: usize,
        cache: &mut HashMap<(usize, usize, usize, usize), bool>,
    ) -> bool {
        let key = (i1, j1, i2, j2);

        if cache.contains_key(&key) {
            return *cache.get(&key).unwrap();
        }

        let elems_count = j1 - i1 + 1;

        if elems_count == 1 {
            let res = Solution::char_at(s1, i1) == Solution::char_at(s2, i2);
            cache.insert((i1, j1, i2, j2), res);
            return res;
        }

        if s1[i1..j1 + 1] == s2[i2..j2 + 1] {
            cache.insert(key, true);
            return true;
        }

        let mut res = false;
        for k in i1..j1 {
            let first_length = k - i1 + 1;
            let second_length = j1 - k;

            // direct scrumbling
            res = res
                || (Solution::has_solution(s1, i1, k, s2, i2, i2 + first_length - 1, cache)
                    && Solution::has_solution(s1, k + 1, j1, s2, i2 + first_length, j2, cache));

            // reverse parts
            res = res
                || (Solution::has_solution(s1, i1, k, s2, i2 + second_length, j2, cache)
                    && Solution::has_solution(
                        s1,
                        k + 1,
                        j1,
                        s2,
                        i2,
                        i2 + second_length - 1,
                        cache,
                    ));

            if res {
                break;
            }
        }

        cache.insert(key, res);
        res
    }

    fn char_at(s: &str, i: usize) -> char {
        s.chars().nth(i).unwrap()
    }
}
