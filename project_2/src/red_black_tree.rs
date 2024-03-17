use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

type Link<T> = Option<Rc<RefCell<TreeNode<T>>>>;

struct TreeNode<T> {
    color: NodeColor,
    key: T,
    parent: Link<T>,
    left: Link<T>,
    right: Link<T>,
}

pub struct RedBlackTree<T> {
    root: Link<T>,
}

impl <T: Ord + Clone + std::fmt::Debug> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree { root: None }
    }

    pub fn insert(&mut self, key: T) {
        let new_node = Rc::new(RefCell::new(TreeNode {
            color: NodeColor::Red,
            key: key.clone(),
            parent: None,
            left: None,
            right: None,
        }));
    
        let mut y = None;
        let mut x = self.root.clone();
    
        while let Some(current) = x {
            y = Some(current.clone());
            if key < current.borrow().key {
                x = current.borrow().left.clone();
            } else {
                x = current.borrow().right.clone();
            }
        }
        
        new_node.borrow_mut().parent = y.clone();

        if let Some(parent) = y {
            if key < parent.borrow().key {
                parent.borrow_mut().left = Some(new_node.clone());
            } else {
                parent.borrow_mut().right = Some(new_node.clone());
            }
        } else {
            self.root = Some(new_node.clone());
        }

        self.insert_fixup(Some(new_node));

    }
    
    fn insert_fixup(&mut self, mut node: Link<T>) {
        while let Some(ref current) = node {
            let parent = Self::get_parent(&node);
            if parent.is_none() || Self::get_color(&parent) == NodeColor::Black {
                break;
            }
            
            let uncle = Self::get_uncle(&node);
            let grandparent = Self::get_grandparent(&node);
            
            if Self::get_color(&uncle) == NodeColor::Red {
                if let Some(ref p) = parent {
                    p.borrow_mut().color = NodeColor::Black;
                }
                if let Some(ref u) = uncle {
                    u.borrow_mut().color = NodeColor::Black;
                }
                if let Some(ref g) = grandparent {
                    g.borrow_mut().color = NodeColor::Red;
                    node = Some(g.clone());
                    continue;
                }
            } else {
                let is_left_parent = if let (Some(ref g), Some(ref p)) = (&grandparent, &parent) {
                    g.borrow().left.as_ref().map_or(false, |left| Rc::ptr_eq(left, p))
                } else {
                    false
                };
                
                let is_left_child = if let Some(ref p) = parent {
                    p.borrow().left.as_ref().map_or(false, |left| Rc::ptr_eq(left, current))
                } else {
                    false
                };
                
                if is_left_parent != is_left_child {
                    if is_left_child {
                        self.rotate_right(parent.clone());
                    } else {
                        self.rotate_left(parent.clone());
                    }
                    node = parent;
                }
                
                let parent = Self::get_parent(&node);
                let grandparent = Self::get_grandparent(&node);
                
                if let Some(ref p) = parent {
                    p.borrow_mut().color = NodeColor::Black;
                }
                if let Some(ref g) = grandparent {
                    g.borrow_mut().color = NodeColor::Red;
                    if is_left_child {
                        self.rotate_right(Some(g.clone()));
                    } else {
                        self.rotate_left(Some(g.clone()));
                    }
                }
                break;
            }
        }
        if let Some(ref root) = self.root {
            root.borrow_mut().color = NodeColor::Black;
        }
    }
    
    fn get_color(node: &Link<T>) -> NodeColor {
        match node {
            Some(ref n) => n.borrow().color.clone(),
            None => NodeColor::Black,
        }
    }
    
    fn get_parent(node: &Link<T>) -> Link<T> {
        match node {
            Some(ref n) => n.borrow().parent.clone(),
            None => None,
        }
    }
    
    fn get_grandparent(node: &Link<T>) -> Link<T> {
        match Self::get_parent(node) {
            Some(ref p) => p.borrow().parent.clone(),
            None => None,
        }
    }
    
    fn get_sibling(node: &Link<T>) -> Link<T> {
        match node {
            Some(ref n) => {
                let parent = n.borrow().parent.clone();
                match parent {
                    Some(ref p) => {
                        if let Some(ref left_child) = p.borrow().left {
                            if Rc::ptr_eq(left_child, n) {
                                return p.borrow().right.clone();
                            }
                        }
                        if let Some(ref right_child) = p.borrow().right {
                            if Rc::ptr_eq(right_child, n) {
                                return p.borrow().left.clone();
                            }
                        }
                        None
                    },
                    None => None,
                }
            },
            None => None,
        }
    }
    
    fn get_uncle(node: &Link<T>) -> Link<T> {
        let parent = Self::get_parent(node);
        let grandparent = Self::get_grandparent(node);
        
        if grandparent.is_none() {
            return None;
        }
        
        Self::get_sibling(&parent)
    }
    
    fn rotate_right(&mut self, node: Link<T>) {
        if let Some(ref n) = node {
            let left_child = n.borrow().left.clone();
            if let Some(ref l) = left_child {
                n.borrow_mut().left = l.borrow().right.clone();
                if let Some(ref right_child) = l.borrow().right {
                    right_child.borrow_mut().parent = Some(Rc::clone(n));
                }
                l.borrow_mut().parent = n.borrow().parent.clone();
                if let Some(ref parent) = n.borrow().parent {
                    if Rc::ptr_eq(parent.borrow().left.as_ref().unwrap(), n) {
                        parent.borrow_mut().left = Some(Rc::clone(l));
                    } else {
                        parent.borrow_mut().right = Some(Rc::clone(l));
                    }
                } else {
                    self.root = Some(Rc::clone(l));
                }
                l.borrow_mut().right = Some(Rc::clone(n));
                n.borrow_mut().parent = Some(Rc::clone(l));
            }
        }
    }
    
    fn rotate_left(&mut self, node: Link<T>) {
        if let Some(ref n) = node {
            let right_child = n.borrow().right.clone();
            if let Some(ref r) = right_child {
                n.borrow_mut().right = r.borrow().left.clone();
                if let Some(ref left_child) = r.borrow().left {
                    left_child.borrow_mut().parent = Some(Rc::clone(n));
                }
                r.borrow_mut().parent = n.borrow().parent.clone();
                if let Some(ref parent) = n.borrow().parent {
                    if Rc::ptr_eq(parent.borrow().left.as_ref().unwrap(), n) {
                        parent.borrow_mut().left = Some(Rc::clone(r));
                    } else {
                        parent.borrow_mut().right = Some(Rc::clone(r));
                    }
                } else {
                    self.root = Some(Rc::clone(r));
                }
                r.borrow_mut().left = Some(Rc::clone(n));
                n.borrow_mut().parent = Some(Rc::clone(r));
            }
        }
    }
    
    pub fn count_leaves(&self) -> usize {
        self.count_leaves_recursive(&self.root)
    }

    fn count_leaves_recursive(&self, node: &Link<T>) -> usize {
        if let Some(ref current) = node {
            if current.borrow().left.is_none() && current.borrow().right.is_none() {
                1
            } else {
                let left_count = self.count_leaves_recursive(&current.borrow().left);
                let right_count = self.count_leaves_recursive(&current.borrow().right);
                left_count + right_count
            }
        } else {
            0
        }
    }

    pub fn height(&self) -> isize  {
        self.get_height(&self.root)
    }

    fn get_height(&self, node: &Link<T>) -> isize  {
        if let Some(ref current) = node {
            let left_height = self.get_height(&current.borrow().left);
            let right_height = self.get_height(&current.borrow().right);
            1 + std::cmp::max(left_height, right_height)
        } else {
            0
        }
    }
    
    pub fn print_in_order(&self) {
        let tree_string = self.build_tree_string(&self.root, 0);
        println!("{}", tree_string);
    }

    fn build_tree_string(&self, node: &Link<T>, depth: usize) -> String {
        if let Some(ref current) = node {
            let indent = "    ".repeat(depth);
            let node_key = format!("{:?}", current.borrow().key);
            let node_color = format!("{:?}", current.borrow().color);
            let left_string = self.build_tree_string(&current.borrow().left, depth + 1);
            let right_string = self.build_tree_string(&current.borrow().right, depth + 1);

            let mut result = format!("{}TreeNode {{\n", indent);
            result += &format!("{}    data: \"{}\",\n", indent, node_key);
            result += &format!("{}    color: \"{}\",\n", indent, node_color);

            if left_string.is_empty() {
                result += &format!("{}    left_child: None,\n", indent);
            } else {
                result += &format!("{}    left_child: Some(\n{}{}\n{}    ),\n", indent, left_string, indent, indent);
            }

            if right_string.is_empty() {
                result += &format!("{}    right_child: None,\n", indent);
            } else {
                result += &format!("{}    right_child: Some(\n{}{}\n{}    ),\n", indent, right_string, indent, indent);
            }

            result += &format!("{}}}", indent);
            result
        } else {
            String::new()
        }
    }
}
