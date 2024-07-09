
use bevy::{ input::{keyboard::Key, mouse::{MouseButtonInput, MouseMotion}}, math::VectorSpace, prelude::*};

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
                    let mouseMotionDelta = mouse_motion.delta.x;
                    println!("Updating Transform...");
                    transform.rotate_around(
                        Vec3::ZERO,
                        Quat::from_rotation_y(-mouseMotionDelta * rotation_speed * time.delta_seconds()),
                    );
                };
        }
    }   
}


