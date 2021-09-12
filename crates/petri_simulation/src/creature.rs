use std::{f32::consts::{FRAC_2_PI, PI}, iter::repeat_with};

use bevy::prelude::*;
use petri_nn::Network;
use petri_rand::PetriRand;
use crate::{Brain, Eye, materials::Materials, simulation::Simulation};

#[derive(Debug, Default)]
pub struct Creature;

#[derive(Debug, Default)]
pub struct Control {
    pub speed: f32,
    pub rotation: f32,
}

#[derive(Debug, Default)]
pub struct Fitness {
    pub score: f32,
}

#[derive(Bundle)]
pub struct CreatureBundle {
    pub creature: Creature,
    pub control: Control,
    pub eye: Eye,
    pub brain: Brain,
    pub fitness: Fitness,
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
            creature: Creature,
            sprite: SpriteBundle {
                material: materials.creature.clone(),
                transform: Transform {
                    translation,
                    rotation,
                    scale,
                },
                ..Default::default()
            },
            control: Default::default(),
            fitness: Default::default(),
            eye: Eye { fov_range: 200.0, fov_angle: FRAC_2_PI, cells: 11 },
            brain: Brain { nn: Network::random(&rng, vec![11, 22, 11, 6, 3]) },
        }
    })
    .take(sim.creatures)
    .collect();

    commands.spawn_batch(creatures);
}
