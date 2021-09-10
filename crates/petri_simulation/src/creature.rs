use std::{f32::consts::PI, iter::repeat_with};

use bevy::prelude::*;
use petri_rand::PetriRand;
use crate::{materials::Materials, simulation::Simulation};

#[derive(Debug, Default)]
pub struct Creature;

#[derive(Debug, Default)]
pub struct Velocity {
    pub vector: Vec3,
}

#[derive(Default, Bundle)]
pub struct CreatureBundle {
    pub creature: Creature,
    pub velocity: Velocity,
    #[bundle]
    pub sprite: SpriteBundle,
}

pub fn creature_setup(mut commands: Commands, materials: Res<Materials>, sim: Res<Simulation>) {
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
