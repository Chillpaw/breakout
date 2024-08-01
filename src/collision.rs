use bevy::prelude::*;

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

pub fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collier)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    
}
