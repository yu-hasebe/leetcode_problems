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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let length = {
            let mut list_node = head.as_ref();
            let mut length = 1;
            while list_node.unwrap().next.is_some() {
                length += 1;
                list_node = list_node.unwrap().next.as_ref();
            }
            length
        };

        if length == 1 && n == 1 {
            return None;
        } else if length == n {
            return head.unwrap().next;
        }

        let target = length - n - 1;
        let mut head = head;
        let mut list_node = head.as_mut();
        for _ in 0..target {
            list_node = list_node.unwrap().next.as_mut();
        }

        let next = list_node
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .take();
        list_node.as_mut().unwrap().next = next;

        head
    }
}
// submitted code ends here

#[cfg(test)]
mod tests {
    use super::*;

    fn create_lisked_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut ret = None;
        for &num in nums.iter().rev() {
            let mut list_node = ListNode::new(num);
            list_node.next = ret;
            ret = Some(Box::new(list_node));
        }
        ret
    }

    #[test]
    fn test_remove_nth_from_end() {
        let head = create_lisked_list(vec![1, 2, 3, 4, 5]);
        let n = 2;
        let expect = create_lisked_list(vec![1, 2, 3, 5]);
        let get = Solution::remove_nth_from_end(head, n);
        assert_eq!(expect, get);
    }

    #[test]
    fn test_remove_nth_from_end_2() {
        let head = create_lisked_list(vec![1, 2]);
        let n = 2;
        let expect = create_lisked_list(vec![2]);
        let get = Solution::remove_nth_from_end(head, n);
        assert_eq!(expect, get);
    }
}
