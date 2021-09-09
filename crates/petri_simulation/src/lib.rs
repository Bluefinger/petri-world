use bevy::prelude::*;

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

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
