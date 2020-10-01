pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// submitted code starts here
impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut ret = vec![];

        let mut c1 = l1.as_ref();
        let mut c2 = l2.as_ref();

        while c1.is_some() || c2.is_some() {
            let n1 = {
                if c1.is_some() {
                    c1.unwrap().val
                } else {
                    std::i32::MAX
                }
            };
            let n2 = {
                if c2.is_some() {
                    c2.unwrap().val
                } else {
                    std::i32::MAX
                }
            };

            if n1 <= n2 {
                ret.push(n1);
                c1 = c1.unwrap().next.as_ref();
            } else {
                ret.push(n2);
                c2 = c2.unwrap().next.as_ref();
            }
        }
        Self::create_linked_list(ret)
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
    fn test_merge_two_lists() {
        let l1 = Solution::create_linked_list(vec![1, 2, 4]);
        let l2 = Solution::create_linked_list(vec![1, 3, 4]);
        let expect = Solution::create_linked_list(vec![1, 1, 2, 3, 4, 4]);
        let get = Solution::merge_two_lists(l1, l2);
        assert_eq!(expect, get);
    }
}
