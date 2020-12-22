pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Node {
            data: data,
            next: next
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_new_test() {
        let n1 = Node::<u32>::new(0, Option::None);
        let n2 = Node::<u32>::new(1, Option::Some(Box::new(n1)));
    }
}