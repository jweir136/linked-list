pub mod node;

use crate::node::Node;

pub struct LinkedList<T> {
    root: Option<Box<Node<T>>>,
    size: usize
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            root: Option::None,
            size: 0
        }
    }
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn ll_new_test() {
        let _ll = LinkedList::<u32>::new();
    }
}