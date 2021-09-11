use std::f32::consts::FRAC_PI_2;

use crate::utils::*;
use crate::*;
use petri_rand::PetriRand;

#[derive(Debug, Default)]
pub struct Simulation {
    pub world: Vec2,
    pub creatures: usize,
    pub food: usize,
}

const SPEED_MIN: f32 = 0.1;
const SPEED_MAX: f32 = 2.0;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;

pub(crate) fn simulation_setup(mut commands: Commands) {
    commands.insert_resource(Simulation {
        world: Vec2::splat(800.0),
        creatures: 40,
        food: 60,
    });
}

pub(crate) fn detect_food_collisions(
    mut q_food: Query<&mut Transform, (With<Food>, Without<Creature>)>,
    mut q_creatures: Query<(&Transform, &mut Fitness), (With<Creature>, Without<Food>)>,
    sim: Res<Simulation>,
) {
    let rng = PetriRand::thread_local();

    for (creature, mut fitness) in q_creatures.iter_mut() {
        for mut food in q_food.iter_mut() {
            let distance = creature.translation.distance(food.translation);

            if distance < 6.0 {
                fitness.score += 1.0;

                food.translation =
                    Vec2::new(rng.get_f32() * sim.world.x, rng.get_f32() * sim.world.y).extend(0.0);
            }
        }
    }
}

pub(crate) fn creatures_thinking(
    mut creatures: Query<(&Transform, &mut Control, &Eye, &Brain), (With<Creature>, Without<Food>)>,
    food: Query<&Transform, (With<Food>, Without<Creature>)>,
) {
    for (creature, mut control, eye, brain) in creatures.iter_mut() {
        let vision = eye.perceive(creature, food.iter());

        let vision = brain.nn.propagate(vision);

        let speed = vision[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);

        control.vector = Vec3::new(
            (control.vector.x + speed).clamp(SPEED_MIN, SPEED_MAX),
            0.0,
            0.0,
        );
        control.rotation = Quat::from_rotation_z(vision[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL));
    }
}

pub(crate) fn move_creatures(
    mut creatures: Query<(&mut Transform, &Control, With<Creature>)>,
    sim: Res<Simulation>,
) {
    for (mut transform, control, _) in creatures.iter_mut() {
        transform.rotation =  control.rotation;
        transform.translation += control.rotation.mul_vec3(control.vector);
        transform.translation.x = wrap(transform.translation.x, 0.0, sim.world.x);
        transform.translation.y = wrap(transform.translation.y, 0.0, sim.world.y);
    }
}
