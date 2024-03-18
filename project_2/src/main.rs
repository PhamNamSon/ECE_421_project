mod red_black_tree;
use red_black_tree::RedBlackTree;

fn main() {
    let mut tree = RedBlackTree::new();
    // tree.insert(12);
    // tree.insert(8);
    // tree.insert(15);
    // tree.insert(5);
    // tree.insert(9);
    // tree.insert(13);
    // tree.insert(19);
    // tree.insert(10);
    // tree.insert(23);
    // tree.delete(19);
    // tree.insert(1);
    // tree.delete(5);
    // tree.delete(12);

    tree.insert(20);
    tree.insert(10);
    tree.insert(30);
    tree.insert(25);
    tree.insert(35);
    tree.delete(10);

    // println!("Number of leaves: {}", tree.count_leaves());
    // println!("Height of the tree: {}", tree.height());
    // println!("Empty: {}", tree.is_empty());
    tree.print_in_order();
}
