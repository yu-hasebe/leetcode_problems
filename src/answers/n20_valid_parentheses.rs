pub struct Solution;

// submitted code starts here
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            if Self::is_open(c) {
                stack.push(c);
            } else if Self::is_close(c) {
                if let Some(poped) = stack.pop() {
                    if !Self::is_match(poped, c) {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        stack.len() == 0
    }

    fn is_open(c: char) -> bool {
        match c {
            '(' | '{' | '[' => true,
            _ => false,
        }
    }

    fn is_close(c: char) -> bool {
        match c {
            ')' | '}' | ']' => true,
            _ => false,
        }
    }

    fn is_match(open: char, close: char) -> bool {
        if !Self::is_open(open) || !Self::is_close(close) {
            return false;
        }

        match (open, close) {
            ('(', ')') | ('{', '}') | ('[', ']') => true,
            _ => false,
        }
    }
}
// submitted code ends here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let s = "{[]}".to_string();
        let get = Solution::is_valid(s);
        assert!(get);
    }
}
