type ChildNode<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: ChildNode<T>,
    right: ChildNode<T>,
}

fn node_value(node: &ChildNode<u8>) -> Option<u8> {
    match node {
        None => None,
        Some(b) => Some(b.value),
    }
}

fn contains<T>(opt: Option<T>, x: T) -> bool where T: PartialEq {
    match opt {
        Some(y) => y == x,
        None => false,
    }
}

fn unival_trees(node: &ChildNode<u8>) -> u8 {
    match node {
        None => 0,
        Some(n) => {
            let left_num = unival_trees(&n.left);
            let right_num = unival_trees(&n.right);
            let left_val = node_value(&n.left);
            let right_val = node_value(&n.right);
	    let is_unival = contains(left_val, n.value) && contains(right_val, n.value) || n.left.is_none() && n.right.is_none();
            let r = if is_unival {1} else {0};
            r + left_num + right_num
        }
    }
}

impl Node<u8> {
    pub fn new(value: u8, left: Node<u8>, right: Node<u8>) -> Self {
        Node::<u8> {
            value,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
    pub fn leaf(value: u8) -> Self {
        Node::<u8> {
            value,
            left: None,
            right: None,
        }
    }
}

fn build_tree() -> Node<u8> {
    let tree_1 = Node::new(1, Node::leaf(1), Node::leaf(1));
    let tree_2 = Node::new(0, tree_1, Node::leaf(0));
    Node::new(0, Node::leaf(1), tree_2)
}

fn main() {
    let tree = build_tree();
    println!("{}", unival_trees(&Some(Box::new(tree))));
}

#[test]
fn test_unival_trees() {
    let tree = build_tree();
    assert_eq!(unival_trees(&Some(Box::new(tree))), 5);
}
