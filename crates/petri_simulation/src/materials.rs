use bevy::prelude::*;

pub struct Materials {
    pub creature: Handle<ColorMaterial>,
    pub food: Handle<ColorMaterial>,
}

pub(crate) fn material_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let _scenes: Vec<HandleUntyped> = asset_server.load_folder("sprites/").unwrap();

    commands.insert_resource(Materials {
        creature: materials.add(asset_server.get_handle("sprites/creature.png").into()),
        food: materials.add(asset_server.get_handle("sprites/food.png").into()),
    });
}
