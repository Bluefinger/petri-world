use bevy::prelude::*;

pub struct Materials {
    pub creature: Handle<Image>,
    pub food: Handle<Image>,
}

pub(crate) fn material_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let _scenes: Vec<HandleUntyped> = asset_server.load_folder("sprites/").unwrap();

    commands.insert_resource(Materials {
        creature: asset_server.get_handle("sprites/creature.png"),
        food: asset_server.get_handle("sprites/food.png"),
    });
}
