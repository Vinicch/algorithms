use std::collections::VecDeque;

use crate::TreeNode;

pub fn bft<T: Copy>(head: Box<TreeNode<T>>, path: &mut Vec<T>) {
    let mut queue = VecDeque::from([head]);

    while !queue.is_empty() {
        let Some(current)=queue.pop_front() else {
            continue;
        };

        path.push(current.value);

        if let Some(node) = current.left {
            queue.push_back(node);
        }

        if let Some(node) = current.right {
            queue.push_back(node);
        }
    }
}
