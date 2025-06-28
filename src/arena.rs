use crate::node::Node;
use std::fmt::Display;

#[derive(Debug, Default, Clone)]
pub struct Arena {
    nodes: Vec<Node>,
    root: usize,
}

impl Arena {
    pub fn new() -> Self {
        Arena {
            nodes: vec![Node::root()],
            root: 0,
        }
    }
    pub fn new_with_capacity(capacity: usize) -> Self {
        let mut arena = Arena {
            nodes: Vec::with_capacity(capacity),
            root: 0,
        };
        arena.nodes.push(Node::root());
        arena
    }

    pub fn get_node(&self, index: usize) -> Option<&Node> {
        self.nodes.get(index)
    }

    pub fn get_mut_node(&mut self, index: usize) -> Option<&mut Node> {
        self.nodes.get_mut(index)
    }

    pub fn get_root(&self) -> &Node {
        &self.nodes[self.root]
    }

    pub fn get_parent_node(&self, index: usize) -> Option<&Node> {
        if index == self.root {
            return None; // Root node has no parent
        }
        if let Some(parent) = self.get_node(index).map(|node| node.parent().unwrap()) {
            self.get_node(parent)
        } else {
            None
        }
    }
    pub fn get_mut_parent_node(&mut self, index: usize) -> Option<&mut Node> {
        if index == self.root {
            return None; // Root node has no parent
        }
        if let Some(parent) = self.get_node(index).map(|node| node.parent().unwrap()) {
            self.get_mut_node(parent)
        } else {
            None
        }
    }

    pub fn add_node(&mut self, value: impl AsRef<str>, kind: &str, parent: usize) {
        let index = self.nodes.len();
        if let Some(parent) = self.get_mut_node(parent) {
            let node = Node::new(value.as_ref(), kind, index, parent.index());
            parent.add_child(index);
            self.nodes.push(node);
        }
    }

    pub fn get_parent_nodes(&self, index: usize) -> Vec<&Node> {
        let mut parents = vec![];
        let mut current_index = index;
        while let Some(node) = self.get_parent_node(current_index) {
            parents.push(node);
            if current_index == self.root {
                break; // Stop if we reach the root
            }
            current_index = node.parent().unwrap();
        }
        parents.reverse();
        parents
    }
    pub fn get_owned_parent_nodes(&self, index: usize) -> Vec<Node> {
        let mut parents = vec![];
        let mut current_index = index;
        while let Some(node) = self.get_parent_node(current_index) {
            parents.push(node.clone());
            current_index = node.parent().unwrap();


        }
        parents.reverse();
        parents
    }

    pub fn find_node_by_value(&self, value: &str) -> Option<&Node> {
        self.nodes.iter().find(|node| node.value_as_str().eq(value))
    }

    pub fn get_children(&self, index: usize) -> Vec<Node> {
        if let Some(node) = self.get_node(index) {
            if let Some(children) = node.children() {
                children.iter().map(|child| {
                    self.nodes[*child].clone()
                }).collect()
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }

    pub fn get_children_nodes(&self, index: usize) -> Vec<Node> {
        let mut children = vec![];
        self._get_children_nodes(index, &mut children);
        children
    }
    pub fn _get_children_nodes(&self, mut index: usize, children: &mut Vec<Node>) {
        for child in self.get_children(index) {
            children.push(child.clone());
            if child.children().is_some() {
                self._get_children_nodes(child.index(), children);
            }
        }
    }

    pub fn find_node(&self, value: &str, kind: &str) -> Option<&Node> {
        self.nodes.iter().find(|node| node.value_as_str().eq(value) && node.kind_as_str().eq(kind))
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn root_index(&self) -> usize {
        self.root
    }

    pub fn swap_nodes(&mut self, index1: usize, index2: usize) {
        if index1 >= self.nodes.len() || index2 >= self.nodes.len() {
            panic!("Index out of bounds for swap_nodes");
        }

        let update_parent_for_index = |arena: &mut Arena, index: usize, new_index: usize| {
            if let Some(parent) = arena.get_mut_parent_node(index) {
                if let Some(ref mut children) = parent.children {
                    for child in children.iter_mut() {
                        if *child == index {
                            *child = new_index;
                            break;
                        }
                    }
                }
            }
        };

        let update_children_for_index = |arena: &mut Arena, index: usize, new_index: usize| {
            if let Some(node) = arena.get_mut_node(index) {
                if let Some(ref mut children) = node.children {
                    for child in children.iter_mut() {
                        if *child == index {
                            *child = new_index;
                        }
                    }
                }
            }
        };

        update_parent_for_index(self, index1, index2);
        update_parent_for_index(self, index2, index1);
        update_children_for_index(self, index1, index2);
        update_children_for_index(self, index2, index1);

        let mut original_node1 = self.nodes[index1].clone();
        let mut original_node2 = self.nodes[index2].clone();

        self.nodes[index1].swap(&mut original_node2);
        self.nodes[index2].swap(&mut original_node1);


    }
}