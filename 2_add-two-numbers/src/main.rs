use libs::list_node::create_list_node;
use libs::list_node::ListNode;

struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode<i32>>>,
        l2: Option<Box<ListNode<i32>>>,
    ) -> Option<Box<ListNode<i32>>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(n1), Some(n2)) => {
                let sum = n1.val + n2.val;
                if sum < 10 {
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Solution::add_two_numbers(n1.next, n2.next),
                    }))
                } else {
                    let carry = Some(Box::new(ListNode::new(1)));
                    Some(Box::new(ListNode {
                        val: sum - 10,
                        next: Solution::add_two_numbers(
                            Solution::add_two_numbers(carry, n1.next),
                            n2.next,
                        ),
                    }))
                }
            }
        }
    }
}

fn main() {
    let v1: Vec<i32> = vec![2, 4, 3];
    let v2: Vec<i32> = vec![5, 6, 4];
    let l1 = create_list_node(v1);
    let l2 = create_list_node(v2);
    let result = Solution::add_two_numbers(l1, l2);
    println!("Result: {:#?}", result);
}
