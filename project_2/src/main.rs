mod avl_tree;

use avl_tree::AVLTree;

fn main() {
    let tree = AVLTree::new();
    if tree.is_empty() {
        println!("Hello, world!");
    }
}
