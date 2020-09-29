pub struct Solution;

// submitted code starts here
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();

        let (mut left_idx, mut right_idx) = (0, 0);
        let mut hash_map: HashMap<char, usize> = HashMap::new();
        let mut ret = 0;

        while right_idx < s.len() {
            if let Some(old_idx) = hash_map.insert(s[right_idx], right_idx) {
                left_idx = std::cmp::max(left_idx, old_idx + 1);
            }
            ret = std::cmp::max(ret, right_idx - left_idx + 1);
            right_idx += 1;
        }
        ret as i32
    }
}
// submitted code ends here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        //let s = "abcabcbb".to_string();
        //let expect = 3;
        let s = "abba".to_string();
        let expect = 2;
        let get = Solution::length_of_longest_substring(s);
        assert_eq!(expect, get);
    }
}
