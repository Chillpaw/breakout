use bevy::ecs::system::Query;
use bevy::ecs::world::World;
use bevy::math::Vec3;

use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}
