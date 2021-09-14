use std::f32::consts::{FRAC_PI_4, PI};

use crate::utils::*;
use crate::*;
use bevy::{ecs::schedule::ShouldRun, tasks::ComputeTaskPool};
use petri_ga::{GaussianMutation, GeneticAlgorithm, RouletteWheelSelection, UniformCrossover};
use petri_nn::Network;
use petri_rand::PetriRand;

const SPEED_MIN: f32 = 0.1;
const SPEED_MAX: f32 = 6.0;
const SPEED_ACCEL: f32 = 0.5;
const ROTATION_ACCEL: f32 = FRAC_PI_4;

#[derive(Debug)]
pub struct Simulation {
    pub world: Vec2,
    pub creatures: usize,
    pub food: usize,
}

#[derive(Debug, Default)]
pub struct Lifecycle {
    pub limit: usize,
    pub step: usize,
}

#[derive(Debug)]
pub struct Evolver {
    ga: GeneticAlgorithm<'static, RouletteWheelSelection, UniformCrossover, GaussianMutation>,
}

pub(crate) fn simulation_setup(mut commands: Commands) {
    commands.insert_resource(Simulation {
        world: Vec2::splat(800.0),
        creatures: 40,
        food: 60,
    });

    commands.insert_resource(Evolver {
        ga: GeneticAlgorithm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::new(),
            GaussianMutation::new(0.01, 0.3),
        ),
    });

    commands.insert_resource(Lifecycle {
        limit: 2000,
        step: 0
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

            if distance < 7.0 {
                fitness.score += 1.0;

                food.translation =
                    Vec2::new(rng.get_f32() * sim.world.x, rng.get_f32() * sim.world.y).extend(0.0);
            }
        }
    }
}

pub(crate) fn creatures_thinking(
    mut creatures: Query<
        (&Transform, &mut Control, &Eye, &Network),
        (With<Creature>, Without<Food>),
    >,
    food: Query<&Transform, (With<Food>, Without<Creature>)>,
    pool: Res<ComputeTaskPool>,
) {
    creatures.par_for_each_mut(&pool, 10, |(creature, mut control, eye, brain)| {
        let vision = eye.perceive(creature, food.iter());

        let vision = brain.propagate(vision);

        let r0 = vision[0].clamp(0.0, 1.0);
        let r1 = vision[1].clamp(0.0, 1.0) - 0.5;
        let r2 = vision[2].clamp(0.0, 1.0);

        let speed = r1.clamp(-SPEED_ACCEL, SPEED_ACCEL);

        control.speed = (control.speed + speed).clamp(SPEED_MIN, SPEED_MAX);

        control.rotation += (r0 - r2).clamp(-ROTATION_ACCEL, ROTATION_ACCEL);
    });
}

pub(crate) fn move_creatures(
    mut creatures: Query<(&mut Transform, &Control, With<Creature>)>,
    mut lifecycle: ResMut<Lifecycle>,
    sim: Res<Simulation>,
) {
    for (mut transform, control, _) in creatures.iter_mut() {
        let rot = Quat::from_rotation_z(control.rotation);
        transform.rotation = rot;
        transform.translation += rot.mul_vec3(Vec3::new(control.speed, 0.0, 0.0));
        transform.translation.x = wrap(transform.translation.x, 0.0, sim.world.x);
        transform.translation.y = wrap(transform.translation.y, 0.0, sim.world.y);
    }

    lifecycle.step += 1;
}

pub(crate) fn evolve_when_ready(lifecycle: Res<Lifecycle>) -> ShouldRun {
    if lifecycle.step == lifecycle.limit {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub(crate) fn evolve_creatures(
    mut creatures: Query<(&mut Network, &mut Fitness, &mut Transform), With<Creature>>,
    mut lifecycle: ResMut<Lifecycle>,
    sim: Res<Simulation>,
    evolver: Res<Evolver>,
) {
    let population: Vec<CreatureIndividual> = creatures
        .iter_mut()
        .map(|(brain, fitness, _)| CreatureIndividual::from_creature(&brain, &fitness))
        .collect();

    let rng = PetriRand::thread_local();

    let new_population = evolver.ga.evolve(&rng, &population).unwrap();

    creatures.iter_mut().zip(new_population).for_each(
        |((mut brain, mut fitness, mut transform), individual)| {
            fitness.score = 0.0;
            brain.adjust_weights(individual);

            transform.translation = Vec3::new(
                rng.get_f32() * sim.world.x,
                rng.get_f32() * sim.world.y,
                1.0,
            );
            transform.rotation = Quat::from_rotation_z(rng.get_f32() * 2.0 * PI);
        },
    );

    lifecycle.step = 0;
}

pub(crate) fn randomise_food(
    mut foods: Query<&mut Transform, (With<Food>, Without<Creature>)>,
    sim: Res<Simulation>,
) {
    let rng = PetriRand::thread_local();

    for mut food in foods.iter_mut() {
        food.translation = Vec3::new(
            rng.get_f32() * sim.world.x,
            rng.get_f32() * sim.world.y,
            0.0,
        );
    }
}
