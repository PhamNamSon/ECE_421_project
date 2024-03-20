use std::cell::RefCell;
use std::rc::Rc;
mod red_black_tree;
use red_black_tree::RedBlackTree;
mod avl_tree;
use avl_tree::AVLTree;
use avl_tree::Node;
use std::io;

fn main() {
    loop {
        let mut input = String::new();
        println!("Enter: ");
        println!("1. Red-Black Tree");
        println!("2. AVL Tree");
        println!("3. Exit");
        io::stdin().read_line(&mut input).expect("Failed to read input.");
        let choice: u32 = input.trim().parse().expect("Invalid input.");

        match choice {
            1 => {
                let mut tree = RedBlackTree::new();
                loop {
                    let mut input = String::new();
                    println!("Enter: ");
                    println!("1. Insert");
                    println!("2. Delete");
                    println!("3. Count the number of leaves");
                    println!("4. Return the height of the tree");
                    println!("5. Print In-order traversal of the tree");
                    println!("6. Check if the tree is empty");
                    println!("7. Print the tree showing its color and structure");
                    println!("8. Exit");
                    io::stdin().read_line(&mut input).expect("Failed to read input.");
                    let choice: u32 = input.trim().parse().expect("Invalid input.");
                    
                    loop {
                        match choice {
                            1 => {
                                let mut input = String::new();
                                println!("Enter the value to insert: ");
                                io::stdin().read_line(&mut input).expect("Failed to read input.");
                                let value: i32 = input.trim().parse().expect("Invalid input.");
                                tree.insert(value);
                                break;
                            }
                            2 => {
                                let mut input = String::new();
                                println!("Enter the value to delete: ");
                                io::stdin().read_line(&mut input).expect("Failed to read input.");
                                let value: i32 = input.trim().parse().expect("Invalid input.");
                                tree.delete(value);
                                break;
                            }
                            3 => {
                                println!("Number of leaves: {}", tree.count_leaves());
                                break;
                            }
                            4 => {
                                println!("Height of the tree: {}", tree.height());
                                break;
                            }
                            5 => {
                                println!("In-order traversal: ");
                                tree.print_in_order();
                                break;
                            }
                            6 => {
                                println!("Is the tree empty? {}", if tree.is_empty() { "Yes" } else { "No" });
                                break;
                            }
                            7 => {
                                // tree.borrow().print_tree();
                                break;
                            }
                            8 => {
                                println!("Exiting...");
                                return;
                            }
                            _ => {
                                println!("Invalid choice.");
                                break;
                            }
                        }
                    
                    }
                }
            }
            2 => {
                let tree = AVLTree::new();
                // Perform operations on AVLTree
            }
            3 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice.");
            }
        }
    }
}
