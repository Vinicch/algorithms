use std::collections::VecDeque;

use crate::TreeNode;

pub fn bfs<T: PartialEq>(head: Box<TreeNode<T>>, needle: T) -> bool {
    let mut queue = VecDeque::from([head]);

    while !queue.is_empty() {
        let Some(current) = queue.pop_front() else {
            continue;
        };

        if current.value == needle {
            return true;
        }

        if let Some(node) = current.left {
            queue.push_back(node);
        }

        if let Some(node) = current.right {
            queue.push_back(node);
        }
    }

    false
}
