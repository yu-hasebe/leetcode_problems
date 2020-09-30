pub struct Solution;

// submitted code starts here
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left_idx, mut right_idx) = (0, height.len() - 1);
        let mut ret = 0;
        while left_idx < right_idx {
            let (left_height, right_height) = (height[left_idx], height[right_idx]);
            let current_area =
                (right_idx - left_idx) as i32 * std::cmp::min(left_height, right_height);
            ret = std::cmp::max(ret, current_area);

            if left_height <= right_height {
                while height[left_idx] <= left_height {
                    if left_idx == height.len() - 1 {
                        break;
                    }
                    left_idx += 1;
                }
            } else {
                while height[right_idx] <= right_height {
                    if right_idx == 0 {
                        break;
                    }
                    right_idx -= 1;
                }
            }
        }
        ret as i32
    }
}
// submitted code ends here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let expect = 49;
        let get = Solution::max_area(height);
        assert_eq!(expect, get);
    }
}
