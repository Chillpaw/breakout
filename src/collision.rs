use bevy::math::bounding::Aabb2d;
use bevy::math::bounding::BoundingCircle;
use bevy::math::bounding::BoundingVolume;
use bevy::math::bounding::IntersectsVolume;
use bevy::prelude::*;



#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Collision {
    Left,
    Right,
    Up,
    Down,
}

pub fn ball_collision(ball: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
    if ball.intersects(&bounding_box) {
        let closest_point = bounding_box.closest_point(ball.center());
        let offset_point = ball.center() - closest_point;
        let side = if offset_point.x.abs() > offset_point.y.abs() {
            if offset_point.x > 0.0 {
                Collision::Right
            } else {
                Collision::Left
            }
        } else {
            if offset_point.y > 0.0 {
                Collision::Up
            } else {
                Collision::Down
            }
        };

        Some(side)
    } else {
        None
    }
}
