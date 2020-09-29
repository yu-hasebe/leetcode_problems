pub struct Solution;

// submitted code starts here
use std::collections::HashMap;

impl Solution {
    pub fn longest_substring_without_repeating_characters(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut ret = 0;
        for i in 0..s.len() {
            for j in i..s.len() {
                let flag = {
                    let mut flag = true;
                    let mut hash_set = std::collections::HashSet::new();
                    for k in i..=j {
                        if !hash_set.insert(s[k]) {
                            flag = false;
                        }
                    }
                    flag
                };
                if flag {
                    ret = std::cmp::max(ret, j as i32 - i as i32 + 1);
                }
            }
        }
        ret
    }
}
// submitted code ends here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let s = "abcabcbb".to_string();
        let expect = 3;
        let get = Solution::longest_substring_without_repeating_characters(s);
        assert_eq!(expect, get);
    }
}
