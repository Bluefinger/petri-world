use std::{f32::consts::PI, iter::repeat_with};

use bevy::{prelude::*, render::camera::WindowOrigin};
use petri_rand::PetriRand;

#[derive(Debug, Default)]
pub struct Simulation {
    pub world: Vec2,
    pub creatures: usize,
    pub food: usize,
}

#[derive(Debug, Default)]
pub struct Creature;

#[derive(Debug, Default)]
pub struct Food;

#[derive(Default, Bundle)]
pub struct FoodBundle {
    pub food: Food,
    #[bundle]
    pub sprite: SpriteBundle,
}

#[derive(Default, Bundle)]
pub struct CreatureBundle {
    pub creature: Creature,
    pub velocity: Velocity,
    #[bundle]
    pub sprite: SpriteBundle,
}

#[derive(Debug, Default)]
pub struct Velocity {
    pub vector: Vec3,
}

struct Materials {
    creature: Handle<ColorMaterial>,
    food: Handle<ColorMaterial>,
}

fn startup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let _scenes: Vec<HandleUntyped> = asset_server.load_folder("sprites/").unwrap();

    commands.insert_resource(Materials {
        creature: materials.add(asset_server.get_handle("sprites/creature.png").into()),
        food: materials.add(asset_server.get_handle("sprites/food.png").into()),
    });

    commands.insert_resource(Simulation {
        world: Vec2::splat(800.0),
        creatures: 40,
        food: 60,
    });

    let mut camera_bundle = OrthographicCameraBundle::new_2d();

    camera_bundle.orthographic_projection.window_origin = WindowOrigin::BottomLeft;

    commands
        // And use an orthographic projection
        .spawn_bundle(camera_bundle);
}

fn creature_setup(mut commands: Commands, materials: Res<Materials>, sim: Res<Simulation>) {
    let rng = PetriRand::thread_local();

    let creatures: Vec<CreatureBundle> = repeat_with(|| {
        let translation = Vec3::new(
            rng.get_f32() * sim.world.x,
            rng.get_f32() * sim.world.y,
            1.0,
        );
        let rotation = Quat::from_rotation_z(rng.get_f32() * 2.0 * PI);
        let scale = Vec2::splat(0.07).extend(1.0);

        CreatureBundle {
            sprite: SpriteBundle {
                material: materials.creature.clone(),
                transform: Transform {
                    translation,
                    rotation,
                    scale,
                },
                ..Default::default()
            },
            ..Default::default()
        }
    })
    .take(sim.creatures)
    .collect();

    commands.spawn_batch(creatures);
}

fn food_setup(mut commands: Commands, materials: Res<Materials>, sim: Res<Simulation>) {
    let rng = PetriRand::thread_local();

    let food: Vec<FoodBundle> = repeat_with(|| {
        let translation = Vec3::new(
            rng.get_f32() * sim.world.x,
            rng.get_f32() * sim.world.y,
            0.0,
        );
        let scale = Vec2::splat(0.05).extend(1.0);

        FoodBundle {
            sprite: SpriteBundle {
                material: materials.food.clone(),
                transform: Transform {
                    translation,
                    scale,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    })
    .take(sim.food)
    .collect();

    commands.spawn_batch(food);
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, StageLabel)]
struct SimulationSetupStage;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup_system.system())
            .add_startup_stage(
                SimulationSetupStage,
                SystemStage::parallel()
                    .with_system(creature_setup.system())
                    .with_system(food_setup.system()),
            );
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
