use hecs::World;

pub mod components;
pub mod systems;

pub struct Ecs {
    pub world: World,
}

impl Ecs {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }
}

impl Default for Ecs {
    fn default() -> Self {
        Self::new()
    }
}
