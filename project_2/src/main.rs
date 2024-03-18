mod avl_tree;
use avl_tree::AVLTree;
use avl_tree::Node;
use std::cell::RefCell;
use std::rc::Rc;
mod red_black_tree;
use red_black_tree::RedBlackTree;

fn main() {
    // let mut tree = AVLTree::new();
    // //println!("{}", tree.get_height());
    // tree.insert_pub(16);
    // //println!("{}", tree.get_height());
    // tree.insert_pub(17);
    // //println!("{}", tree.get_height());
    // tree.insert_pub(15);
    // //println!("{}", tree.get_height());
    // tree.insert_pub(5);
    // //println!("{}", tree.get_height());
    // tree.insert_pub(6);
    // //println!("{}", tree.get_height());

    // // tree.print_traversal();
    // // println!("{}", tree.count_leaves());

    // tree.print_traversal();

    // Create nodes for the AVL tree
    let node_3 = Rc::new(RefCell::new(Node::new(3, 1)));
    let node_5 = Rc::new(RefCell::new(Node::new(5, 1)));
    let node_7 = Rc::new(RefCell::new(Node::new(7, 1)));
    let node_10 = Rc::new(RefCell::new(Node::new(10, 1)));
    let node_12 = Rc::new(RefCell::new(Node::new(12, 1)));
    let node_15 = Rc::new(RefCell::new(Node::new(15, 1)));
    let node_17 = Rc::new(RefCell::new(Node::new(17, 1)));

     // Set up the tree structure
     node_10.borrow_mut().set_left_child(Some(Rc::clone(&node_5)));
     node_10.borrow_mut().set_right_child(Some(Rc::clone(&node_15)));
 
     node_5.borrow_mut().set_left_child(Some(Rc::clone(&node_3)));
     node_5.borrow_mut().set_right_child(Some(Rc::clone(&node_7)));
 
     node_15.borrow_mut().set_left_child(Some(Rc::clone(&node_12)));
     node_15.borrow_mut().set_right_child(Some(Rc::clone(&node_17)));

    // Create an AVL tree with the root node
    let mut tree = AVLTree::new();
    tree.set_root(Some(Rc::clone(&node_10)));

    // Print the original tree
    println!("Original AVL Tree:");
    tree.print_tree();

    // Delete a node from the tree
    let value_to_delete = 5;
    tree.delete(value_to_delete);

    // Print the tree after deletion
    println!("\nAVL Tree after deleting node with value {}:", value_to_delete);
    tree.print_tree();


    let mut tree = RedBlackTree::new();
    tree.insert(7);
    tree.insert(3);
    tree.insert(18);
    tree.insert(10);
    tree.insert(22);
    tree.insert(8);
    tree.insert(11);
    tree.insert(26);
    println!("Number of leaves: {}", tree.count_leaves());
    println!("Height of the tree: {}", tree.height());
    tree.print_in_order();
}
