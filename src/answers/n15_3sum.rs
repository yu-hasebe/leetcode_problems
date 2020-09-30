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
            let (mut j, mut k) = (i + 1, nums.len() - 1);
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum > 0 {
                    k -= 1;
                } else if sum < 0 {
                    j += 1;
                } else {
                    ret.insert(vec![nums[i], nums[j], nums[k]]);
                    j += 1;
                    k -= 1;
                }
            }
        }
        let mut ret = ret.into_iter().collect::<Vec<Vec<i32>>>();
        ret.sort();
        ret
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
