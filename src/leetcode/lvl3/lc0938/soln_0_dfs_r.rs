/// https://leetcode.com/problems/range-sum-of-bst/
/// 
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
/// 
/// Reference:
/// https://leetcode.com/problems/range-sum-of-bst/discuss/213098/Rust-recursive-solution
use crate::leetcode::util::data_structure::tree_node::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut sum = 0;
        
        if let Some(node) = root{
            let value = node.borrow().val;
            if value >= low && value <= high{
                sum += value;
            }
            
            if value >= low{
                sum += Self::range_sum_bst(node.borrow().left.clone(), low, high);
            }
            
            if value <= high{
                sum += Self::range_sum_bst(node.borrow().right.clone(), low, high);
            }
            
        }
        
        sum
    }
}