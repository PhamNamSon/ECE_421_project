mod avl_tree;

use avl_tree::AVLTree;

fn main() {
    let mut tree = AVLTree::new();
    tree.insert_pub(2);
    tree.insert_pub(3);
    tree.insert_pub(6);
    tree.insert_pub(5);

    tree.print_tree();
    //println!("{}", tree.count_leaves());
}
