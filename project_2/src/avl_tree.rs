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

     // Right rotation
     fn rotate_right(&mut self, mut y: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        if let Some(ref mut y_node) = y {
            let mut x = y_node.borrow_mut().left.take(); // Set x to be y's left child
            if let Some(ref mut x_node) = x {
                y_node.borrow_mut().left = x_node.borrow_mut().right.take(); // Turn x's right subtree into y's left subtree
                x_node.borrow_mut().right = y.take(); // Make y the right child of x
                // Update heights
                self.update_heights_recursive(&x);
                self.update_heights_recursive(&y);
                return x; // Return the new root (x)
            }
        }
        None
    }

    // Left rotation
    fn rotate_left(&mut self, mut x: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        if let Some(ref mut x_node) = x {
            let mut y = x_node.borrow_mut().right.take(); // Set y to be x's right child
            if let Some(ref mut y_node) = y {
                x_node.borrow_mut().right = y_node.borrow_mut().left.take(); // Turn y's left subtree into x's right subtree
                y_node.borrow_mut().left = x.take(); // Make x the left child of y
                // Update heights
                self.update_heights_recursive(&x);
                self.update_heights_recursive(&y);
                return y; // Return the new root (y)
            }
        }
        None
    }

    pub fn insert_pub(&mut self, val: i64) {
        self.insert(self.root.clone(), val);
        self.balance_tree();
    }

    pub fn insert(&mut self, root: Option<Rc<RefCell<Node>>>, val: i64) -> Option<Rc<RefCell<Node>>> {
        let new_node = Rc::new(RefCell::new(Node::new(val, 1))); // New node with height 1
        match root {
            None => {
                self.root = Some(new_node.clone());
                Some(new_node)
            }
            Some(node) => {
                let node_val = node.borrow().val;
                if val < node_val {
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
                if left_balance_factor < 0 {
                    let new_left = self.rotate_left(current.borrow().left.clone());
                    current.borrow_mut().left = new_left.clone();
                }

                // Left-Left case
                return self.rotate_right(Some(Rc::clone(current)));
            }

            // Right heavy
            if balance_factor < -1 {
                let right_balance_factor = current.borrow().right.as_ref().map_or(0, |n| n.borrow().get_balance_factor());
                
                // Right-Left case
                if right_balance_factor > 0 {
                    let right_child = current.borrow().right.clone();
                    let new_right = self.rotate_right(right_child);
                    current.borrow_mut().right = new_right.clone();
                }

                // Right-Right case
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

    pub fn delete(&mut self, val: i64) {
        let root = self.root.take();
        self.root = self.delete_node(root, val);
        self.balance_tree();
    }

    fn delete_node(&mut self, root: Option<Rc<RefCell<Node>>>, val: i64) -> Option<Rc<RefCell<Node>>> {
        if let Some(ref node) = root {
            {
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

            };
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
        fn df(node: Option<Rc<RefCell<Node>>>) -> i64 {
            if let Some(ref current_node) = node {
                if current_node.borrow().left.is_none() && current_node.borrow().right.is_none() {
                    return 1; 
                }
                return df(current_node.borrow().right.clone()) + df(current_node.borrow().left.clone())
            } else {
                return 0;
            }
        }

        df(self.root.clone())
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