#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Bundle, Clone)]
pub struct Shadow(Mesh2d, MeshMaterial2d<ColorMaterial>);
impl Shadow {
    pub fn new(mesh2d_handle: Handle<Mesh>, color_handle: Handle<ColorMaterial>) -> Self {
        Self(Mesh2d(mesh2d_handle), MeshMaterial2d(color_handle))
    }
}
