use crate::node::{Node, Value};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone)]
pub struct Arena {
    arena: Vec<Node>,
    index: Option<usize>,
}

impl Arena {
    pub fn new() -> Self {
        Arena {
            arena: vec![],
            index: None,
        }
    }
    pub fn new_with_capacity(capacity: usize) -> Self {
        Arena {
            arena: Vec::with_capacity(capacity),
            index: None,
        }
    }
    pub fn advance(&mut self) {
        self.set_index(
            self.index.map(|index| index + 1).unwrap_or(0)
        );
    }
    
    pub fn move_to_parent(&mut self) {
        if let Some(parent) = self.parent() {
            self.set_index(parent);
        }
    }
    fn set_index(&mut self, index: usize) {
        self.index.replace(index);
    }
    pub fn index(&self) -> Option<usize> {
        self.index
    }
    pub fn len(&self) -> usize {
        self.arena.len()
    }

    pub fn is_empty(&self) -> bool {
        self.arena.is_empty()
    }

    pub fn parent(&self) -> Option<usize> {
        if let Some(index) = self.index {
            self.parent_from_index(index)
        } else {
            None
        }
    }

    pub fn parent_from_index(&self, index: usize) -> Option<usize> {
        self.arena[index].parent()
    }

    /// useful when creating a new tree and you want new index references to parents and children
    pub fn child_values_from_index(&self, index: usize) -> Option<Vec<Value>> {
        self.arena[index].children().and_then(|children| {
            let mut values = Vec::with_capacity(children.len());
            for child in children {
                if let Some(node) = self.node_from_index(*child) {
                    values.push(node.value())
                }
            }
            Some(values)
        })
    }

    pub fn node(&self) -> Option<Node> {
        if let Some(index) = self.index {
            self.node_from_index(index)
        } else {
            None
        }
    }

    pub fn node_from_index(&self, index: usize) -> Option<Node> {
        if index < self.arena.len() {
            Some(self.arena[index].clone())
        } else {
            None
        }
    }

    pub fn node_by_value(&self, value: impl Into<Value>) {}

    pub fn node_by_value_from_index(&self, value: impl Into<Value>, start: usize) -> Option<&Node> {
        let value = value.into();
        self.arena[start..]
            .iter()
            .find(|node| node.value().eq(&value))
    }

    pub fn lineage_for_index(&self, index: usize) -> Vec<usize> {
        let mut parents = vec![];
        let mut tmp_index = index;
        while let Some(parent) = self
            .node_from_index(tmp_index)
            .and_then(|node| node.parent())
        {
            if let Some(node) = self.node_from_index(parent) {
                parents.push(node.index());
                tmp_index = parent;
            }
        }
        parents
    }

    pub fn lineage(&self) -> Vec<usize> {
        if let Some(index) = self.index() {
            self.lineage_for_index(index)
        } else {
            vec![]
        }
    }

    /// Tree born from the current position
    ///
    /// assumes ordering has been unchanged
    pub fn subtree(&self) -> Self {
        if let Some(index) = self.index {
            self.subtree_from_index(index)
        } else {
            Self::new()
        }
    }
    pub fn subtree_from_index(&self, start_index: usize) -> Self {
        let mut new_tree = Self::new();
        let mut index_map: HashMap<usize, usize> = HashMap::new();
        let mut queue = vec![start_index];

        // First pass: collect all nodes in the subtree
        let mut nodes_to_copy = vec![];
        let mut visited = vec![false; self.arena.len()];

        while let Some(current_index) = queue.pop() {
            if visited[current_index] {
                continue;
            }
            visited[current_index] = true;
            nodes_to_copy.push(current_index);
            let parent = self.arena[current_index].children();
            // Add children to queue
            if let Some(children) =  parent {
                for &child in children {
                    queue.push(child);
                }
            }
        }

        // Second pass: copy nodes with remapped indices
        for (new_index, &old_index) in nodes_to_copy.iter().enumerate() {
            index_map.insert(old_index, new_index);

            let old_node = &self.arena[old_index];
            let parent = if old_index == start_index {
                None // Root of subtree has no parent
            } else {
                old_node.parent().and_then(|p| index_map.get(&p).copied())
            };

            let mut new_node = Node::new(old_node.value(), new_index, parent);

            // Remap children indices
            if let Some(old_children) = old_node.children() {
                let new_children: Vec<usize> = old_children
                    .iter()
                    .filter_map(|&child| {
                        // Only include children that are part of the subtree
                        if nodes_to_copy.contains(&child) {
                            Some(nodes_to_copy.iter().position(|&i| i == child).unwrap())
                        } else {
                            None
                        }
                    })
                    .collect();

                if !new_children.is_empty() {
                    new_node.set_children(new_children);
                }
            }

            new_tree.arena.push(new_node);
        }

        // Set the index to the root of the new tree
        new_tree.set_index(0);
        new_tree
    }

    pub fn add_root_node(&mut self, value: impl Into<Value>) {
        let index = self.arena.len();
        self.arena.push(Node::new(value, index, None));
        self.index.replace(index);
    }
    pub fn add_child_node(&mut self, value: impl Into<Value>) {
        let child_node_index = self.arena.len();
        self.arena
            .push(Node::new(value, child_node_index, self.index));
        self.arena[self.index.expect("expect to have an index to add a child")].add_child(child_node_index);
    }
}

impl Display for Arena {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for node in &self.arena {
            let tabs = self.lineage_for_index(node.index()).iter().map(|t| "\t").collect::<String>();
            writeln!(f, "{tabs}{}", node)?
        }
        Ok(())
    }
}