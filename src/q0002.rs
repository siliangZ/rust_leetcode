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
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut result = l1;
    let mut lhs = &mut result;
    let mut rhs = l2;
    let mut increment = 0;
    loop {
        if let Some(lhs_v) = lhs {
            if let Some(rhs_v) = rhs {
                // combine them together
                let sum = lhs_v.val + rhs_v.val + increment;
                lhs_v.val = sum % 10;
                increment = sum / 10;

                lhs = &mut lhs_v.next;
                rhs = rhs_v.next;
            } else {
                let sum = lhs_v.val + increment;
                lhs_v.val = sum % 10;
                increment = sum / 10;
                lhs = &mut lhs_v.next;
            }
        } else if let Some(rhs_v) = rhs {
            let sum = rhs_v.val + increment;
            increment = sum / 10;
            *lhs = Some(Box::new(ListNode::new(sum % 10)));
            lhs = &mut lhs.as_mut().unwrap().next;
            rhs = rhs_v.next;
        } else {
            if increment != 0 {
                *lhs = Some(Box::new(ListNode::new(increment)));
            }
            break;
        }
    }

    return result;
}
