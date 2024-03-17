mod red_black_tree;
use red_black_tree::RedBlackTree;

fn main() {
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
    // tree.print_in_order();
}
