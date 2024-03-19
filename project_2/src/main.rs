mod avl_tree;

use avl_tree::AVLTree;


fn main() {
    let mut tree = AVLTree::new();
    
    tree.insert_pub(7);
    tree.insert_pub(24);
    tree.insert_pub(1);
    tree.insert_pub(35);
    tree.insert_pub(999);
    tree.insert_pub(154);
    tree.insert_pub(5);
    tree.insert_pub(2);
    tree.insert_pub(3);
    tree.insert_pub(4);
    tree.insert_pub(6);

    tree.print_tree();

}
