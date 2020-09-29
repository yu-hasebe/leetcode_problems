pub struct Solution;

// submitted code starts here
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s: Vec<char> = s.chars().collect();

        let (mut max_left_idx, mut max_right_idx) = (0, 0);
        for i in 0..s.len() {
            for &(left_idx, right_idx) in [(i, i), (i, i + 1)].iter() {
                let (mut left_idx, mut right_idx) = (left_idx, right_idx);
                while right_idx < s.len() {
                    if s[left_idx] == s[right_idx] {
                        if left_idx == 0 || right_idx + 1 == s.len() {
                            break;
                        } else {
                            left_idx -= 1;
                            right_idx += 1;
                        }
                    } else {
                        left_idx += 1;
                        right_idx -= 1;
                        break;
                    }
                }
                if right_idx < s.len() && right_idx + max_left_idx > max_right_idx + left_idx {
                    max_left_idx = left_idx;
                    max_right_idx = right_idx;
                }
            }
        }

        if s.len() == 0 {
            "".to_string()
        } else {
            s[max_left_idx..=max_right_idx].iter().collect::<String>()
        }
    }
}
// submitted code ends here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        let s = "babad".to_string();
        let expect = "bab".to_string();
        let get = Solution::longest_palindrome(s);
        assert_eq!(expect, get);
    }
}
