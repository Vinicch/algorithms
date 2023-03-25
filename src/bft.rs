use std::{collections::VecDeque, ops::Deref};

use crate::BinaryNode;

pub fn bft<T: Copy>(head: &BinaryNode<T>, path: &mut Vec<T>) {
    let mut queue = VecDeque::from([head]);

    while !queue.is_empty() {
        let Some(current)=queue.pop_front() else {
            continue;
        };

        path.push(current.value);

        if let Some(node) = current.left.deref() {
            queue.push_back(node);
        }

        if let Some(node) = current.right.deref() {
            queue.push_back(node);
        }
    }
}
