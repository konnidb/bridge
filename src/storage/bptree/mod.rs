use self::node::Node;

pub mod node;

#[derive(Debug)]
pub struct BinPlusTree<'a, 'b> {
    root: &'a mut Node<'b>
}