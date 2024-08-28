use std::f32::consts::PI;

use bevy::app::Plugin;
use bevy::input::mouse::MouseMotion;

use crate::raid::FreeLookCamera;
use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "camera look";
const LOOK_SPEED: f32 = 1.0 / 360.0;

// Plugin
pub struct CameraLookPlugin;

impl Plugin for CameraLookPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_camera_look)
            .add_systems(
                Update,
                (update_camera_look).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_camera_look);
    }
}

// Components

// Resources

// Events

// Systems

fn start_camera_look(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn update_camera_look(
    mut mouse_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<FreeLookCamera>>,
) {
    debug!("updating {}", NAME);
    if let Ok(mut transform) = query.get_single_mut() {
        let mut mouse_delta = Vec2::ZERO;

        for mouse_event in mouse_events.read() {
            mouse_delta += mouse_event.delta;
        }

        // TODO: check for side effects for other systems that read mouse events
        mouse_events.clear();

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let new_pitch = (pitch - mouse_delta.y * LOOK_SPEED).clamp(-PI / 2., PI / 2.);
        let new_yaw = yaw - mouse_delta.x * LOOK_SPEED;
        let new_roll = roll;
        transform.rotation = Quat::from_euler(EulerRot::YXZ, new_yaw, new_pitch, new_roll);
    }
}

fn bye_camera_look(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}
