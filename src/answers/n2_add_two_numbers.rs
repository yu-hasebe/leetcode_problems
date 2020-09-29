pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// submitted code starts here
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut ret = vec![];
        let (mut overflow, mut l1_end, mut l2_end) = (false, false, false);
        loop {
            let v1 = match l1 {
                Some(n) => {
                    l1 = n.next;
                    n.val
                }
                None => {
                    l1_end = true;
                    0
                }
            };
            let v2 = match l2 {
                Some(n) => {
                    l2 = n.next;
                    n.val
                }
                None => {
                    l2_end = true;
                    0
                }
            };

            if !overflow && l1_end && l2_end {
                break;
            }

            let mut sum = v1 + v2 + if overflow { 1 } else { 0 };
            if sum >= 10 {
                overflow = true;
                sum -= 10;
            } else {
                overflow = false;
            }
            ret.push(sum);
        }
        Solution::create_linked_list(ret)
    }

    fn create_linked_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut ret = None;
        for &num in nums.iter().rev() {
            let mut list_node = ListNode::new(num);
            list_node.next = ret;
            ret = Some(Box::new(list_node));
        }
        ret
    }
}
// submitted code ends here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let l1 = Solution::create_linked_list(vec![2, 4, 3]);
        let l2 = Solution::create_linked_list(vec![5, 6, 4]);
        let expect = Solution::create_linked_list(vec![7, 0, 8]);
        let get = Solution::add_two_numbers(l1, l2);
        assert_eq!(expect, get);
    }
}
