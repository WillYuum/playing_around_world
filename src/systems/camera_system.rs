
use bevy::{ input::{keyboard::Key, mouse::{MouseButtonInput, MouseMotion, MouseWheel}}, math::VectorSpace, prelude::*};

use crate::resources::camera_state::CameraState;

pub fn rotate_camera(
    time: Res<Time>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut camera_state: ResMut<CameraState>,
    mut query: Query<&mut Transform, With<Camera>>
){
    let rotation_speed: f32 = 0.2;

    if mouse_buttons.just_pressed(MouseButton::Left) {
        camera_state.rotating = true;
    }

    if mouse_buttons.just_released(MouseButton::Left) {
        camera_state.rotating = false;
    }


    if(camera_state.rotating){
        println!("Left Mouse Being Pressed.");
        for mouse_motion in evr_motion.read() {

            for mut transform in query.iter_mut() {
                    let mouse_motion_delta = mouse_motion.delta.x;
                    println!("Updating Transform...");
                    transform.rotate_around(
                        Vec3::ZERO,
                        Quat::from_rotation_y(-mouse_motion_delta * rotation_speed * time.delta_seconds()),
                    );
                };
        }
    }   
}


pub fn zoom_camera(
    mut evr_scroll: EventReader<MouseWheel>,
    mut camera_state: ResMut<CameraState>,
    mut query: Query<&mut Transform, With<Camera>>
){
    let zoom_speed: f32 = 1.0;
    let min_distance:f32 = 5.0;
    let max_distance:f32 = 35.0;


    for scroll in evr_scroll.read() {
        camera_state.distance -= scroll.y * zoom_speed;
        camera_state.distance = camera_state.distance.clamp(min_distance, max_distance);
    }

    for mut transform in query.iter_mut() {
        transform.translation = transform.translation.normalize() * camera_state.distance;
    }


}


