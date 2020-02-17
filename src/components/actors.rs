use std::fmt::Debug;

use amethyst::{
    assets::{
        PrefabData, ProgressCounter, AssetPrefab
    },
    core::{Named, },
    derive::PrefabData,
    ecs::{
        storage::DenseVecStorage, Component, Entity, 
        WriteStorage,
    },
    gltf::{GltfSceneAsset, GltfSceneFormat},
    Error,
};
use serde::{Deserialize, Serialize};

//Components
#[derive(Clone, Copy, Component, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Position(pub f32, pub f32, pub f32);

//Actor
#[derive(Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct Actor {
    actor: Named,
    position: Position,
    gltf: Option<AssetPrefab<GltfSceneAsset, GltfSceneFormat>>,
}

//avatar