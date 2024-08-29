use bevy::app::Plugin;

use crate::exfil::Operator;
use crate::raid::Volume;
use crate::AppState;
use crate::AppState::Raid;
use bevy::{math::bounding::Aabb3d, prelude::*};

// Constants
const NAME: &str = "first person controller";

// Plugin
pub struct FirstPersonControllerPlugin;

impl Plugin for FirstPersonControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_first_person_controller_system)
            .add_systems(
                Update,
                (update_first_person_controller_system).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_first_person_controller_system);
    }
}

// Components

#[derive(Component)]
pub struct FirstPersonCamera;

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
    let camera = commands
        .spawn(FirstPersonCamera)
        //.insert(FreeLookCamera)
        .insert(Name::new("FirstPersonCamera"))
        .insert(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.75, -0.3)
                .looking_at(Vec3::new(0.0, 1.75, -1.0), Vec3::Y),
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

    let transform = Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands
        .spawn(Operator)
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
fn update_first_person_controller_system() {
    debug!("updating {}", NAME);
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
