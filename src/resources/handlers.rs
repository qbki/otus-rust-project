use bevy::prelude::*;
use std::collections::HashMap;

pub struct Handlers {
    pub materials: HashMap<String, Handle<StandardMaterial>>,
    pub meshes: HashMap<String, Handle<Mesh>>,
}

impl Handlers {
    pub fn new() -> Self {
        Self {
            materials: HashMap::new(),
            meshes: HashMap::new(),
        }
    }
}
