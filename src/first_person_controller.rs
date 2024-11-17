use std::f32::consts::PI;

use bevy::{app::Plugin, input::mouse::MouseMotion};

use crate::compass::Compass;
use crate::heightmap::FlatEarth;
use crate::raid::{RaidState, Volume};
use crate::AppState;
use crate::AppState::Raid;
use crate::{exfil::Operator, heightmap::YProbe};
use bevy::{math::bounding::Aabb3d, prelude::*};

// Constants
const NAME: &str = "first person controller";
const LOOK_SPEED: f32 = 1.0 / 360.0;
const RUN_SPEED_FACTOR: f32 = 2.0;
const CROUCH_SPEED_FACTOR: f32 = 0.5;
const CROUCH_HEIGHT_OFFSET: f32 = -0.75;
const JUMP_HEIGHT_OFFSET: f32 = 0.5;
const PI_QUARTER: f32 = PI / 4.0;
const PI_HALF: f32 = PI / 2.0;

// Plugin
pub struct FirstPersonControllerPlugin;

impl Plugin for FirstPersonControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_first_person_controller_system)
            .add_systems(
                Update,
                (update_camera_move, debug_im_crosshair).run_if(in_state(AppState::Raid)),
            )
            .add_systems(
                Update,
                (update_camera_look_yaw, update_camera_look_pitch)
                    .run_if(in_state(AppState::Raid).and_then(in_state(RaidState::Raid))),
            )
            .add_systems(OnExit(AppState::Raid), bye_first_person_controller_system);
    }
}

// Components

#[derive(Component)]
pub struct FirstPersonCamera;

#[derive(Component)]
pub struct PlayerControlled;

// Resources

// Events

// Systems
pub fn start_first_person_controller_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    debug!("starting {}", NAME);
    // camera
    let operator_position = Vec3::new(0.0, 1.75, -0.3);
    let operator_looking_at = operator_position - Vec3::Z;
    let camera = commands
        .spawn(FirstPersonCamera)
        .insert(Name::new("FirstPersonCamera"))
        .insert(Camera3dBundle {
            transform: Transform::from_translation(operator_position)
                .looking_at(operator_looking_at, Vec3::Y),
            ..default()
        })
        .id();

    let capsule = commands
        .spawn(PbrBundle {
            mesh: meshes.add(Capsule3d::new(0.25, 1.5)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 1.0, 0.0),
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 1.0, 0.0).with_scale(Vec3::new(1.0, 1.0, 0.5)),
            ..default()
        })
        .id();

    let transform = Transform::from_translation(Vec3::ZERO).looking_at(-Vec3::Z, Vec3::Y);
    commands
        .spawn(Operator)
        .insert(PlayerControlled)
        .insert(Compass)
        .insert(Name::new("Operator"))
        .insert(transform)
        .insert(GlobalTransform::from(transform))
        .insert(Volume(Aabb3d {
            min: Vec3 {
                x: -0.5,
                y: 0.0,
                z: -0.5,
            }
            .into(),
            max: Vec3 {
                x: 0.5,
                y: 1.0,
                z: 0.5,
            }
            .into(),
        }))
        .add_child(camera)
        .add_child(capsule);
}

#[allow(clippy::type_complexity)]
fn update_camera_move(
    time: Res<Time>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, (With<PlayerControlled>, With<Operator>)>,
) {
    debug!("updating {}", NAME);
    let dt = time.delta_seconds();

    if let Ok(mut transform) = query.get_single_mut() {
        let mut axis_input = Vec3::ZERO;
        let mut speed_modifier = 1.0;
        let mut height_modifier = 0.0;

        // TODO : probe for y baseline, could be const Y for first draft

        if key_input.pressed(KeyCode::ShiftLeft) || key_input.pressed(KeyCode::ShiftRight) {
            speed_modifier = RUN_SPEED_FACTOR;
        }

        if key_input.pressed(KeyCode::ControlLeft) || key_input.pressed(KeyCode::ControlRight) {
            speed_modifier = CROUCH_SPEED_FACTOR;
            height_modifier = CROUCH_HEIGHT_OFFSET;
        }

        if key_input.pressed(KeyCode::Space) {
            height_modifier = JUMP_HEIGHT_OFFSET;
        }

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

        let speed = 3.0;
        let mut velocity = Vec3::ZERO;

        if axis_input != Vec3::ZERO {
            velocity = axis_input.normalize() * speed * speed_modifier;
        }

        let forward = *transform.forward();
        let right = *transform.right();
        let new_x = velocity.x * dt * right;
        let new_y = velocity.y * dt * Vec3::Y; // jump and crouch modifiers go here
        let new_z = velocity.z * dt * forward;
        transform.translation += new_x + new_y + new_z;
        transform.translation.y =
            FlatEarth::probe_y(transform.translation.x, transform.translation.z);
        //XZSineTerrain::probe_y(transform.translation.x, transform.translation.z);
        transform.translation.y += height_modifier;
    }
}

// TODO: 0.0 for now, could be querying a heightmap in the future
impl YProbe for FirstPersonControllerPlugin {
    fn probe_y(x: f32, z: f32) -> f32 {
        0.0 * x * z
    }
}

/// system for yaw only
#[allow(clippy::type_complexity)]
fn update_camera_look_yaw(
    mut mouse_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, (With<Operator>, With<PlayerControlled>)>,
) {
    debug!("updating {}", NAME);
    if let Ok(mut transform) = query.get_single_mut() {
        let mut mouse_delta = Vec2::ZERO;

        for mouse_event in mouse_events.read() {
            mouse_delta += mouse_event.delta;
        }

        // TODO: check for side effects for other systems that read mouse events
        mouse_events.clear();

        transform.rotate_y((-mouse_delta.x * LOOK_SPEED).clamp(-PI_HALF, PI_HALF));
    }
}

/// system for pitch only
#[allow(clippy::type_complexity)]
fn update_camera_look_pitch(
    mut mouse_events: EventReader<MouseMotion>,
    operator_query: Query<&Transform, (With<Operator>, With<PlayerControlled>)>,
    mut camera_query: Query<
        (&Parent, &mut Transform),
        (With<FirstPersonCamera>, Without<Operator>),
    >,
) {
    debug!("updating {}", NAME);
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        let mut mouse_delta = Vec2::ZERO;

        for mouse_event in mouse_events.read() {
            mouse_delta += mouse_event.delta;
        }

        // TODO: check for side effects for other systems that read mouse events
        mouse_events.clear();

        // check if cameras parent is the actual operator
        if operator_query.get(camera_transform.0.get()).is_ok() {
            // default orientation is y up and down the minus z axis == forward

            // delta value to apply to rotation
            let delta = -mouse_delta.y * LOOK_SPEED;

            camera_transform.1.rotate_local_x(delta);
            if camera_transform.1.rotation.to_axis_angle().1 > PI_HALF {
                // if overshoot just snap back to 90 degrees
                let x = camera_transform.1.rotation.to_axis_angle().0.x;
                camera_transform.1.rotation.x = x * PI_QUARTER;
                camera_transform.1.rotation.w = PI_QUARTER;
            }
        }
    }
}

fn debug_im_crosshair(
    mut _gizmos: Gizmos,
    _camera_query: Query<(&Parent, &mut GlobalTransform), With<FirstPersonCamera>>,
) {
    /*
        for transform in camera_query.iter() {
            gizmos.ray(
                transform.1.translation() + Vec3::new(-0.1, 0.0, -0.5),
                Vec3::new(0.2, 0.0, 0.0),
                Color::RED,
            );
            gizmos.ray(
                transform.1.translation() + Vec3::new(0.0, -0.1, -0.5),
                Vec3::new(0.0, 0.2, 0.0),
                Color::RED,
            );
        }
    */
}

fn bye_first_person_controller_system(
    mut commands: Commands,
    query: Query<Entity, With<PlayerControlled>>,
) {
    debug!("stopping {}", NAME);
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    #[test]
    fn should_test_something() {
        // given
        //let mut app = App::new();

        // when
        //app.add_event::<HealthDamageReceived>();
        //app.add_systems(Update, damage_received_listener);
        //let entity = app.borrow_mut().world.spawn(Health(100)).id();
        //app.borrow_mut().world.resource_mut::<Events<HealthDamageReceived>>().send(HealthDamageReceived { entity, damage: 10 });
        //app.update();

        // then
        //assert!(app.world.get::<Health>(entity).is_some());
        //assert_eq!(app.world.get::<Health>(entity).unwrap().0, 90);
    }
}
