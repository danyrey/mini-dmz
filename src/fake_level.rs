use crate::damage::HurtBox;
use crate::exfil::ExfilArea;
use crate::inventory::{Inventory, ItemSlots};
use crate::loot::{Durability, ItemType, Loot, LootName, LootType, Price, Rarity, Stackable};
use crate::AppState;
use crate::AppState::Raid;
use bevy::app::Plugin;
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
    // cube 1
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
        .insert(ExfilArea(String::from("Exfil1")))
        .insert(Name::new("Exfil1"))
        .insert(FakeLevelStuff);
    // cube 2
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::GOLD,
                ..Default::default()
            }),
            transform: Transform::from_xyz(10.0, 0.5, 10.0),
            ..default()
        })
        .insert(ExfilArea(String::from("Exfil2")))
        .insert(Name::new("Exfil2"))
        .insert(HurtBox(bevy::math::bounding::Aabb3d {
            min: Vec3 {
                x: 10.0,
                y: 0.5,
                z: 10.0,
            },
            max: Vec3 {
                x: 11.0,
                y: 0.5,
                z: 11.0,
            },
        }))
        .insert(FakeLevelStuff);
    // enemy cube 1
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(0.5, 2.0, 0.5)),
            material: materials.add(StandardMaterial {
                base_color: Color::RED,
                ..Default::default()
            }),
            transform: Transform::from_xyz(5.0, 1.0, 5.0),
            ..default()
        })
        .insert(Name::new("Enemy"))
        .insert(HurtBox(bevy::math::bounding::Aabb3d {
            min: Vec3 {
                x: 4.75,
                y: 0.0,
                z: 4.75,
            },
            max: Vec3 {
                x: 5.25,
                y: 2.0,
                z: 5.25,
            },
        }))
        .insert(FakeLevelStuff);
    // inventory cube
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
            material: materials.add(StandardMaterial {
                base_color: Color::BLUE,
                ..Default::default()
            }),
            transform: Transform::from_xyz(5.0, 0.25, -4.0),
            ..default()
        })
        .insert(Name::new("Inventory1"))
        .insert(Inventory)
        .insert(ItemSlots(3))
        .insert(FakeLevelStuff);
    // loot cube 1
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(0.2, 0.2, 0.2)),
            material: materials.add(StandardMaterial {
                base_color: Color::GREEN,
                ..Default::default()
            }),
            transform: Transform::from_xyz(5.0, 0.1, -2.0),
            ..default()
        })
        .insert(Name::new("Loot1"))
        .insert(Loot)
        .insert(LootName(String::from("Wrench")))
        .insert(LootType::Item(ItemType::Item))
        .insert(Price(100))
        .insert(Stackable {
            max_stack: 3,
            current_stack: 1,
        })
        .insert(FakeLevelStuff);
    // loot cube 2
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(0.2, 0.2, 0.2)),
            material: materials.add(StandardMaterial {
                base_color: Color::GREEN,
                ..Default::default()
            }),
            transform: Transform::from_xyz(4.0, 0.1, -2.0),
            ..default()
        })
        .insert(Name::new("Loot2"))
        .insert(Loot)
        .insert(LootName(String::from("Durable Gaskmask")))
        .insert(LootType::CombatDefense)
        .insert(Rarity::Rare)
        .insert(FakeLevelStuff)
        .insert(Durability::default());
    // loot cube 3
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(0.2, 0.2, 0.2)),
            material: materials.add(StandardMaterial {
                base_color: Color::GREEN,
                ..Default::default()
            }),
            transform: Transform::from_xyz(3.0, 0.1, -2.0),
            ..default()
        })
        .insert(Name::new("Loot2"))
        .insert(Loot)
        .insert(FakeLevelStuff);
    // loot cube 4
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(0.2, 0.2, 0.2)),
            material: materials.add(StandardMaterial {
                base_color: Color::GREEN,
                ..Default::default()
            }),
            transform: Transform::from_xyz(2.0, 0.1, -2.0),
            ..default()
        })
        .insert(Name::new("Loot4"))
        .insert(Loot)
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
