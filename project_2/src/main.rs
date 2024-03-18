mod avl_tree;

use avl_tree::AVLTree;

fn main() {
    let mut tree = AVLTree::new();
    tree.insert_pub(2);
    tree.insert_pub(3);
    tree.insert_pub(1);
    tree.insert_pub(5);
    tree.insert_pub(1324);

    tree.print_tree();
    //println!("{}", tree.count_leaves());
}
