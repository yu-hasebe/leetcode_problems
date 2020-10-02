pub struct Solution;

// submitted code starts here
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }

        let first_idx = {
            let (mut left_idx, mut right_idx) = (0, nums.len() - 1);
            if nums[left_idx] == target {
                left_idx as i32
            } else {
                while left_idx + 1 < right_idx {
                    let mid_idx = left_idx + (right_idx - left_idx) / 2;

                    if nums[mid_idx] >= target {
                        right_idx = mid_idx;
                    } else {
                        left_idx = mid_idx;
                    }
                }
                if nums[right_idx] == target {
                    right_idx as i32
                } else {
                    -1
                }
            }
        };
        let last_idx = {
            let (mut left_idx, mut right_idx) = (0, nums.len() - 1);
            if nums[right_idx] == target {
                right_idx as i32
            } else {
                while left_idx + 1 < right_idx {
                    let mid_idx = left_idx + (right_idx - left_idx) / 2;

                    if nums[mid_idx] <= target {
                        left_idx = mid_idx;
                    } else {
                        right_idx = mid_idx;
                    }
                }
                if nums[left_idx] == target {
                    left_idx as i32
                } else {
                    -1
                }
            }
        };
        vec![first_idx, last_idx]
    }
}
// submitted code ends here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_range() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;
        let expect = vec![3, 4];
        let get = Solution::search_range(nums, target);
        assert_eq!(expect, get);
    }
}
