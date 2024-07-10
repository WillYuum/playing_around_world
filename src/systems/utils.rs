use bevy::prelude::*;

pub fn rotation_towards_target(position: Vec3, target: Vec3) -> Quat {
    let direction = (target - position).normalize();
    let angle_y = direction.x.atan2(direction.z);
    Quat::from_rotation_y(angle_y)
}