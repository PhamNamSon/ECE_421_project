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

    fn rotate_left(&mut self, mut x: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        // Some(ref mut a): a is an Rc
        if let Some(ref mut x_node) = x {
            
            // Move the right child out
            if let Some(ref mut y) = x_node.borrow_mut().right {
                if let Some(ref mut y_left) = y.borrow_mut().left {
                    x_node.borrow_mut().right = Some(Rc::clone(y_left)); // Set x's right child to y's left child
                    y.borrow_mut().left = Some(Rc::clone(x_node)); // Move x into y's left child

                    // Update heights
                    let x_node_height = 1 + std::cmp::max(
                        x_node.borrow().left.as_ref().map_or(0, |n| n.borrow().height),
                        x_node.borrow().right.as_ref().map_or(0, |n| n.borrow().height),
                    );

                    let y_height = 1 + std::cmp::max(
                        y.borrow().left.as_ref().map_or(0, |n| n.borrow().height),
                        y.borrow().right.as_ref().map_or(0, |n| n.borrow().height),
                    );

                    x_node.borrow_mut().height = x_node_height;
                    y.borrow_mut().height = y_height;

                    Some(Rc::clone(y))
                } else {
                    // Restore x_node's right child if y_left is None
                    x_node.borrow_mut().right = Some(Rc::clone(y));
                    Some(Rc::clone(x_node))
                }
            } else {
                // Restore x if its right child is None
                Some(Rc::clone(x_node))
            } 
        } else {
            None
        }
    }

    fn rotate_right(&mut self, mut y: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        if let Some(ref mut y_node) = y {

            if let Some(ref mut x) = y_node.borrow_mut().left {
                if let Some(ref mut x_right) = x.borrow_mut().right {
                    
                    y_node.borrow_mut().left = Some(Rc::clone(x_right));
                    x.borrow_mut().right = Some(Rc::clone(y_node));
    
                    // Update heights
                    let y_node_height = 1 + std::cmp::max(
                        y_node.borrow().left.as_ref().map_or(0, |n| n.borrow().height),
                        y_node.borrow().right.as_ref().map_or(0, |n| n.borrow().height),
                    );
    
                    let x_height = 1 + std::cmp::max(
                        x.borrow().left.as_ref().map_or(0, |n| n.borrow().height),
                        x.borrow().right.as_ref().map_or(0, |n| n.borrow().height),
                    );
    
                    y_node.borrow_mut().height = y_node_height;
                    x.borrow_mut().height = x_height;
    
                    Some(Rc::clone(x))
                } else {
                    // Restore y_node's left child if x_right is None
                    y_node.borrow_mut().left = Some(Rc::clone(x));
                    Some(Rc::clone(y_node))
                }
            } else {
                // Restore y if its left child is None
                Some(Rc::clone(y_node))
            }
        } else {
            None
        }
    }

    pub fn insert(&mut self, mut root: Option<Rc<RefCell<Node>>>, val: i64) {
        let new_node = Rc::new(RefCell::new(Node::new(val, 1))); // New node with height 1
        
        match root {
            None => {
                self.root = Some(new_node);
            }
            Some(ref node) => {
                let mut current_node = node.borrow_mut();
                if val < current_node.val {
                    if current_node.left.is_none() {
                        current_node.left = Some(new_node.clone());
                    } else {
                        self.insert(current_node.left.clone(), val);
                    }
                } else if val > current_node.val {
                    if current_node.right.is_none() {
                        current_node.right = Some(new_node.clone());
                    } else {
                        self.insert(current_node.right.clone(), val);
                    }
                }

                // Set node's height
                current_node.height = 1 + std::cmp::max(
                    current_node.left.as_ref().map_or(0, |n| n.borrow().height), // Node's height or 0 if node is None
                    current_node.right.as_ref().map_or(0, |n| n.borrow().height),
                );

                let balance = current_node.get_balance_factor();

                if balance > 1 {
                    if let Some(left_node) = &current_node.left {
                        let left_node_borrow = left_node.borrow();
                        if val < left_node_borrow.val {
                            // Left Left Case
                            //self.rotate_right(Some(node.clone())); // Rotate and return
                        } else {
                            // Left Right Case
                            self.rotate_left(Some(left_node.clone()));
                            //self.rotate_right(Some(node.clone())); // Rotate and return
                        }
                    }
                } else if balance < -1 {
                    if let Some(right_node) = &current_node.right {
                        let right_node_borrow = right_node.borrow();
                        if val > right_node_borrow.val {
                            // Right Right Case
                            self.rotate_left(Some(node.clone())); // Rotate and return
                        } else {
                            // Right Left Case
                            //self.rotate_right(Some(right_node.clone()));
                            self.rotate_left(Some(node.clone())); // Rotate and return
                        }
                    }
                }
            }
        }
    }
    
    pub fn delete(&self, val: i64) {

    }

    pub fn get_num_leaves(&self) -> i64 {
        2 // Replace with height
    }

    pub fn get_height(&self) -> i64 {
        if let Some(ref node) = self.root {
            node.borrow().height
        } else {
            0
        }
    }

    pub fn print_traversal(&self) {

    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn print_tree(&self) {

    }

}