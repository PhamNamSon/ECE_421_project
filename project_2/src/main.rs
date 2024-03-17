mod avl_tree;

use avl_tree::AVLTree;

fn main() {
    let mut tree = AVLTree::new();
    println!("{}", tree.get_height());
    tree.insert_pub(16);
    println!("{}", tree.get_height());
    tree.insert_pub(17);
    println!("{}", tree.get_height());
    tree.insert_pub(18);
    println!("{}", tree.get_height());
}
