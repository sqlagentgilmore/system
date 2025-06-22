use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Deserialize, Serialize};
use crate::arena::obj_desc::Description;
use crate::arena::types::SystemType;
use crate::client::system::{DatabaseChild, DatabaseParent};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct ObjectArg {
    arg: usize,
    system: SystemType
}

impl ObjectArg {
    fn get_position(&self) -> usize {
        self.arg
    }
}

impl Into<ObjectArg> for DatabaseParent {
    fn into(self) -> ObjectArg {
        ObjectArg {
            arg: self.get_position(),
            system: self.get_system_type(),
        }
    }
}

impl Into<ObjectArg> for (usize, &str) {
    fn into(self) -> ObjectArg {
        ObjectArg {
            arg: self.0,
            system: SystemType::from(self.1)
        }
    }
}

impl Into<ObjectArg> for (usize, SystemType) {
    fn into(self) -> ObjectArg {
        ObjectArg {
            arg: self.0,
            system: self.1
        }
    }
}

impl Into<ObjectArg> for (DatabaseChild, SystemType) {
    fn into(self) -> ObjectArg {
        ObjectArg {
            arg: self.0.get_position(),
            system: self.1
        }
    }
}

pub trait Load {
    fn get_system_type(&self) -> SystemType;
    fn get_position(&self) -> usize;
    fn get_description(&self) -> Description;
    async fn connect(&self) -> bool {
        let object = (self.get_position(), self.get_system_type());
        let args = serde_wasm_bindgen::to_value::<ObjectArg>(&object.into());
        if args.is_ok() {
            let result = serde_wasm_bindgen::from_value::<String>(invoke("connect", unsafe { args.unwrap_unchecked() }).await);
            if result.is_ok() {
                unsafe { result.unwrap_unchecked() };
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    async fn child_to_parent(&self, object: DatabaseChild, system_type: SystemType) -> Option<DatabaseParent> {
        let new_parent_position = object.get_position();
        let new_parent_description = object.get_description().clone();
        let args = serde_wasm_bindgen::to_value::<ObjectArg>(&(object, system_type).into());
        if args.is_ok() {
            let result = serde_wasm_bindgen::from_value::<Vec<(usize, Description)>>(invoke("get_children", unsafe { args.unwrap_unchecked() }).await);
            if result.is_ok() {
                let children = unsafe { result.unwrap_unchecked() }.into_iter().map(|val| DatabaseChild::new(val.0, val.1) ).collect();
                Some(DatabaseParent::new(new_parent_position, children, new_parent_description, system_type))
                
            } else {
                None
            }
        } else {
            None
        }
    }
}