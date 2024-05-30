use bevy::app::Plugin;
use bevy::math::bounding::Aabb3d;

use crate::exfil::ExfilArea;
use crate::raid::Volume;
use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Plugin
pub struct FakeLevelPlugin;

impl Plugin for FakeLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_fake_level)
            .add_systems(Update, (update_fake_level).run_if(in_state(AppState::Raid)))
            .add_systems(OnExit(AppState::Raid), bye_fake_level);
    }
}

// Components
#[derive(Component)]
struct FakeLevelStuff;

// Resources

// Events
fn start_fake_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    debug!("starting fake level");
    // circular base
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Circle::new(8.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..Default::default()
            }),
            transform: Transform::from_rotation(Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            )),
            ..default()
        })
        .insert(Name::new("Disc"))
        .insert(FakeLevelStuff);
    // cube
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::GOLD,
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
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
        .insert(ExfilArea(String::from("Exfil1")))
        .insert(Name::new("Cuboid"))
        .insert(FakeLevelStuff);
    // light
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("PointyLight"))
        .insert(FakeLevelStuff);
}

fn update_fake_level() {
    // TODO: maybe just render them near any cameras
    // TODO: maybe put code here that moves the scene near cameras to maintain a reference for
    // movement
    debug!("updating fake level");
}
fn bye_fake_level(mut commands: Commands, query: Query<Entity, With<FakeLevelStuff>>) {
    debug!("stopping fake level");
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
