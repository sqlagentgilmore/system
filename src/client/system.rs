use crate::arena::arena::ArenaTree;
use crate::arena::obj_desc::Description;
use crate::arena::types::SystemType;
use crate::client::load::Load;

pub struct Systems {
    _system: ArenaTree,
    _type: SystemType,
}

pub struct DatabaseParent {
    position: usize,
    children: Vec<DatabaseChild>,
    description: Description,
    system_type: SystemType
}

pub struct DatabaseChild {
    position: usize,
    description: Description,
}

impl DatabaseParent {
    pub fn new(position: usize, children: Vec<DatabaseChild>, description: impl Into<Description>, system_type: SystemType) -> Self {
        Self {
            position,
            children,
            description: description.into(),
            system_type
        }
    }
    
    pub fn get_position(&self) -> usize {
        self.position
    }
    pub fn get_system_type(&self) -> SystemType {
        self.system_type
    }
}

impl DatabaseChild {
    pub fn new(position: usize, description: impl Into<Description>) -> Self {
        Self {
            position,
            description: description.into()
        }
    }
    
    pub fn get_position(&self) -> usize {
        self.position
    }
    
    pub fn get_description(&self) -> &Description {
        &self.description
    }
}

impl Systems {
    pub fn new(system: impl Into<SystemType>) -> Self {
        Self {
            _system: ArenaTree::new(None),
            _type: system.into()
        }
    }
    
    pub fn get_system(&self) -> &ArenaTree {
        &self._system
    }
    
    pub fn get_system_type(&self) -> &SystemType {
        &self._type
    }    
    
    pub fn get_system_mut(&mut self) -> &mut ArenaTree {
        &mut self._system
    }
    
}

impl Load for DatabaseParent {
    fn get_system_type(&self) -> SystemType {
        self.system_type
    }

    fn get_position(&self) -> usize {
        self.position
    }

    fn get_description(&self) -> Description {
        self.description.clone()
    }
}