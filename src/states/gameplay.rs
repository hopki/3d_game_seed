use std::fmt::Debug;
use std::f32;

use amethyst::{
    assets::{
        Handle, Prefab, PrefabLoader, RonFormat
    },
    core::{Named, Parent, ArcThreadPool, transform::Transform, math::Vector3},
    renderer::{
        camera::{Camera, Projection},
        debug_drawing::DebugLinesComponent,
        light::{DirectionalLight, Light},
        palette::rgb::{Srgb, Srgba},
        resources::AmbientColor,
    },
    ecs::*,/*{
         Entities, Join, ReadStorage, World,
    },*/
    prelude::*,
    window::ScreenDimensions,
    input::{is_key_down, is_mouse_button_down},
    winit::{MouseButton, VirtualKeyCode},
    controls::{FlyControlBundle, HideCursor, FlyControlTag},
};

use crate::components::actors::{Actor, Position};

pub struct GamePlayState{
    //dispatcher: Dispatcher<'static, 'static>,
    //debug_dispatcher: Dispatcher<'static, 'static>,
    //ui_dispatcher: Dispatcher<'static, 'static>,
    //ui: Option<Entity>,
    
    paused: bool,
    desired_time_scale: f32,
    
    ///Camera related
    camera: Option<Entity>,
     /// Z-axis position of the camera.
    ///
    /// The Z axis increases "out of the screen" if the camera faces the XY plane (i.e. towards the
    /// origin from (0.0, 0.0, 1.0)). This is the default orientation, when no rotation is applied to the
    /// camera's transform.
    camera_z: f32,
    /// Depth (Z-axis distance) that the camera can see.
    ///
    /// The camera cannot see things on the limits of its view, i.e. entities with the same Z
    /// coordinate cannot be seen, and entities at `Z - camera_depth_vision` also cannot be seen.
    /// Entities with Z coordinates between these limits are visible.
    camera_depth_vision: f32,
}

impl SimpleState for GamePlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        std::println!("Game playing...");

        //self.dispatcher.setup(&mut data.world);
        //self.debug_dispatcher.setup(&mut data.world);

        // Setup directional light (sun)
        let light_component = Light::Directional(DirectionalLight {
            color: Srgb::new(1.0, 1.0, 1.0),
            intensity: 2.0f32,
            direction: Vector3::new(0.0, 0.3, -1.0),
        });
        world.create_entity().with(light_component).build();

        self.initialise_camera(world);
    }
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        //self.display_loaded_entities(&mut data.world);
        Trans::None
    }
    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let StateData { world, .. } = data;
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                let mut hide_cursor = world.write_resource::<HideCursor>();
                hide_cursor.hide = false;
            } else if is_mouse_button_down(&event, MouseButton::Left) {
                let mut hide_cursor = world.write_resource::<HideCursor>();
                hide_cursor.hide = true;
            }
        }
        Trans::None
    }
}
impl GamePlayState {
    pub fn new(world: &mut World) -> Self {
        // For profiling, the dispatcher needs to specify the pool that is created for us by `ApplicationBuilder::new`.
        // This thread pool will include the necessary setup for `profile_scope`.
        //let pool = (*world.read_resource::<ArcThreadPool>()).clone();
        GamePlayState {
            //dispatcher: DispatcherBuilder::new()
            //    .with_pool(pool)
            //    .build(),
            //debug_dispatcher: DispatcherBuilder::new()
            //    .build(),
            camera: None,
            paused: false,
            desired_time_scale: 1.0,
            camera_z: 8.0,
            camera_depth_vision: 1000.0f32,
        }
    }
    //todo: move camera to player avatar, implement 3d person veiw
 /// This method initialises a camera which will view our world.
 fn initialise_camera(&mut self, world: &mut World) {
    // Position the camera. Here we translate it forward (out of the screen) far enough to view
    // all of the sprites. Note that camera_z is 1.0, whereas the furthest sprite is -11.0.
    //
    // For the depth, the additional + 1.0 is needed because the camera can see up to, but
    // excluding, entities with a Z coordinate that is `camera_z - camera_depth_vision`. The
    // additional distance means the camera can see up to just before -12.0 on the Z axis, so
    // we can view the sprite at -11.0.
    self.camera_z = 2.0;
    //self.camera_depth_vision =
      //  self.loaded_sprite_sheet.as_ref().unwrap().sprite_count as f32 + 1.0;

    /*let prefab_handle = world.exec(|loader: PrefabLoader<'_, Camera>| {
        loader.load("prefab/fly_camera.ron", RonFormat, ())
    });
    world
        .create_entity()
        .named("Camera")
        .with(prefab_handle)
        .build();
    */
    self.adjust_camera(world);
}

fn adjust_camera(&mut self, world: &mut World) {
    if let Some(camera) = self.camera.take() {
        world
            .delete_entity(camera)
            .expect("Failed to delete camera entity.");
    }

    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    let zoom_factor = 95.0;

    let mut camera_transform = Transform::default();
    camera_transform.set_translation_xyz(5.0, 0.0, self.camera_z);
    let pi = f32::consts::PI;
    camera_transform.set_rotation_euler(pi/3.0, 0.0, pi/2.0);

    let camera = world
        .create_entity()
        .with(camera_transform)
        // Define the view that the camera can see. It makes sense to keep the `near` value as
        // 0.0, as this means it starts seeing anything that is 0 units in front of it. The
        // `far` value is the distance the camera can see facing the origin.
        /*orthographic(
            -width / zoom_factor,
            width / zoom_factor,
            -height / zoom_factor,
            height / zoom_factor,
            0.1f32,
            self.camera_depth_vision, */
        .with(Camera::from(Projection::perspective(
            1.3,
            1.0471975512,
            0.1f32,
            self.camera_depth_vision,
        )))
        .with(FlyControlTag)
        .build();

    self.camera = Some(camera);
    }
    // Displays the `Component`s of entities in the `World`.
    fn display_loaded_entities(&self, world: &mut World) {
        println!("Entities");
        println!("========");
        println!();
        println!(
            "| {e:24} | {prefab_handle:22} | {parent:6} | {pos:23} | {named:22} |",
            e = "Entity",
            prefab_handle = "Handle<Prefab<Actor>>",
            parent = "Parent",
            pos = "Position",
            named = "Actor",
        );
        println!(
            "| {c:-^24} | {c:-^22} | {c:-^6} | {c:-^23} | {c:-^22} |",
            c = "",
        );
        world.exec(
            |(entities, prefab_handles, parents, positions, nameds): (
                Entities,
                ReadStorage<Handle<Prefab<Actor>>>,
                ReadStorage<Parent>,
                ReadStorage<Position>,
                ReadStorage<Named>,
            )| {
                (
                    &entities,
                    prefab_handles.maybe(),
                    parents.maybe(),
                    positions.maybe(),
                    nameds.maybe(),
                )
                    .join()
                    .for_each(|(e, prefab_handle, parent, pos, named)| {
                        println!(
                            "| {e:24} | {prefab_handle:22} | {parent:6} | {pos:23} | {named:22} |",
                            e = format!("{:?}", e),
                            prefab_handle = Self::display(prefab_handle),
                            parent = Self::display(parent.map(|p| p.entity)),
                            pos = Self::display(pos),
                            named = Self::display(named),
                        )
                    });
            },
        )
    }

    fn display<T: Debug>(component: Option<T>) -> String {
        if let Some(component) = component {
            format!("{:?}", component)
        } else {
            format!("{:?}", component)
        }
    }
}