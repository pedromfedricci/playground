use libplayground::binary_tree::*;

fn main() {
    let subtree_l = make_node(BinaryTree::Empty, "mecha", BinaryTree::Empty);
    let subtree_rl = make_node(BinaryTree::Empty, "droid", BinaryTree::Empty);
    let subtree_r = make_node(subtree_rl, "robot", BinaryTree::Empty);
    let tree = make_node(subtree_l, "titan", subtree_r);

    let mut v = Vec::new();
    for kind in &tree {
        v.push(*kind);
    }

    assert_eq!(v, ["mecha", "titan", "droid", "robot"]);

    let string = String::from("asdasdasdsad");
    let subtree_l = make_node(BinaryTree::Empty, &string, BinaryTree::Empty);
    let string = String::from("qqqqqqqqqqqqq");
    let subtree_rl = make_node(BinaryTree::Empty, &string, BinaryTree::Empty);
    let string = String::from("wwwwwwwwwwwwwww");
    let subtree_r = make_node(subtree_rl, &string, BinaryTree::Empty);
    let string = String::from("aaaaaaaaaaaaaaa");
    let tree = make_node(subtree_l, &string, subtree_r);

    let mut v = Vec::new();
    for kind in &tree {
        v.push(*kind);
    }
}
