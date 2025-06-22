use std::fmt::{Debug, Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::arena::obj_desc::Description;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Object {
    idx: usize,
    val: Description,
    parent: Option<usize>,
    children: Option<Vec<usize>>,
}

impl Object {
    pub fn new(value: impl Into<Description>, index: usize, parent: Option<usize>) -> Self {
        Object {
            idx: index,
            val: value.into(),
            parent,
            children: None,
        }
    }
    pub fn new_with_children(value: impl Into<Description>, index: usize, parent: Option<usize>, children: Option<Vec<usize>>) -> Self {
        Object {
            idx: index,
            val: value.into(),
            parent,
            children,
        }
    }
    pub fn children(&self) -> Option<&[usize]> {
        self.children.as_deref()
    }

    pub fn describe(&self) -> Description {
        self.val.clone()
    }
    pub fn position(&self) -> usize {
        self.idx
    }
    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }
    pub fn get_parent(&self) -> Option<usize> {
        self.parent
    }
    pub fn update_parent(&mut self, index: usize) {
        self.parent.replace(index);
    }

    pub fn update_children(&mut self, index: usize) {
        match self.children {
            Some(ref mut children) => {
                children.push(index);
            }
            ref mut children => {
                children.replace(vec![index]);
            }
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.val)
    }
}