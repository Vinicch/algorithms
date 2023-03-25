use std::collections::VecDeque;
use std::ops::Deref;

use crate::BinaryNode;

pub fn bfs<T: PartialEq>(head: &BinaryNode<T>, needle: T) -> bool {
    let mut queue = VecDeque::from([head]);

    while !queue.is_empty() {
        let Some(current) = queue.pop_front() else {
            continue;
        };

        if current.value == needle {
            return true;
        }

        if let Some(node) = current.left.deref() {
            queue.push_back(node);
        }

        if let Some(node) = current.right.deref() {
            queue.push_back(node);
        }
    }

    false
}
