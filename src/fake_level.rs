// TODO: how to make sure every operator has a backpack attached to it
//  TODO: transfer from the active loadout screen should be done
//  * transfer from state from one appstate to another: active dute layout -> ...load in -> raid
use crate::damage::HurtBox;
use crate::exfil::{ExfilArea, Operator};
use crate::inventory::{Inventory, ItemSlots, WeaponSlots};
use crate::loot::{Durability, ItemType, Loot, LootName, LootType, Price, Rarity, Stackable};
use crate::raid::{Enemy, FreeLookCamera};
use crate::AppState;
use crate::AppState::Raid;
use bevy::app::Plugin;
use bevy::prelude::*;

// Plugin
pub struct FakeLevelPlugin;

impl Plugin for FakeLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_fake_level)
            .add_systems(
                Update,
                (
                    update_fake_level,
                    add_inventory_to_operators,
                    //update_inventory_to_follow_camera,
                )
                    .run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_fake_level);
    }
}

// Components
#[derive(Component)]
struct FakeLevelStuff;

// Resources

// Events

// Systems
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
        .insert(Enemy)
        .insert(Name::new("Enemy1"))
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
    // enemy 2 capsule
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Capsule3d::new(0.25, 1.5)),
            material: materials.add(StandardMaterial {
                base_color: Color::ORANGE_RED,
                ..Default::default()
            }),
            transform: Transform::from_xyz(6.0, 1.0, 6.0).with_scale(Vec3::new(1.0, 1.0, 0.5)),
            ..default()
        })
        .insert(Enemy)
        .insert(Name::new("Enemy2"))
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
                base_color: Color::DARK_GREEN,
                ..Default::default()
            }),
            transform: Transform::from_xyz(3.0, 0.1, -2.0),
            ..default()
        })
        .insert(Name::new("Loot3"))
        .insert(Loot)
        .insert(LootName(String::from("M4")))
        .insert(LootType::Weapon)
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
        .insert(LootType::Item(ItemType::Item))
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

/// semi init procedure: add inventory cube to Operator entities.
// TODO: not sure if this is an idiomatic way to do post setup stuff but it works
fn add_inventory_to_operators(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, Added<Operator>>,
) {
    debug!("adding inventory to new operators");
    for added in query.iter() {
        debug!("found one added operator: {:?}", added);
        // backpack/inventory cube
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.4, 0.5, 0.25)),
                material: materials.add(StandardMaterial {
                    base_color: Color::OLIVE,
                    ..Default::default()
                }),
                transform: Transform::from_xyz(0.0, 1.5, 0.25),
                ..default()
            })
            .insert(Name::new("OperatorBackpack"))
            .insert(Inventory)
            .insert(ItemSlots(3))
            .insert(WeaponSlots(1))
            .insert(FakeLevelStuff)
            .set_parent(added);
    }
}

// renders some fake level exclusive gizmos
fn update_fake_level(mut gizmos: Gizmos, query: Query<&GlobalTransform, With<Enemy>>) {
    debug!("updating fake level");
    for global_transform in query.iter() {
        gizmos.ray(
            global_transform.to_scale_rotation_translation().2 + Vec3::new(0.0, 0.75, 0.0),
            (global_transform.to_scale_rotation_translation().1 * Vec3::Z).xyz() * -1.0,
            Color::RED,
        );
    }
}

fn _update_inventory_to_follow_camera(
    camera_query: Query<&Transform, With<FreeLookCamera>>,
    mut inventory_query: Query<&mut Transform, (With<Inventory>, Without<FreeLookCamera>)>,
) {
    let transform = camera_query.single();
    let offset = Vec3::new(1.0, -2.0, 4.0);
    let new_transform = transform.with_translation(transform.translation + offset);
    let mut inventory_transform = inventory_query.single_mut();
    inventory_transform.translation = new_transform.translation;
}

fn bye_fake_level(mut commands: Commands, query: Query<Entity, With<FakeLevelStuff>>) {
    debug!("stopping fake level");
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
