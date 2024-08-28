use bevy::app::Plugin;

use crate::raid::FreeLookCamera;
use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "camera move";

// Plugin
pub struct CameraMovePlugin;

impl Plugin for CameraMovePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_camera_move)
            .add_systems(
                Update,
                (update_camera_move).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_camera_move);
    }
}

// Components

// Resources

// Events

// Systems

fn start_camera_move(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn update_camera_move(
    time: Res<Time>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<FreeLookCamera>>,
) {
    debug!("updating {}", NAME);
    let dt = time.delta_seconds();

    if let Ok(mut transform) = query.get_single_mut() {
        let mut axis_input = Vec3::ZERO;
        if key_input.pressed(KeyCode::KeyW) {
            axis_input.z += 1.0;
        }
        if key_input.pressed(KeyCode::KeyS) {
            axis_input.z -= 1.0;
        }
        if key_input.pressed(KeyCode::KeyD) {
            axis_input.x += 1.0;
        }
        if key_input.pressed(KeyCode::KeyA) {
            axis_input.x -= 1.0;
        }
        if key_input.pressed(KeyCode::KeyQ) {
            axis_input.y += 1.0;
        }
        if key_input.pressed(KeyCode::KeyE) {
            axis_input.y -= 1.0;
        }

        let max_speed = 3.0;
        let mut velocity = Vec3::ZERO;

        if axis_input != Vec3::ZERO {
            velocity = axis_input.normalize() * max_speed;
        }

        let forward = *transform.forward();
        let right = *transform.right();
        let new_x = velocity.x * dt * right;
        let new_y = velocity.y * dt * Vec3::Y;
        let new_z = velocity.z * dt * forward;
        transform.translation += new_x + new_y + new_z;
    }
}

fn bye_camera_move(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}
