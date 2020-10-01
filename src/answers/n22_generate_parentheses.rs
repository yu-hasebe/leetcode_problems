pub struct Solution;

// submitted code starts here
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ret = vec![];
        Solution::recursion(n as usize, &mut ret, "".to_string(), 0, 0);
        ret
    }

    fn recursion(n: usize, ret: &mut Vec<String>, current: String, left: usize, right: usize) {
        if current.len() == 2 * n {
            ret.push(current);
            return;
        }
        if left < n {
            let mut next = current.clone();
            next.push('(');
            Solution::recursion(n, ret, next, left + 1, right);
        }
        if right < left {
            let mut next = current.clone();
            next.push(')');
            Solution::recursion(n, ret, next, left, right + 1);
        }
    }
}
// submitted code ends here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_parenthesis() {
        let n = 3;
        let expect = vec![
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string(),
        ];
        let get = Solution::generate_parenthesis(n);
        assert_eq!(expect, get);
    }
}
