pub(crate) use crate::value::Value;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    index: usize,
    value: Value,
    parent: Option<usize>,
    children: Option<Vec<usize>>,
}

impl Node {
    pub fn new(value: impl Into<Value>, index: usize, parent: Option<usize>) -> Self {
        Node {
            index,
            value: value.into(),
            parent,
            children: None,
        }
    }
    pub fn children(&self) -> Option<&[usize]> {
        self.children.as_deref()
    }
    pub fn value(&self) -> Value {
        self.value.clone()
    }
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }
    pub fn parent(&self) -> Option<usize> {
        self.parent
    }
    pub fn set_parent(&mut self, index: usize) {
        self.parent.replace(index);
    }
    pub fn add_child(&mut self, index: usize) {
        match self.children {
            Some(ref mut children) => {
                children.push(index);
            }
            ref mut children => {
                children.replace(vec![index]);
            }
        }
    }
    pub fn set_children(&mut self, children: Vec<usize>) {
        self.children.replace(children);
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}
