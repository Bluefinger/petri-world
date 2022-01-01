use std::iter::repeat_with;

use crate::*;
use petri_rand::PetriRand;

#[derive(Component, Debug, Default)]
pub struct Food;

#[derive(Default, Bundle)]
pub struct FoodBundle {
    pub food: Food,
    #[bundle]
    pub sprite: SpriteBundle,
}

pub(crate) fn food_setup(mut commands: Commands, materials: Res<Materials>, sim: Res<Simulation>) {
    let rng = PetriRand::new();

    let food: Vec<FoodBundle> = repeat_with(|| {
        let translation = Vec3::new(
            rng.get_f32() * sim.world.x,
            rng.get_f32() * sim.world.y,
            0.0,
        );
        let scale = Vec2::splat(0.05).extend(1.0);

        FoodBundle {
            sprite: SpriteBundle {
                texture: materials.food.clone(),
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
