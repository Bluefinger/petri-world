use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::*,
    prelude::*,
    render::camera::WindowOrigin,
};
use petri_rand::PetriRand;
use petri_simulation as sim;
use std::{f32::consts::PI, iter::repeat_with};

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

    commands.insert_resource(sim::Simulation {
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

fn creature_setup(mut commands: Commands, materials: Res<Materials>, sim: Res<sim::Simulation>) {
    let rng = PetriRand::thread_local();

    repeat_with(|| {
        let translation = Vec3::new(
            rng.get_f32() * sim.world.x,
            rng.get_f32() * sim.world.y,
            1.0,
        );
        let rotation = Quat::from_rotation_z(rng.get_f32() * 2.0 * PI);
        let scale = Vec2::splat(0.07).extend(1.0);

        sim::CreatureBundle {
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
    .for_each(|bundle| {
        commands.spawn_bundle(bundle);
    });
}

fn food_setup(mut commands: Commands, materials: Res<Materials>, sim: Res<sim::Simulation>) {
    let rng = PetriRand::thread_local();

    repeat_with(|| {
        let translation = Vec3::new(
            rng.get_f32() * sim.world.x,
            rng.get_f32() * sim.world.y,
            1.0,
        );
        let scale = Vec2::splat(0.05).extend(1.0);

        sim::FoodBundle {
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
    .for_each(|bundle| {
        commands.spawn_bundle(bundle);
    });
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE))
        .insert_resource(WindowDescriptor {
            title: "Petri World!".to_string(),
            width: 800.0,
            height: 800.0,
            ..Default::default()
        })
        .insert_resource(Msaa {
            samples: 4
        })
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup_system.system())
        .add_startup_stage(
            "game_setup_actors",
            SystemStage::parallel()
                .with_system(creature_setup.system())
                .with_system(food_setup.system()),
        )
        .run();
}
