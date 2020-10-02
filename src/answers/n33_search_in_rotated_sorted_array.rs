pub struct Solution;

// submitted code starts here
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left_idx, mut right_idx) = (0, nums.len() - 1);
        if nums[left_idx] == target {
            return left_idx as i32;
        } else if nums[right_idx] == target {
            return right_idx as i32;
        }

        // Keeps nums[left_idx] not to become the target.
        while left_idx + 1 < right_idx {
            let mid_idx = left_idx + (right_idx - left_idx) / 2;
            let (left, mid, right) = (nums[left_idx], nums[mid_idx], nums[right_idx]);

            let is_valid = {
                let has_no_pivot = left <= right;
                let has_pivot_on_the_left = !has_no_pivot && mid < right;
                let has_pivot_on_the_right = !has_no_pivot && left < mid;

                let is_valid_with_no_pivot = has_no_pivot && target <= mid;
                let is_valid_with_pivot_on_the_left =
                    has_pivot_on_the_left && (target <= mid || left < target);
                let is_valid_with_pivot_on_the_right =
                    has_pivot_on_the_right && left < target && target <= mid;

                is_valid_with_no_pivot
                    || is_valid_with_pivot_on_the_left
                    || is_valid_with_pivot_on_the_right
            };

            if is_valid {
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
}
// submitted code ends here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        let expect = 4;
        let get = Solution::search(nums, target);
        assert_eq!(expect, get);
    }

    #[test]
    fn test_search2() {
        let nums = vec![1];
        let target = 1;
        let expect = 0;
        let get = Solution::search(nums, target);
        assert_eq!(expect, get);
    }
}
