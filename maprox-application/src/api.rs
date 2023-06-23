pub mod connection;
mod events;

use bevy::prelude::*;

pub struct BevySpawnStructs<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
}
