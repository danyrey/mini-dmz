use std::f32::consts::PI;

use bevy::{app::Plugin, input::mouse::MouseMotion};

use crate::exfil::Operator;
use crate::raid::Volume;
use crate::AppState;
use crate::AppState::Raid;
use bevy::{math::bounding::Aabb3d, prelude::*};

// Constants
const NAME: &str = "first person controller";
const LOOK_SPEED: f32 = 1.0 / 360.0;

// Plugin
pub struct FirstPersonControllerPlugin;

impl Plugin for FirstPersonControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_first_person_controller_system)
            .add_systems(
                Update,
                (
                    update_camera_move,
                    update_camera_look_yaw,
                    update_camera_look_pitch,
                )
                    .run_if(in_state(AppState::Raid)),
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
fn start_first_person_controller_system(
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
                base_color: Color::GREEN,
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
        .insert(Name::new("Operator"))
        .insert(transform)
        .insert(GlobalTransform::from(transform))
        .insert(Volume(Aabb3d {
            min: Vec3 {
                x: -0.5,
                y: 0.0,
                z: -0.5,
            },
            max: Vec3 {
                x: 0.5,
                y: 1.0,
                z: 0.5,
            },
        }))
        .add_child(camera)
        .add_child(capsule);
}

fn update_camera_move(
    time: Res<Time>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, (With<PlayerControlled>, With<Operator>)>,
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

/// system for yaw only
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

        transform.rotate_y((-mouse_delta.x * LOOK_SPEED).clamp(-PI / 2., PI / 2.));
    }
}

/// system for pitch only
fn update_camera_look_pitch(
    mut mouse_events: EventReader<MouseMotion>,
    operator_query: Query<&Transform, (With<Operator>, With<PlayerControlled>)>,
    mut camera_query: Query<
        (&Parent, &mut Transform),
        (With<FirstPersonCamera>, Without<Operator>),
    >,
) {
    debug!("updating {}", NAME);
    if let Ok(mut transform) = camera_query.get_single_mut() {
        let mut mouse_delta = Vec2::ZERO;

        for mouse_event in mouse_events.read() {
            mouse_delta += mouse_event.delta;
        }

        // TODO: check for side effects for other systems that read mouse events
        mouse_events.clear();

        if operator_query.get(transform.0.get()).is_ok() {
            transform
                .1
                .rotate_x((-mouse_delta.y * LOOK_SPEED).clamp(-PI / 2., PI / 2.));
        }
    }
}

fn bye_first_person_controller_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

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
