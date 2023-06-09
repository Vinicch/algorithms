use crate::TreeNode;

pub fn dfs<T: PartialEq + PartialOrd>(head: Option<Box<TreeNode<T>>>, needle: T) -> bool {
    let Some(current) = head else {
        return false;
    };

    if current.value == needle {
        return true;
    }

    if current.value > needle {
        return dfs(current.left, needle);
    }

    dfs(current.right, needle)
}

pub fn compare<T: PartialEq>(a: Option<Box<TreeNode<T>>>, b: Option<Box<TreeNode<T>>>) -> bool {
    if a.is_none() && b.is_none() {
        return true;
    }

    let Some(a) = a else {
        return false;
    };

    let Some(b) = b else {
        return false;
    };

    if a.value != b.value {
        return false;
    }

    compare(a.left, b.left) && compare(a.right, b.right)
}
