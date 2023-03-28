use crate::TreeNode;

pub fn pre_order<T: Copy>(node: Option<Box<TreeNode<T>>>, path: &mut Vec<T>) {
    let Some(current) = node else {
        return;
    };

    // pre
    path.push(current.value);

    // recurse
    pre_order(current.left, path);
    pre_order(current.right, path);

    // post
}

pub fn in_order<T: Copy>(node: Option<Box<TreeNode<T>>>, path: &mut Vec<T>) {
    let Some(current) = node else {
        return;
    };

    // pre

    // recurse
    in_order(current.left, path);
    path.push(current.value);
    in_order(current.right, path);

    // post
}

pub fn post_order<T: Copy>(node: Option<Box<TreeNode<T>>>, path: &mut Vec<T>) {
    let Some(current) = node else {
        return;
    };

    // pre

    // recurse
    post_order(current.left, path);
    post_order(current.right, path);

    // post
    path.push(current.value);
}
