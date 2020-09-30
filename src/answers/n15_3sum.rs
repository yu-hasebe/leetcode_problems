pub struct Solution;

// submitted code starts here
use std::collections::HashSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let nums = nums;

        let mut ret: HashSet<Vec<i32>> = HashSet::new();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let want = -nums[i] - nums[j];
                if let Ok(k) = nums.binary_search(&want) {
                    if j < k {
                        ret.insert(vec![nums[i], nums[j], nums[k]]);
                    }
                }
            }
        }
        ret.into_iter().collect::<Vec<Vec<i32>>>()
    }
}
// submitted code ends here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let expect = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        let get = Solution::three_sum(nums);
        assert_eq!(expect, get);
    }
}
