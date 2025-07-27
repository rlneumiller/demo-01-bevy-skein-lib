// This file is part of the demo-01-bevy-skein-lib crate.
// It provides the necessary components and utilities to work with Bevy Skein scenes.
use bevy::prelude::*;
use bevy::reflect::Reflect;


#[derive(Component)]
pub struct SceneRoot(pub Handle<Scene>);

#[derive(Debug, Clone)]
pub enum GltfAssetLabel {
    Scene(usize),
}

/// Helper to load a GLTF scene using AssetServer
pub fn load_gltf_scene(asset_server: &AssetServer, asset_path: &str, scene_index: usize) -> Handle<Scene> {
    // Bevy requires the "#SceneN" suffix to load a Scene from a GLTF file
    let path = if scene_index == 0 {
        format!("{}#Scene0", asset_path)
    } else {
        format!("{}#Scene{}", asset_path, scene_index)
    };
    asset_server.load(path)
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct MagicBox {
    pub sparkle_intensity: f32,
    pub color: Color, // Each channel is specified as the range [0.0, 1.0]
}

