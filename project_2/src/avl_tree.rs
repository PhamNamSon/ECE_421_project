use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    val: i64,
    height: i64,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

pub struct AVLTree {
    root: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i64, height: i64) -> Self {
        Self { val, height, left: None, right: None }
    }

    fn get_balance_factor(&self) -> i64 {
        let left_height = if let Some(ref left) = self.left {
            left.borrow().height
        } else {
            0
        };

        let right_height = if let Some(ref right) = self.right {
            right.borrow().height
        } else {
            0
        };

        left_height - right_height
    }
}

impl AVLTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    fn rotate_left(&mut self, x: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        println!("ROTATE LEFT");
        if let Some(ref x_node) = x {
            println!("1");
            //let mut x_node_borrowed = x_node.borrow_mut();
            println!("2");
            let y = x_node.borrow().right.clone();
            println!("2");
            if let Some(ref y_ref_cell) = y {
                println!("3");
                let mut y_node = y_ref_cell.borrow_mut();
                if let Some(y_left) = y_node.left.take() {
                    x_node.borrow_mut().right = Some(Rc::clone(&y_left));
                    y_node.left = Some(x_node.clone());
    
                    // Update heights
                    let x_node_height = 1 + std::cmp::max(
                        x_node.borrow().left.as_ref().map_or(0, |n| n.borrow().height),
                        x_node.borrow().right.as_ref().map_or(0, |n| n.borrow().height),
                    );
    
                    let y_height = 1 + std::cmp::max(
                        y_node.left.as_ref().map_or(0, |n| n.borrow().height),
                        y_node.right.as_ref().map_or(0, |n| n.borrow().height),
                    );
    
                    x_node.borrow_mut().height = x_node_height;
                    y_node.height = y_height;
    
                    Some(y_left)
                } else {
                    //println!("TEST LEFT");
                    x_node.borrow_mut().right = Some(Rc::clone(&y_ref_cell));
                    Some(x_node.clone())
                }
            } else {
                Some(x_node.clone())
            }
        } else {
            None
        }
    }

    fn rotate_right(&mut self, mut y: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        println!("ROTATE RIGHT");
        if let Some(y_node) = y.take() {
            let x = y_node.borrow_mut().left.take();
            if let Some(x_ref_cell) = x.clone() {
                let mut x_node = x_ref_cell.borrow_mut();
                if let Some(x_right) = x_node.right.take() {
                    y_node.borrow_mut().left = Some(Rc::clone(&x_right));
                    x_node.right = Some(Rc::clone(&y_node));
    
                    // Update heights
                    let y_node_height = 1 + std::cmp::max(
                        y_node.borrow().left.as_ref().map_or(0, |n| n.borrow().height),
                        y_node.borrow().right.as_ref().map_or(0, |n| n.borrow().height),
                    );
    
                    let x_height = 1 + std::cmp::max(
                        x_node.left.as_ref().map_or(0, |n| n.borrow().height),
                        x_node.right.as_ref().map_or(0, |n| n.borrow().height),
                    );
    
                    y_node.borrow_mut().height = y_node_height;
                    x_node.height = x_height;
    
                    Some(x_right)
                } else {
                    // Restore y_node's left child if x_right is None
                    //println!("TEST RIGHT");
                    y_node.borrow_mut().left = Some(Rc::clone(&x_ref_cell));
                    Some(y_node)
                }
            } else {
                // Restore y if its left child is None
                Some(y_node)
            }
        } else {
            None
        }
    }

    pub fn insert_pub(&mut self, val: i64) {
        self.insert(self.root.clone(), val);
    }

    pub fn insert(&mut self, root: Option<Rc<RefCell<Node>>>, val: i64) -> Option<Rc<RefCell<Node>>> {
        let new_node = Rc::new(RefCell::new(Node::new(val, 1))); // New node with height 1
        match root {
            None => {
                self.root = Some(new_node.clone());
                Some(new_node)
            }
            Some(node) => {
                //let mut current_node = node.borrow_mut();
                if val < node.borrow().val {
                    let mut current_node = node.borrow_mut();
                    if current_node.left.is_none() {
                        current_node.left = Some(new_node.clone());
                    } else {
                        current_node.left = self.insert(current_node.left.take(), val);
                    }
                } else if val > node.borrow().val {
                    let mut current_node = node.borrow_mut();
                    if current_node.right.is_none() {
                        current_node.right = Some(new_node.clone());
                    } else {
                        current_node.right = self.insert(current_node.right.take(), val);
                    }
                }

                {
                    let mut node_borrow = node.borrow_mut();
                    // Set node's height
                    node_borrow.height = 1 + std::cmp::max(
                        node_borrow.left.as_ref().map_or(0, |n| n.borrow().height), // Node's height or 0 if node is None
                        node_borrow.right.as_ref().map_or(0, |n| n.borrow().height),
                    );
                }
                

                let balance = node.borrow().get_balance_factor();
                println!("{}", balance);
                if balance > 1 {
                    if let Some(left_node) = &mut node.borrow_mut().left {
                        if val < left_node.borrow().val {
                            // Left Left Case
                            println!("LEFT LEFT");
                            return self.rotate_right(Some(node.clone())); // Rotate and return (prev node.clone())
                        } else {
                            // Left Right Case
                            println!("LEFT RIGHT");
                            if let Some(rotated_node) = self.rotate_left(Some(left_node.clone())) { // prev left_node.clone()
                                *left_node = rotated_node;
                            }
                            //return self.rotate_right(current_node.left.take()); // Rotate and return (prev node.clone())
                            return self.rotate_right(Some(node.clone()));
                        }
                    }
                } else if balance < -1 {
                    if let Some(right_node) = &node.borrow().right {
                        if val > right_node.borrow().val {
                            // Right Right Case
                            println!("RIGHT RIGHT");

                            return self.rotate_left(Some(node.clone())); // Rotate and return (prev node.clone())
                        } else {
                            // Right Left Case
                            println!("RIGHT LEFT");
                            if let Some(right_node_mut) = &mut node.borrow_mut().right {
                                if let Some(rotated_node) = self.rotate_right(Some(right_node_mut.clone())) {
                                    *right_node_mut = rotated_node;
                                }
                                //return self.rotate_left(current_node.right.take()); // Rotate and return (prev node.clone())
                                return self.rotate_left(Some(node.clone()));
                            }
                            
                        }
                    }
                }
                
                Some(Rc::clone(&node))
            }
        }
    }
    
    pub fn delete(&self, val: i64) {

    }

    pub fn count_leaves(&self) -> i64 {
        self.count_leaves_helper(&self.root)
    }

    fn count_leaves_helper(&self, node: &Option<Rc<RefCell<Node>>>) -> i64 {
        if let Some(ref current) = node {
            let left_count = self.count_leaves_helper(&current.borrow().left);
            let right_count = self.count_leaves_helper(&current.borrow().right);
            if current.borrow().left.is_none() && current.borrow().right.is_none() {
                return left_count + right_count + 1; // Add 1 for the current leaf node
            } else {
                return left_count + right_count;
            }
        }
        0 // Empty tree
    }

    pub fn get_height(&self) -> i64 {
        if let Some(ref node) = self.root {
            node.borrow().height
        } else {
            0
        }
    }

    pub fn print_traversal(&self) {
        self.print_traversal_helper(&self.root);
    }

    fn print_traversal_helper(&self, node: &Option<Rc<RefCell<Node>>>) {
        if let Some(ref current) = node {
            self.print_traversal_helper(&current.borrow().left);
            println!("{}", current.borrow().val);
            self.print_traversal_helper(&current.borrow().right);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn print_node(node: &Node, depth: usize) {
        if let Some(right) = &node.right {
            let right_ref = &*right.borrow();
            Self::print_node(right_ref, depth + 1);
        }
    
        println!("{:indent$}{}", "", node.val, indent = depth * 4);
    
        if let Some(left) = &node.left {
            let left_ref = &*left.borrow();
            Self::print_node(left_ref, depth + 1);
        }
    }

    pub fn print_tree(&self) {
        match &self.root { 
            Some(node) => {Self::print_node(&*node.borrow(), 0)},
            None => {}
        }
    }

}