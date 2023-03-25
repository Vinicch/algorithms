pub mod bfs;
pub mod bft;
pub mod bubble;
pub mod dfs;
pub mod dft;
pub mod quick;

pub struct BinaryNode<T> {
    value: T,
    left: Box<Option<BinaryNode<T>>>,
    right: Box<Option<BinaryNode<T>>>,
}
