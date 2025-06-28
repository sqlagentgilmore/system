use std::collections::HashSet;
pub(crate) use crate::value::Value;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use compact_str::{CompactString, ToCompactString};
use crate::value::ValueKind;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub index: usize,
    pub value: Value,
    pub parent: Option<usize>,
    pub children: Option<Vec<usize>>,
}

impl Node {
    pub fn new(value: &str, kind: &str, index: usize, parent: usize) -> Self {
        Node {
            index,
            value: Value::from((value, kind)),
            parent: Some(parent),
            children: None,
        }
    }
    
    pub(crate) fn root() -> Self {
        Node {
            index: 0,
            value: ("root", "system").into(),
            parent: None,
            children: None,
        }
    }
    pub fn children(&self) -> Option<&[usize]> {
        self.children.as_deref()
    }
    pub fn value(&self) -> Value {
        self.value.clone()
    }
    pub fn value_as_str(&self) -> &str {
        self.value.get_name_ref()
    }
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn set_index(&mut self, index: usize) {
        self.index = index
    }
    pub fn parent(&self) -> Option<usize> {
        self.parent
    }
    pub fn set_parent(&mut self, index: usize) {
        self.parent.replace(index);
    }
    pub fn set_opt_parent(&mut self, index: Option<usize>) {
        if let Some(idx) = index {
            self.parent.replace(idx);
        } else {
            self.parent = None;
        }
    }
    pub fn pop_parent(&mut self) -> Option<usize> {
        self.parent.take()
    }
    pub fn add_child(&mut self, index: usize) {
        match self.children {
            Some(ref mut children) => {
                if !children.contains(&index) {
                    children.push(index);
                }
            }
            ref mut children => {
                children.replace(vec![index]);
            }
        }
    }
    pub fn set_children(&mut self, children: Vec<usize>) {
        self.children.replace(children);
    }
    pub fn update_childrens_parent(&mut self, index: usize) {
        if let Some(ref mut children) = self.children {
            for child in children.iter_mut() {
                *child = index;
            }
        }
    }
    pub fn remove_children(&mut self) {
        self.children = None;
    }
    pub fn swap(&mut self, other: &mut Node) {
        std::mem::swap(&mut self.value, &mut other.value);
        std::mem::swap(&mut self.parent, &mut other.parent);
        std::mem::swap(&mut self.children, &mut other.children);
    }
    pub fn kind(&self) -> ValueKind {
        self.kind()
    }
    
    pub fn kind_as_str(&self) -> &str {
        self.kind().into()
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        let mut self_children = HashSet::new();
        if let Some(ref children) = self.children {
            for child in children {
                self_children.insert(*child);
            }
        }
        let mut other_children = HashSet::new();
        if let Some(ref children) = other.children {
            for child in children {
                other_children.insert(*child);
            }
        }
        self.index == other.index && self.value == other.value && self_children.eq(&other_children) && self.parent ==other.parent
    }
}