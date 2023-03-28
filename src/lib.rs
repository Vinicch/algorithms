pub mod bfs;
pub mod bft;
pub mod bubble;
pub mod dfs;
pub mod dft;
pub mod quick;

pub struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}
