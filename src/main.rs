use amethyst::{
    assets::{
        PrefabLoaderSystemDesc, 
    },
    core::transform::TransformBundle,
    gltf::{GltfSceneLoaderSystemDesc},
    prelude::*,
    renderer::{
        plugins::{RenderPbr3D, RenderSkybox, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    input::{is_key_down, is_mouse_button_down, InputBundle, StringBindings},
    controls::{FlyControlBundle}
};

mod states;
mod components;

use crate::states::{loading::LoadingState};
use crate::components::actors::Actor;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    // Add our meshes directory to the asset loader.
    let assets_dir = app_root.join("assets");

    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let key_bindings_path = config_dir.join("input.ron");

    let game_data = GameDataBuilder::default()
        .with_system_desc(
            PrefabLoaderSystemDesc::<Actor>::default(),
            "actor_loader",
            &[],
        )
        .with_system_desc(
            GltfSceneLoaderSystemDesc::default(),
            "gltf_loader",
            &["actor_loader"], // This is important so that entity instantiation is performed in a single frame.
        )
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?)
                    .with_plugin(RenderPbr3D::default().with_skinning())
                    .with_plugin(RenderSkybox::default()),
        )?
        .with_bundle(
            FlyControlBundle::<StringBindings>::new(
                Some(String::from("move_x")),
                Some(String::from("move_y")),
                Some(String::from("move_z")),
            )
            .with_sensitivity(0.1, 0.1),
        )?
        .with_bundle(TransformBundle::new().with_dep(&["fly_movement"]))?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?;

    let mut game = Application::new(assets_dir, LoadingState::new(), game_data)?;
    game.run();

    Ok(())
}
