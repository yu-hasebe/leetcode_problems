pub struct Solution;

// submitted code starts here
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_map: HashMap<i32, usize> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let key = target - num;
            if let Some(&val) = hash_map.get(&key) {
                return vec![val as i32, i as i32];
            } else {
                hash_map.entry(num).or_insert(i);
            }
        }
        vec![]
    }
}
// submitted code ends here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expect = vec![0, 1];
        let get = Solution::two_sum(nums, target);
        assert_eq!(expect, get);
    }
}
