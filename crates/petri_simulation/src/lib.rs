#![allow(clippy::type_complexity)]

use bevy::{core::FixedTimestep, prelude::*, render::camera::WindowOrigin};

pub use crate::{brain::*, eye::*, creature::*, food::*, materials::*, simulation::*};

mod brain;
mod eye;
mod creature;
mod food;
mod materials;
mod simulation;
mod utils;

const SIM_UPDATE: f64 = 1.0 / 60.0;

fn camera_setup(mut commands: Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();

    camera_bundle.orthographic_projection.window_origin = WindowOrigin::BottomLeft;

    commands
        // And use an orthographic projection
        .spawn_bundle(camera_bundle);
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, StageLabel)]
struct SimulationSetupStage;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(camera_setup.system())
            .add_startup_system(material_setup.system())
            .add_startup_system(simulation_setup.system())
            .add_startup_stage(
                SimulationSetupStage,
                SystemStage::parallel()
                    .with_system(creature_setup.system())
                    .with_system(food_setup.system()),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(SIM_UPDATE))
                    .with_system(detect_food_collisions.system())
                    .with_system(creatures_thinking.system())
                    .with_system(move_creatures.system()),
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
