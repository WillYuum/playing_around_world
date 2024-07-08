use bevy::{ input::{keyboard::Key, mouse::{MouseButtonInput, MouseMotion}}, prelude::*};



#[derive(Resource)]
pub struct CameraState {
    pub rotating: bool,
    pub last_cursor_position: Vec2,
}

impl Default for CameraState {
    fn default() -> Self {
        CameraState {
            rotating: false,
            last_cursor_position: Vec2::ZERO,
        }
    }
}



pub fn rotate_camera(
    time: Res<Time>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut camera_state: ResMut<CameraState>,
    mut query: Query<&mut Transform, With<Camera>>
){
    let rotation_speed: f32 = std::f32::consts::PI / 2.0; // Rotate 90 degrees per second


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
                let delta = mouse_motion.delta - camera_state.last_cursor_position;
                println!("Updating Transform...");
                transform.rotate_around(
                    Vec3::ZERO,
                    Quat::from_rotation_y(-delta.x * rotation_speed * time.delta_seconds()),
                );
            }
            camera_state.last_cursor_position = mouse_motion.delta;
        };
    }
}


