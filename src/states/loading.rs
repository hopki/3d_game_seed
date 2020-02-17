use amethyst::{
    assets::{
        Handle, Prefab, PrefabLoader, 
        ProgressCounter, RonFormat,
    },
    ecs::{
        World,
    },
    prelude::*,
};
use derive_new::new;

use crate::components::actors::Actor;
use crate::states::gameplay::GamePlayState;

#[derive(new)]
pub struct LoadingState {
    /// Tracks loaded assets.
    #[new(default)]
    pub progress_counter: ProgressCounter,
    
    /// Handles to loaded prefabs.
    //multiple prefabs?
    #[new(default)]
    pub avatar_prefab_handle: Option<Handle<Prefab<Actor>>>,
    #[new(default)]
    pub monster_prefab_handle: Option<Handle<Prefab<Actor>>>,

}

impl SimpleState for LoadingState{
    //manually register resources in on_start if needed

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        std::println!("Loading...");
        //load avatar prefab
        let prefab_handle = data.world.exec(|loader: PrefabLoader<'_, Actor>| {
            loader.load(
                "prefab/player.ron",
                RonFormat,
                &mut self.progress_counter,
            )
        });
        std::println!("Loaded player.");

        //initialize prefabs
        self.initialize_actor_prefab( &prefab_handle, data.world);
        self.avatar_prefab_handle = Some(prefab_handle);

        //load moster prefab
        let prefab_handle = data.world.exec(|loader: PrefabLoader<'_, Actor>| {
            loader.load(
                "prefab/monster.ron",
                RonFormat,
                &mut self.progress_counter,
            )
        });
        std::println!("Loaded monster");
        //initialize prefabs
        self.initialize_actor_prefab( &prefab_handle, data.world);
        self.monster_prefab_handle = Some(prefab_handle);
        
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        
        if self.progress_counter.is_complete() {
            //self.display_loaded_prefab(&data.world);
            std::println!("In update..");
            Trans::Switch(Box::new(GamePlayState::new(data.world)))
        } else {
            Trans::None
        }
    }
}
impl LoadingState{
    //initialize player avatar

    //retrieve prefabs by name?

    fn initialize_actor_prefab(&mut self, prefab_handle: & Handle<Prefab<Actor>>, world: &mut World){
        let entity = world
            .create_entity()
            .with(prefab_handle.clone())
            .build();

        std::println!("id:{}", entity.id());
    }

    // Displays the contents of the loaded prefab.
    /*fn display_loaded_prefab(&self, world: &World) {
        let prefab_assets = world.read_resource::<AssetStorage<Prefab<Actor>>>();
        if let Some(handle) = self.avatar_prefab_handle.as_ref() {
            let prefab = prefab_assets
                .get(handle)
                .expect("Expected prefab to be loaded.");

            println!("Prefab");
            println!("======");
            prefab
                .entities()
                .for_each(|entity| println!("{:?}", entity));
            println!();
        }
        if let Some(handle) = self.monster_prefab_handle.as_ref() {
            let prefab2 = prefab_assets
                .get(handle)
                .expect("Expected prefab to be loaded.");
                println!("Prefab2");
                println!("======");
                prefab2
                    .entities()
                    .for_each(|entity| println!("{:?}", entity));
                println!();
        }
    }*/
}