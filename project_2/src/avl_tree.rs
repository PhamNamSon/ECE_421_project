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

    pub fn get_val(&self) -> i64 {
        return self.val;
    }

    pub fn set_left_child(&mut self, child: Option<Rc<RefCell<Node>>>) {
        self.left = child;
    }

    pub fn set_right_child(&mut self, child: Option<Rc<RefCell<Node>>>) {
        self.right = child;
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
  
    pub fn get_root(&self) -> Option<Rc<RefCell<Node>>>{
        self.root.clone()
    }

    pub fn set_root(&mut self, root: Option<Rc<RefCell<Node>>>) {
        self.root = root;
    }

    // return new root
    fn rotate_left(&mut self, x: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        if let Some(ref x_node) = x {
            let y = x_node.borrow().right.clone();
            if let Some(ref y_ref_cell) = y {
                let mut y_node = y_ref_cell.borrow_mut();
                if let Some(y_left) = y_node.left.take() {
                    println!("4");
                    y_node.left = Some(x_node.clone());
                    x_node.borrow_mut().right = Some(Rc::clone(&y_ref_cell));
                    /*
                    // Update heights
                    {
                        //let mut x_node_borrow_mut = x_node.borrow_mut();

                        let x_left_height = if let Some(x_node_left) = &x_node.borrow().left {
                            x_node_left.borrow().height
                        } else {
                            0
                        };

                        let x_right_height = if let Some(x_node_right) = &x_node.borrow().left {
                            x_node_right.borrow().height
                        } else {
                            0
                        };

                        let mut x_node_borrow_mut = x_node.borrow_mut();

                        let x_node_height = 1 + std::cmp::max(
                            x_left_height,
                            x_right_height,
                        );

                        x_node_borrow_mut.height = x_node_height;
                    }
                    
                    let y_height = 1 + std::cmp::max(
                        y_node.left.as_ref().map_or(0, |n| n.borrow().height),
                        y_node.right.as_ref().map_or(0, |n| n.borrow().height),
                    );
    
                    //x_node.borrow_mut().height = x_node_height; // was here before outside block scope
                    y_node.height = y_height;
                    */
                    //Some(y_left) // change maybe?
                    x_node.borrow_mut().right.take()
                } else {
                    y_node.left = Some(x_node.clone()); //y_node.left -> right left
                    x_node.borrow_mut().right = Some(Rc::clone(&y_ref_cell));

                    // Update heights
                    {
                        //let mut x_node_borrow_mut = x_node.borrow_mut();

                        let x_left_height = if let Some(x_node_left) = &x_node.borrow().left {
                            x_node_left.borrow().height
                        } else {
                            0
                        };

                        let x_right_height = if let Some(x_node_right) = &x_node.borrow().left {
                            x_node_right.borrow().height
                        } else {
                            0
                        };

                        let mut x_node_borrow_mut = x_node.borrow_mut();

                        
                        let x_node_height = 1 + std::cmp::max(
                            x_left_height,
                            x_right_height,
                        );


                        x_node_borrow_mut.height = x_node_height;
                    }
                    
                    let y_height = 1 + std::cmp::max(
                        y_node.left.as_ref().map_or(0, |n| n.borrow().height),
                        y_node.right.as_ref().map_or(0, |n| n.borrow().height),
                    );
    
                    //x_node.borrow_mut().height = x_node_height; // was here before outside block scope
                    y_node.height = y_height;
                    
                    x_node.borrow_mut().right.take()
                }
                    
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
                // Restore x if its right child is None
                Some(Rc::clone(x_node))
            } 
        } else {
            println!("7");
            None
        }
    }

    fn rotate_right(&mut self, mut y: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        if let Some(y_node) = y.take() {
            let x = y_node.borrow_mut().left.take();
            if let Some(x_ref_cell) = x.clone() {
                let mut x_node = x_ref_cell.borrow_mut();
                if let Some(x_right) = x_node.right.take() {
                    println!("inner");
                    x_node.right = Some(Rc::clone(&y_node));
                    y_node.borrow_mut().left = Some(Rc::clone(&x_ref_cell));

                    // Update heights

                    {
                        let y_left_height = if let Some(y_node_left) = &y_node.borrow().left {
                            y_node_left.borrow().height
                        } else {
                            0
                        };

                        let y_right_height = if let Some(y_node_right) = &y_node.borrow().left {
                            y_node_right.borrow().height
                        } else {
                            0
                        };

                        let mut y_node_borrow_mut = y_node.borrow_mut();

                        let y_node_height = 1 + std::cmp::max(
                            y_left_height,
                            y_right_height,
                        );
                        y_node_borrow_mut.height = y_node_height;
                    }
                    
                    let x_height = 1 + std::cmp::max(
                        x_node.left.as_ref().map_or(0, |n| n.borrow().height),
                        x_node.right.as_ref().map_or(0, |n| n.borrow().height),
                    );

                    x_node.height = x_height;
                    
                    y_node.borrow_mut().left.take()
                } else {
                    println!("outer");
                    //y_node.borrow_mut().left = Some(Rc::clone(&x_ref_cell));
                    
                    x_node.right = Some(Rc::clone(&y_node));
                    y_node.borrow_mut().left = Some(Rc::clone(&x_ref_cell));

                    // Update heights
                    /* 
                    {
                        let y_left_height = if let Some(y_node_left) = y_node.borrow().left.clone() {
                            y_node_left.borrow().height
                        } else {
                            0
                        };

                        let y_right_height = if let Some(y_node_right) = y_node.borrow().left.clone() {
                            y_node_right.borrow().height
                        } else {
                            0
                        };

                        let mut y_node_borrow_mut = y_node.borrow_mut();

                        let y_node_height = 1 + std::cmp::max(
                            y_left_height,
                            y_right_height,
                        );
                        y_node_borrow_mut.height = y_node_height;
                    }
                    
                    let x_height = 1 + std::cmp::max(
                        x_node.left.as_ref().map_or(0, |n| n.borrow().height),
                        x_node.right.as_ref().map_or(0, |n| n.borrow().height),
                    );

                    x_node.height = x_height;
                    */
                    y_node.borrow_mut().left.take()
                    //Some(y_node)
                }

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
                // Restore y if its left child is None
                Some(Rc::clone(y_node))
            }
        } else {
            None
        }
    }

    pub fn insert_pub(&mut self, val: i64) {
        self.insert(self.root.clone(), val);
        self.balance_tree();
    }

    pub fn insert(&mut self, root: Option<Rc<RefCell<Node>>>, val: i64) -> Option<Rc<RefCell<Node>>> {
        let new_node = Rc::new(RefCell::new(Node::new(val, 1))); // New node with height 1
        
        match root {
            None => {
                self.root = Some(new_node);
            }
            Some(node) => {
                let node_val = node.borrow().val;
                if val < node_val {
                    let mut current_node = node.borrow_mut();
                    if current_node.left.is_none() {
                        current_node.left = Some(new_node.clone());
                    } else {
                        self.insert(current_node.left.clone(), val);
                    }
                } else if val > node.borrow().val {
                    let mut current_node = node.borrow_mut();
                    if current_node.right.is_none() {
                        current_node.right = Some(new_node.clone());
                    } else {
                        self.insert(current_node.right.clone(), val);
                    }
                }

                {
                    let mut node_borrow = node.borrow_mut();
                    // Set node's height
                    node_borrow.height = 1 + std::cmp::max(
                        node_borrow.left.as_ref().map_or(0, |n| n.borrow().height), // Node's height or 0 if node is None
                        node_borrow.right.as_ref().map_or(0, |n| n.borrow().height),
                    );
                    //println!("{}: {}", node_borrow.val, node_borrow.height);
                }
                
                //println!("NODE: {}; HEIGHT: {}", node_val, node.borrow().height);
                Some(Rc::clone(&node))
            }
        }
    }

    fn balance_subtree(&mut self, node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        
        if let Some(ref current) = node {
            let balance_factor = current.borrow().get_balance_factor();
            // Left heavy
            if balance_factor > 1 {
                let left_balance_factor = current.borrow().left.as_ref().map_or(0, |n| n.borrow().get_balance_factor());
                
                // Left-Right case
                //println!("LEFT HEAVY: {}; BALANCE: {}", current.borrow().val, balance_factor);
                if left_balance_factor < 0 {
                    println!("LEFT RIGHT");
                    //let left_child = current.borrow_mut().left.take();
                    let new_left = self.rotate_left(current.borrow().left.clone());
                    current.borrow_mut().left = new_left.clone();
                }

                // Left-Left case
                //println!("LEFT LEFT");
                return self.rotate_right(Some(Rc::clone(current)));
            }

            // Right heavy
            if balance_factor < -1 {
                let right_balance_factor = current.borrow().right.as_ref().map_or(0, |n| n.borrow().get_balance_factor());
                
                // Right-Left case
                //println!("RIGHT HEAVY: {}", current.borrow().val);
                if right_balance_factor > 0 {
                    println!("RIGHT LEFT");
                    let right_child = current.borrow().right.clone();
                    let new_right = self.rotate_right(right_child);
                    current.borrow_mut().right = new_right.clone();
                }

                // Right-Right case
                //println!("RIGHT RIGHT");
                return self.rotate_left(Some(Rc::clone(current)));
            }

            // Recursively balance children
            let left = self.balance_subtree(current.borrow().left.clone());
            current.borrow_mut().left = left.clone();

            let right = self.balance_subtree(current.borrow().right.clone());
            current.borrow_mut().right = right.clone();

            Some(Rc::clone(current))
        } else {
            None
        }
    }

    // Public function to balance the entire AVL tree
    fn balance_tree(&mut self) {
        let root = self.root.clone();
        self.root = self.balance_subtree(root);
        self.update_heights();
    }

    pub fn update_heights(&mut self) {
        self.update_heights_recursive(&self.root);
    }

    // Recursive helper function for updating heights
    fn update_heights_recursive(&self, node: &Option<Rc<RefCell<Node>>>) {
        if let Some(current) = node {
            // Recursively update heights of left and right subtrees
            self.update_heights_recursive(&current.borrow().left);
            self.update_heights_recursive(&current.borrow().right);

            // Calculate the new height of the current node
            let left_height = current.borrow().left.as_ref().map_or(0, |n| n.borrow().height);
            let right_height = current.borrow().right.as_ref().map_or(0, |n| n.borrow().height);
            let new_height = 1 + std::cmp::max(left_height, right_height);

            // Update the height of the current node
            current.borrow_mut().height = new_height;
        }
    }
    
    pub fn delete(&self, val: i64) {

    }

    pub fn delete(&mut self, val: i64) {
        let root = self.root.take();
        self.root = self.delete_node(root, val);
    }

    fn delete_node(&mut self, root: Option<Rc<RefCell<Node>>>, val: i64) -> Option<Rc<RefCell<Node>>> {
        if let Some(ref node) = root {
            let mut current_node = node.borrow_mut();
            if val < current_node.val {
                current_node.left = self.delete_node(current_node.left.take(), val);
            } else if val > current_node.val {
                current_node.right = self.delete_node(current_node.right.take(), val);
            } else {
                // Node to delete found
                if current_node.left.is_none() {
                    // Case 1: No left child or no child at all
                    return current_node.right.take();
                } else if current_node.right.is_none() {
                    // Case 2: No right child
                    return current_node.left.take();
                } else {
                    // Case 3: Node has both children
                    let successor = self.find_min(&current_node.right);
                    let successor_val = successor.borrow().val;
                    current_node.val = successor_val;
                    current_node.right = self.delete_node(current_node.right.take(), successor_val);
                }
            }

            // Set node's height
            current_node.height = 1 + std::cmp::max(
                current_node.left.as_ref().map_or(0, |n| n.borrow().height), // Node's height or 0 if node is None
                current_node.right.as_ref().map_or(0, |n| n.borrow().height),
            );

            let balance = current_node.get_balance_factor();

            if balance > 1 {
                if let Some(left_node) = &mut current_node.left {
                    if val < left_node.borrow().val {
                        // Left Left Case
                        return self.rotate_right(Some(node.clone())); // Rotate and return
                    } else {
                        // Left Right Case
                        if let Some(rotated_node) = self.rotate_left(Some(left_node.clone())) {
                            *left_node = rotated_node;
                        }
                        return self.rotate_right(Some(node.clone())); // Rotate and return
                    }
                }
            } else if balance < -1 {
                if let Some(right_node) = &mut current_node.right {
                    if val > right_node.borrow().val {
                        // Right Right Case
                        return self.rotate_left(Some(node.clone())); // Rotate and return
                    } else {
                        // Right Left Case
                        if let Some(rotated_node) = self.rotate_right(Some(right_node.clone())) {
                            *right_node = rotated_node;
                        }
                        return self.rotate_left(Some(node.clone())); // Rotate and return
                    }
                }
            }
        }
        root
    }

    fn find_min(&self, node: &Option<Rc<RefCell<Node>>>) -> Rc<RefCell<Node>> {
        let mut current_node = node.clone();
        while let Some(ref next_node) = current_node.clone().unwrap().borrow().left {
            current_node = Some(Rc::clone(&next_node));
        }
        current_node.unwrap()
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
        self.print_traversal_helper(&self.root);
    }

    fn print_traversal_helper(&self, node: &Option<Rc<RefCell<Node>>>) {
        if let Some(ref current) = node {
            self.print_traversal_helper(&current.borrow().left);
            //println!("{}", current.borrow().val);
            println!("Value: {}, Height: {}", current.borrow().val, current.borrow().height);
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

    pub fn search_tree(&self, val: i64) -> Option<Rc<RefCell<Node>>> {
        fn df(n: Option<Rc<RefCell<Node>>>, val: i64) -> Option<Rc<RefCell<Node>>> {
            if let Some(ref node) = n {
                let mut current_node = node.borrow_mut();
                if val < current_node.val  && !current_node.left.is_none() {
                    df(current_node.left.take(), val);
                } else if val > current_node.val && !current_node.right.is_none() {
                    df(current_node.right.take(), val);
                } else {
                    return Some(Rc::clone(node));
                }
            }
            None
        }

        let root = self.root.clone();
        df(root, val)
    }

}