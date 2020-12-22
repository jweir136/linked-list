pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}
