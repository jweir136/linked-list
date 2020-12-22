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

    pub fn get_data(&self) -> &T {
        &self.data
    }

    pub fn get_next(&self) -> Option<&Box<Node<T>>> {
        self.next.as_ref()
    }

    pub fn set_data(&mut self, data: T) {
        self.data = data;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_new_test() {
        let n1 = Node::<u32>::new(0, Option::None);
        let _n2 = Node::<u32>::new(1, Option::Some(Box::new(n1)));
    }

    #[test]
    fn node_getdata_test() {
        let n1 = Node::<u32>::new(0 as u32, Option::None);
        let n2 = Node::<u32>::new(1 as u32, Option::Some(Box::new(n1)));

        assert_eq!(*n2.get_data(), 1);
    }

    #[test]
    fn node_setdata_test() {
        let mut n1 = Node::<u32>::new(0 as u32, Option::None);
        n1.set_data(1 as u32);

        assert_eq!(*n1.get_data(), 1);
    }
}