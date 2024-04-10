use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

impl Display for NodeColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeColor::Red => write!(f, "Red"),
            NodeColor::Black => write!(f, "black"),
        }
    }
}

type Tree = Rc<RefCell<TreeNode<u32>>>;

type RedBlackTree = Option<Tree>;

pub struct TreeNode<T> {
    color: NodeColor,
    key: T,
    parent: RedBlackTree,
    left: RedBlackTree,
    right: RedBlackTree,
}

impl TreeNode<u32> {
    fn new(key: u32) -> Tree {
        Rc::new(RefCell::new(TreeNode {
            color: NodeColor::Red,
            key,
            parent: None,
            left: None,
            right: None,
        }))
    }
}

impl PartialEq for TreeNode<u32> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.color == other.color
    }
}

pub struct RBTree {
    root: RedBlackTree,
}

impl RBTree {
    pub fn new() -> Self {
        RBTree { root: None }
    }

    pub fn insert(&mut self, key: u32) {
        let new_node = TreeNode::new(key);
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

    pub fn delete(&mut self, key: u32) {
        if let Some(node_to_delete) = self.search_node(key) {
            let original_color = node_to_delete.borrow().color.clone();
            let x;
            let y;
            if node_to_delete.borrow().left.is_none() {
                x = node_to_delete.borrow().right.clone();
                self.transplant(Some(node_to_delete.clone()), node_to_delete.borrow().right.clone());
            } else if node_to_delete.borrow().right.is_none() {
                x = node_to_delete.borrow().left.clone();
                self.transplant(Some(node_to_delete.clone()), node_to_delete.borrow().left.clone());
            } else {
                y = self.minimum(node_to_delete.borrow().right.clone()).unwrap();
                let y_original_color = y.borrow().color.clone();
                x = y.borrow().right.clone();
                if y.borrow().parent.as_ref().unwrap().borrow().key != node_to_delete.borrow().key {
                    self.transplant(Some(y.clone()), y.borrow().right.clone());
                    y.borrow_mut().right = node_to_delete.borrow().right.clone();
                    y.borrow().right.as_ref().unwrap().borrow_mut().parent = Some(y.clone());
                }
                self.transplant(Some(node_to_delete.clone()), Some(y.clone()));
                y.borrow_mut().left = node_to_delete.borrow().left.clone();
                y.borrow().left.as_ref().unwrap().borrow_mut().parent = Some(y.clone());
                y.borrow_mut().color = node_to_delete.borrow().color.clone();
                if y_original_color == NodeColor::Black {
                    self.delete_fixup(&x);
                }
            }
            if original_color == NodeColor::Black {
                self.delete_fixup(&x);
            }
        }
    }

    pub fn count_leaves(&self) -> usize {
        self.count_leaves_recursive(&self.root)
    }

    pub fn height(&self) -> isize {
        self.get_height(&self.root)
    }

    pub fn print_in_order(&self) {
        let tree_string = self.build_tree_string(&self.root, 0);
        println!("{}", tree_string);
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }    

    pub fn print_tree(&self) {
        Self::print_from_node(&self.root, 0);
    }

    fn insert_fixup(&mut self, mut node: RedBlackTree) {
        while let Some(ref current) = node {
            let parent = Self::get_parent(&Some(current.clone()));
            if parent.is_none() || Self::get_color(&parent) == NodeColor::Black {
                break;
            }
            let uncle = Self::get_uncle(&Some(current.clone()));
            let grandparent = Self::get_grandparent(&Some(current.clone()));
            if Self::get_color(&uncle) == NodeColor::Red {
                Self::set_color(&parent, NodeColor::Black);
                Self::set_color(&uncle, NodeColor::Black);
                if let Some(ref g) = grandparent {
                    Self::set_color(&Some(g.clone()), NodeColor::Red);
                    node = Some(g.clone());
                }
                continue;
            }
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
                    if let Some(ref p) = parent {
                        self.rotate_right(Some(p.clone()));
                    }
                } else {
                    if let Some(ref p) = parent {
                        self.rotate_left(Some(p.clone()));
                    }
                }
                node = parent;
                continue;
            }
            Self::set_color(&parent, NodeColor::Black);
            Self::set_color(&grandparent, NodeColor::Red);
            if is_left_child {
                if let Some(ref g) = grandparent {
                    self.rotate_right(Some(g.clone()));
                }
            } else {
                if let Some(ref g) = grandparent {
                    self.rotate_left(Some(g.clone()));
                }
            }
            break;
        }
        if let Some(ref root) = self.root {
            Self::set_color(&Some(root.clone()), NodeColor::Black);
        }
    }

    fn get_color(node: &RedBlackTree) -> NodeColor {
        match node {
            Some(ref n) => n.borrow().color.clone(),
            None => NodeColor::Black,
        }
    }

    fn set_color(node: &RedBlackTree, color: NodeColor) {
        if let Some(ref n) = node {
            n.borrow_mut().color = color;
        }
    }

    fn get_parent(node: &RedBlackTree) -> RedBlackTree {
        match node {
            Some(ref n) => n.borrow().parent.clone(),
            None => None,
        }
    }

    fn get_grandparent(node: &RedBlackTree) -> RedBlackTree {
        match Self::get_parent(node) {
            Some(ref p) => p.borrow().parent.clone(),
            None => None,
        }
    }

    fn get_sibling(node: &RedBlackTree) -> RedBlackTree {
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

    fn get_uncle(node: &RedBlackTree) -> RedBlackTree {
        let parent = Self::get_parent(node);
        let grandparent = Self::get_grandparent(node);
        if grandparent.is_none() {
            return None;
        }
        Self::get_sibling(&parent)
    }

    fn is_left_child(parent: &Tree, child: &Tree) -> bool {
        if let Some(ref left_child) = parent.borrow().left {
            Rc::ptr_eq(left_child, child)
        } else {
            false
        }
    }

    fn rotate_right(&mut self, node: RedBlackTree) {
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

    fn rotate_left(&mut self, node: RedBlackTree) {
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
    
    pub fn search_node(&self, key: u32) -> Option<Tree> {
        let mut current = self.root.clone();
        while let Some(current_node) = current {
            let current_key = current_node.borrow().key;
            if key < current_key {
                current = current_node.borrow().left.clone();
            } else if key > current_key {
                current = current_node.borrow().right.clone();
            } else {
                return Some(current_node.clone());
            }
        }
        None
    }

    fn transplant(&mut self, u: RedBlackTree, v: RedBlackTree) {
        match u.as_ref().and_then(|u_node| u_node.borrow().parent.clone()) {
            Some(u_parent_node) => {
                if Rc::ptr_eq(&u_parent_node.borrow().left.as_ref().unwrap(), &u.as_ref().unwrap()) {
                    u_parent_node.borrow_mut().left = v.clone();
                } else {
                    u_parent_node.borrow_mut().right = v.clone();
                }
            },
            None => {
                self.root = v.clone();
            },
        }
        if let Some(v_node) = v {
            v_node.borrow_mut().parent = u.as_ref().and_then(|u_node| u_node.borrow().parent.clone());
        }
    }

    fn minimum(&self, mut node: RedBlackTree) -> RedBlackTree {
        while let Some(current_node) = node {
            if current_node.borrow().left.is_none() {
                return Some(current_node.clone());
            }
            node = current_node.borrow().left.clone();
        }
        None
    }

    fn delete_fixup(&mut self, x: &RedBlackTree) {
        let mut x = x.clone();
        while x != self.root && Self::get_color(&x) == NodeColor::Black {
            if let Some(x_node) = x.clone() {
                if x == x_node.borrow().parent.as_ref().unwrap().borrow().left {
                    let mut w = Self::get_sibling(&x);
                    if Self::get_color(&w) == NodeColor::Red {
                        if let Some(w_node) = w.clone() {
                            w_node.borrow_mut().color = NodeColor::Black;
                        }
                        x_node.borrow().parent.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                        self.rotate_left(x_node.borrow().parent.clone());
                        w = Self::get_sibling(&x);
                    }
                    if Self::get_color(&w.as_ref().unwrap().borrow().left) == NodeColor::Black &&
                       Self::get_color(&w.as_ref().unwrap().borrow().right) == NodeColor::Black {
                        if let Some(w_node) = w.clone() {
                            w_node.borrow_mut().color = NodeColor::Red;
                        }
                        x = x_node.borrow().parent.clone();
                    } else {
                        if Self::get_color(&w.as_ref().unwrap().borrow().right) == NodeColor::Black {
                            if let Some(w_left) = w.as_ref().unwrap().borrow().left.clone() {
                                w_left.borrow_mut().color = NodeColor::Black;
                            }
                            if let Some(w_node) = w.clone() {
                                w_node.borrow_mut().color = NodeColor::Red;
                            }
                            self.rotate_right(w.clone());
                            w = Self::get_sibling(&x);
                        }
                        if let Some(w_node) = w.clone() {
                            w_node.borrow_mut().color = Self::get_color(&x_node.borrow().parent);
                            x_node.borrow().parent.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                        }
                        if let Some(w_right) = w.as_ref().unwrap().borrow().right.clone() {
                            w_right.borrow_mut().color = NodeColor::Black;
                        }
                        self.rotate_left(x_node.borrow().parent.clone());
                        x = self.root.clone();
                    }
                } else {
                    let mut w = Self::get_sibling(&x);
                    if Self::get_color(&w) == NodeColor::Red {
                        if let Some(w_node) = w.clone() {
                            w_node.borrow_mut().color = NodeColor::Black;
                        }
                        x_node.borrow().parent.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                        self.rotate_right(x_node.borrow().parent.clone());
                        w = Self::get_sibling(&x);
                    }
                    if Self::get_color(&w.as_ref().unwrap().borrow().right) == NodeColor::Black &&
                       Self::get_color(&w.as_ref().unwrap().borrow().left) == NodeColor::Black {
                        if let Some(w_node) = w.clone() {
                            w_node.borrow_mut().color = NodeColor::Red;
                        }
                        x = x_node.borrow().parent.clone();
                    } else {
                        if Self::get_color(&w.as_ref().unwrap().borrow().left) == NodeColor::Black {
                            if let Some(w_right) = w.as_ref().unwrap().borrow().right.clone() {
                                w_right.borrow_mut().color = NodeColor::Black;
                            }
                            if let Some(w_node) = w.clone() {
                                w_node.borrow_mut().color = NodeColor::Red;
                            }
                            self.rotate_left(w.clone());
                            w = Self::get_sibling(&x);
                        }
                        if let Some(w_node) = w.clone() {
                            w_node.borrow_mut().color = Self::get_color(&x_node.borrow().parent);
                            x_node.borrow().parent.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                        }
                        if let Some(w_left) = w.as_ref().unwrap().borrow().left.clone() {
                            w_left.borrow_mut().color = NodeColor::Black;
                        }
                        self.rotate_right(x_node.borrow().parent.clone());
                        x = self.root.clone();
                    }
                }
            } else {
                break;
            }
        }
        if let Some(x_node) = x {
            x_node.borrow_mut().color = NodeColor::Black;
        }
    }

    fn count_leaves_recursive(&self, node: &RedBlackTree) -> usize {
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

    fn get_height(&self, node: &RedBlackTree) -> isize  {
        if let Some(ref current) = node {
            let left_height = self.get_height(&current.borrow().left);
            let right_height = self.get_height(&current.borrow().right);
            1 + std::cmp::max(left_height, right_height)
        } else {
            0
        }
    }

    fn build_tree_string(&self, node: &RedBlackTree, depth: usize) -> String {
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

    fn print_from_node(node: &RedBlackTree, depth: usize) {
        if let Some(n) = node {
            let n = n.borrow();
            let indent = "      ".repeat(depth);
            Self::print_from_node(&n.left, depth + 1);
            println!("{}{}({})", indent, n.key, n.color);
            Self::print_from_node(&n.right, depth + 1);
        }
    }
}


