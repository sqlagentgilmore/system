use std::fmt::{Display, Formatter};
use crate::arena::obj::Object;

#[derive(Debug, Default, Clone)]
pub struct ArenaTree
{
    arena: Vec<Object>,
    curr: Option<usize>,
}

impl ArenaTree
{
    pub fn new(capacity: Option<usize>) -> Self {
        if let Some(capacity) = capacity {
            ArenaTree { arena: Vec::with_capacity(capacity), curr: None }
        } else {
            ArenaTree { arena: vec![], curr: None }
        }
        
    }
    
    pub fn current_position(&self) -> Option<usize> {
        self.curr
    }
    pub fn get_object(&self) -> Option<Object> {
        if self.arena.is_empty() {
            None
        } else {
            Some(self.arena[self.unsafe_position()].clone())
        }
    }
    pub fn add_node<'a>(&mut self, val: &str) {
        // only update on first push
        if self.curr.is_none() {
            self.curr = Some(0);
        }
        self.arena.push(Object::new(val, if self.arena.is_empty() {0} else {self.arena.len()}, None));
    }
    fn unsafe_position(&self) -> usize {
        unsafe { self.curr.unwrap_unchecked() }
    }
    pub fn add_child_node(&mut self, val: &str, parent: usize) {
        let new_node_position = self.arena.len();
         self.add_node(val);
            unsafe {self.arena.last_mut().unwrap_unchecked()}.update_parent(parent);
        self.arena[parent].update_children(new_node_position);
    }
    
    pub fn get_object_from_position(&self, position: usize) -> Object {
        self.arena[position].clone()
    }
    
    pub fn get_current_objects_children(&self) -> Option<Vec<Object>> {
        if let Some(current_position) = self.curr {
            if let Some(children) = self.arena[current_position].children() {
                let mut children_objects = Vec::with_capacity(children.len());
                for child in children {
                    children_objects.push(self.arena[*child].clone());
                }
                Some(children_objects)
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn sub_tree(&self, from_position: usize) -> Self {
        let mut arena = Vec::with_capacity(self.arena.len());
        let current_object = self.arena[from_position].clone();
        arena.push(current_object.clone());
        if let Some(children) = current_object.children() {
            for child in children {
                arena.extend(self.sub_tree(*child).arena);
            }
        }
        Self {
            arena,
            curr: Some(0)
        }
        
    }
    pub fn next_position(&mut self) {
        self.curr = self.curr.map(|curr| curr + 1)
    }
}

impl Display for ArenaTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        
        for pos in 0..self.arena.len() {
            
            let current_object = &self.arena[pos];
            
            if current_object.has_parent() {
                writeln!(f, "{}", current_object.describe())?;
            }
            
            if let Some(children) = self.get_current_objects_children() {
                for child in children.iter() {
                    writeln!(f, "{}", self.sub_tree(child.position()))?;
                }
            }
        }
        
        Ok(())
    }
}