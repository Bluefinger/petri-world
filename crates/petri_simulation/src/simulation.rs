use bevy::prelude::*;
use petri_rand::PetriRand;
use crate::utils::*;
use crate::*;

#[derive(Debug, Default)]
pub struct Simulation {
    pub world: Vec2,
    pub creatures: usize,
    pub food: usize,
}

pub(crate) fn simulation_setup(mut commands: Commands) {
    commands.insert_resource(Simulation {
        world: Vec2::splat(800.0),
        creatures: 40,
        food: 60,
    });
}

pub(crate) fn detect_food_collisions(
    mut q_food: Query<&mut Transform, (With<Food>, Without<Creature>)>,
    mut q_creatures: Query<&mut Transform, (With<Creature>, Without<Food>)>,
    sim: Res<Simulation>,
) {
    let rng = PetriRand::thread_local();

    for creature in q_creatures.iter_mut() {
        for mut food in q_food.iter_mut() {
            let distance = creature.translation.distance(food.translation);

            if distance < 5.0 {
                food.translation =
                    Vec2::new(rng.get_f32() * sim.world.x, rng.get_f32() * sim.world.y).extend(0.0);
            }
        }
    }
}

pub(crate) fn move_creatures(
    mut creatures: Query<(&mut Transform, &Velocity, With<Creature>)>,
    sim: Res<Simulation>,
) {
    for (mut transform, _, _) in creatures.iter_mut() {
        let rot = transform.rotation;
        transform.translation += rot.mul_vec3(Vec3::new(0.0, 2.0, 0.0));
        transform.translation.x = wrap(transform.translation.x, 0.0, sim.world.x);
        transform.translation.y = wrap(transform.translation.y, 0.0, sim.world.y);
    }
}
