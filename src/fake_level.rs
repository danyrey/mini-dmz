use crate::backpack_summary::BackpackSummary;
use crate::contracts::{ContractId, ContractPhone, ContractType};
use crate::coordinates::{GridOffset, GridScale};
// TODO: how to make sure every operator has a backpack attached to it
//  TODO: transfer from the active loadout screen should be done
//  * transfer from state from one appstate to another: active dute layout -> ...load in -> raid
use crate::damage::HurtBox;
use crate::exfil::{ExfilArea, Operator};
use crate::first_person_controller::PlayerControlled;
use crate::flee::Ghost;
use crate::follow::Zombie;
use crate::interaction::Interactable;
use crate::inventory::{Inventory, ItemSlot, ItemSlots, WeaponSlot, WeaponSlots};
use crate::lock::{Key, Lock};
use crate::loot::{
    Durability, ItemType, Loot, LootCacheState, LootName, LootType, Price, Rarity, Stackable,
};
use crate::projectile::{Projectile, ProjectileEmitter, ProjectileTime, ProjectileVelocity};
use crate::raid::Enemy;
use crate::spawn::{Formation, Spawn, SpawnId, SpawnPosition};
use crate::squad::SquadId;
use crate::wallet::Money;
use crate::AppState;
use crate::AppState::Raid;
use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::math::Affine2;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

// Plugin
pub struct FakeLevelPlugin;

impl Plugin for FakeLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), (start_fake_level_ui, start_fake_level))
            .add_systems(
                Update,
                (
                    update_fake_level,
                    add_backpack_summary,
                    add_weapon_to_operators,
                    add_inventory_to_operators,
                    add_cubes_to_projectiles,
                    add_squad_id_to_my_operator,
                    manage_cursor,
                )
                    .run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_fake_level);
    }
}

// Components
#[derive(Component)]
struct FakeLevelStuff;

#[derive(Component)]
pub struct Crosshair;

// Resources
#[derive(Resource)]
struct PrototypeTextures {
    #[allow(dead_code)]
    texture_01: Handle<Image>,
    #[allow(dead_code)]
    texture_06: Handle<Image>,
}

// Events

// Systems
pub fn start_fake_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    debug!("starting fake level");

    let texture_01 =
        asset_server.load_with_settings("textures/prototype/Dark/texture_01.png", |s: &mut _| {
            *s = ImageLoaderSettings {
                sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                    // rewriting mode to repeat image,
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    ..default()
                }),
                ..default()
            }
        });
    let texture_06 =
        asset_server.load_with_settings("textures/prototype/Light/texture_06.png", |s: &mut _| {
            *s = ImageLoaderSettings {
                sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                    // rewriting mode to repeat image,
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    ..default()
                }),
                ..default()
            }
        });

    commands.insert_resource(PrototypeTextures {
        texture_01: texture_01.clone(),
        texture_06: texture_06.clone(),
    });

    commands.insert_resource(GridOffset(Vec2 { x: 1.0, y: 1.0 }));
    commands.insert_resource(GridScale(0.1));

    // circular base
    let disc_size = 8.0;

    // spawn
    commands
        .spawn(Spawn {
            formation: Formation::Staggered,
        })
        .insert(Name::new("Spawn1"))
        .insert(SpawnId(1))
        .insert(SquadId(111))
        .insert(Transform::from_xyz(-4.0, 0.0, 5.0));

    // TODO: should this be a child of spawn or no?
    // spawn position 1 / 1
    commands
        .spawn(SpawnPosition)
        .insert(Name::new("Spawn1/1"))
        .insert(SpawnId(1))
        .insert(SquadId(111))
        .insert(Transform::from_xyz(-1.0, 0.0, 5.0));

    // spawn position 1 / 2
    commands
        .spawn(SpawnPosition)
        .insert(Name::new("Spawn1/2"))
        .insert(SpawnId(1))
        .insert(SquadId(111))
        .insert(Transform::from_xyz(-3.0, 0.0, 5.0));

    // spawn position 1 / 3
    commands
        .spawn(SpawnPosition)
        .insert(Name::new("Spawn1/3"))
        .insert(SpawnId(1))
        .insert(SquadId(111))
        .insert(Transform::from_xyz(-5.0, 0.0, 5.0));

    commands
        .spawn(Spawn {
            formation: Formation::Staggered,
        })
        .insert(Name::new("Spawn2"))
        .insert(SpawnId(2))
        .insert(SquadId(222))
        .insert(Transform::from_xyz(4.0, 0.0, 3.0));

    // spawn position 2 / 1
    commands
        .spawn(SpawnPosition)
        .insert(Name::new("Spawn2/1"))
        .insert(SpawnId(2))
        .insert(SquadId(222))
        .insert(Transform::from_xyz(1.0, 0.0, 3.0));

    // spawn position 2 / 2
    commands
        .spawn(SpawnPosition)
        .insert(Name::new("Spawn2/2"))
        .insert(SpawnId(2))
        .insert(SquadId(222))
        .insert(Transform::from_xyz(3.0, 0.0, 3.0));

    // spawn position 2 / 3
    commands
        .spawn(SpawnPosition)
        .insert(Name::new("Spawn2/3"))
        .insert(SpawnId(2))
        .insert(SquadId(222))
        .insert(Transform::from_xyz(5.0, 0.0, 3.0));

    commands
        .spawn((
            Mesh3d(meshes.add(Circle::new(disc_size))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture_06.clone()),
                base_color: Color::WHITE,
                uv_transform: Affine2::from_scale(Vec2::new(2. * disc_size, 2. * disc_size)),
                ..Default::default()
            })),
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ))
        .insert(Name::new("Disc"))
        .insert(FakeLevelStuff);
    // cube 1
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture_06.clone()),
                base_color: Color::srgb(1.0, 1.0, 0.0),
                ..Default::default()
            })),
            Transform::from_xyz(2.0, 0.5, 2.0),
        ))
        .insert(ExfilArea(String::from("Exfil1")))
        .insert(Name::new("Exfil1"))
        .insert(FakeLevelStuff);
    // cube 2
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture_06.clone()),
                base_color: Color::srgb(1.0, 1.0, 0.0),
                ..Default::default()
            })),
            Transform::from_xyz(4.0, 0.5, 5.0),
        ))
        .insert(ExfilArea(String::from("Exfil2")))
        .insert(Name::new("Exfil2"))
        .insert(HurtBox(bevy::math::bounding::Aabb3d {
            min: Vec3 {
                x: 4.0,
                y: 0.5,
                z: 5.0,
            }
            .into(),
            max: Vec3 {
                x: 5.0,
                y: 0.5,
                z: 6.0,
            }
            .into(),
        }))
        .insert(FakeLevelStuff);
    // enemy cube 1

    let enemy_cube_size_x = 0.5;
    let enemy_cube_size_y = 2.0;
    let enemy_cube = meshes.add(Cuboid::new(
        enemy_cube_size_x,
        enemy_cube_size_y,
        enemy_cube_size_x,
    ));
    //let enemy_mesh = meshes.get_mut(&enemy_cube).unwrap();
    //scale_uv(enemy_mesh, enemy_cube_size_x, enemy_cube_size_y);

    commands
        .spawn((
            Mesh3d(enemy_cube),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture_06.clone()),
                base_color: Color::srgb(1.0, 1.0, 1.0),
                uv_transform: Affine2::from_scale(Vec2::new(enemy_cube_size_x, enemy_cube_size_y)),
                ..Default::default()
            })),
            Transform::from_xyz(5.0, 1.0, 5.0),
        ))
        .insert(Enemy)
        .insert(Name::new("Enemy1"))
        .insert(Ghost)
        .insert(HurtBox(bevy::math::bounding::Aabb3d {
            min: Vec3 {
                x: 4.75,
                y: 0.0,
                z: 4.75,
            }
            .into(),
            max: Vec3 {
                x: 5.25,
                y: 2.0,
                z: 5.25,
            }
            .into(),
        }))
        .insert(FakeLevelStuff);
    // enemy 2 capsule
    let capsule_height = 1.50;
    let capsule_radius = 0.25;

    commands
        .spawn((
            Mesh3d(meshes.add(Capsule3d::new(capsule_radius, capsule_height))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture_06.clone()),
                base_color: Color::srgb(0.75, 0.0, 0.0),
                uv_transform: Affine2::from_scale(Vec2::new(
                    6.0 * capsule_radius,
                    capsule_height + (2.0 * capsule_radius),
                )),
                ..Default::default()
            })),
            Transform::from_xyz(3.5, 1.0, 5.0).with_scale(Vec3::new(1.0, 1.0, 0.5)),
        ))
        .insert(Enemy)
        .insert(Name::new("Enemy2"))
        .insert(Zombie)
        .insert(FakeLevelStuff);

    commands
        .spawn((
            Mesh3d(meshes.add(Capsule3d::new(capsule_radius, capsule_height))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture_06.clone()),
                base_color: Color::srgb(0.0, 0.75, 0.0),
                uv_transform: Affine2::from_scale(Vec2::new(
                    6.0 * capsule_radius,
                    capsule_height + (2.0 * capsule_radius),
                )),
                ..Default::default()
            })),
            Transform::from_xyz(-1.0, 1.0, 4.0).with_scale(Vec3::new(1.0, 1.0, 0.5)),
        ))
        .insert(Operator)
        .insert(SquadId(111))
        .insert(Name::new("Squadmate1"))
        .insert(FakeLevelStuff);

    commands
        .spawn((
            Mesh3d(meshes.add(Capsule3d::new(capsule_radius, capsule_height))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture_06.clone()),
                base_color: Color::srgb(0.0, 0.75, 0.0),
                uv_transform: Affine2::from_scale(Vec2::new(
                    6.0 * capsule_radius,
                    capsule_height + (2.0 * capsule_radius),
                )),
                ..Default::default()
            })),
            Transform::from_xyz(-3.0, 1.0, 4.0).with_scale(Vec3::new(1.0, 1.0, 0.5)),
        ))
        .insert(Operator)
        .insert(SquadId(111))
        .insert(Name::new("Squadmate2"))
        .insert(FakeLevelStuff);

    commands
        .spawn((
            Mesh3d(meshes.add(Capsule3d::new(capsule_radius, capsule_height))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture_06.clone()),
                base_color: Color::srgb(0.75, 0.75, 0.75),
                uv_transform: Affine2::from_scale(Vec2::new(
                    6.0 * capsule_radius,
                    capsule_height + (2.0 * capsule_radius),
                )),
                ..Default::default()
            })),
            Transform::from_xyz(-1.0, 1.0, 4.0).with_scale(Vec3::new(1.0, 1.0, 0.5)),
        ))
        .insert(Operator)
        .insert(SquadId(222))
        .insert(Name::new("Enemy Squadmember 1"))
        .insert(FakeLevelStuff);

    commands
        .spawn((
            Mesh3d(meshes.add(Capsule3d::new(capsule_radius, capsule_height))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture_06.clone()),
                base_color: Color::srgb(0.75, 0.75, 0.75),
                uv_transform: Affine2::from_scale(Vec2::new(
                    6.0 * capsule_radius,
                    capsule_height + (2.0 * capsule_radius),
                )),
                ..Default::default()
            })),
            Transform::from_xyz(-3.0, 1.0, 4.0).with_scale(Vec3::new(1.0, 1.0, 0.5)),
        ))
        .insert(Operator)
        .insert(SquadId(222))
        .insert(Name::new("Enemy Squadmember 2"))
        .insert(FakeLevelStuff);

    commands
        .spawn((
            Mesh3d(meshes.add(Capsule3d::new(capsule_radius, capsule_height))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture_06.clone()),
                base_color: Color::srgb(0.75, 0.75, 0.75),
                uv_transform: Affine2::from_scale(Vec2::new(
                    6.0 * capsule_radius,
                    capsule_height + (2.0 * capsule_radius),
                )),
                ..Default::default()
            })),
            Transform::from_xyz(5.0, 1.0, 4.0).with_scale(Vec3::new(1.0, 1.0, 0.5)),
        ))
        .insert(Operator)
        .insert(SquadId(222))
        .insert(Name::new("Enemy Squadmember 3"))
        .insert(FakeLevelStuff);

    // loot cube 1

    let loot_cube_size = 0.2;
    let loot_cube = meshes.add(Cuboid::new(loot_cube_size, loot_cube_size, loot_cube_size));

    commands
        .spawn((
            Mesh3d(loot_cube.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture_06.clone()),
                base_color: Color::srgb(0.5, 0.75, 0.0),
                uv_transform: Affine2::from_scale(Vec2::new(loot_cube_size, loot_cube_size)),
                ..Default::default()
            })),
            Transform::from_xyz(6.0, 1.1, -2.0),
        ))
        .insert(Name::new("Toolbox Key"))
        .insert(Loot)
        .insert(Interactable)
        .insert(LootName(String::from("Toolbox Key")))
        .insert(Key::RegularKey(crate::lock::RegularKey { code: 123 }))
        .insert(LootType::Key);

    commands
        .spawn((
            Mesh3d(loot_cube.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture_06.clone()),
                base_color: Color::srgb(0.0, 1.0, 0.0),
                uv_transform: Affine2::from_scale(Vec2::new(loot_cube_size, loot_cube_size)),
                ..Default::default()
            })),
            Transform::from_xyz(5.0, 1.1, -2.0),
        ))
        .insert(Name::new("Loot1"))
        .insert(Loot)
        .insert(Interactable)
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
        .spawn((
            Mesh3d(loot_cube.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 1.0, 0.0),
                base_color_texture: Some(texture_06.clone()),
                uv_transform: Affine2::from_scale(Vec2::new(loot_cube_size, loot_cube_size)),
                ..Default::default()
            })),
            Transform::from_xyz(4.0, 1.1, -2.0),
        ))
        .insert(Name::new("Loot2"))
        .insert(Loot)
        .insert(Interactable)
        .insert(LootName(String::from("Durable Gaskmask")))
        .insert(LootType::CombatDefense)
        .insert(Rarity::Rare)
        .insert(FakeLevelStuff)
        .insert(Durability {
            max: 100,
            current: 99,
        });

    // loot cube 3
    commands
        .spawn((
            Mesh3d(loot_cube.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 0.75, 0.0),
                base_color_texture: Some(texture_06.clone()),
                uv_transform: Affine2::from_scale(Vec2::new(loot_cube_size, loot_cube_size)),
                ..Default::default()
            })),
            Transform::from_xyz(3.0, 0.1, -2.0),
        ))
        .insert(Name::new("Loot3"))
        .insert(Loot)
        .insert(Interactable)
        .insert(LootName(String::from("P890")))
        .insert(LootType::Weapon)
        .insert(FakeLevelStuff);

    // loot cube 4
    commands
        .spawn((
            Mesh3d(loot_cube.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 1.0, 0.0),
                base_color_texture: Some(texture_06.clone()),
                uv_transform: Affine2::from_scale(Vec2::new(loot_cube_size, loot_cube_size)),
                ..Default::default()
            })),
            Transform::from_xyz(2.0, 0.1, -2.0),
        ))
        .insert(Name::new("Loot4"))
        .insert(LootName(String::from("Harddrive")))
        .insert(LootType::Item(ItemType::Item))
        .insert(Loot)
        .insert(Interactable)
        .insert(Stackable {
            max_stack: 5,
            current_stack: 1,
        })
        .insert(FakeLevelStuff);

    // sell station
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(0.3, 0.6, 0.5))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.5, 0.5, 1.0),
                base_color_texture: Some(texture_06.clone()),
                ..Default::default()
            })),
            Transform::from_xyz(-4.0, 0.3, 2.0),
        ))
        .insert(Name::new("Sellstation"))
        .insert(Interactable)
        .insert(FakeLevelStuff);

    // loot cache 1
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(0.3, 0.3, 0.5))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 0.0, 0.75),
                base_color_texture: Some(texture_06.clone()),
                ..Default::default()
            })),
            Transform::from_xyz(-4.0, 0.15, 0.0),
        ))
        .insert(Name::new("Toolbox"))
        .insert(Inventory)
        .insert(Interactable)
        .insert(ItemSlots(4))
        .insert(FakeLevelStuff);

    // loot cache 2
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(0.4, 2.0, 1.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 0.0, 0.75),
                base_color_texture: Some(texture_06.clone()),
                ..Default::default()
            })),
            Transform::from_xyz(-3.0, 1.0, 0.0),
        ))
        .insert(Name::new("Weapon Locker"))
        .insert(Inventory)
        .insert(Interactable)
        .insert(WeaponSlots(2))
        .insert(ItemSlots(6))
        .insert(FakeLevelStuff)
        .with_children(|parent| {
            parent
                .spawn((
                    Mesh3d(loot_cube.clone()),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::srgb(0.0, 1.0, 1.0),
                        base_color_texture: Some(texture_06.clone()),
                        uv_transform: Affine2::from_scale(Vec2::new(
                            loot_cube_size,
                            loot_cube_size,
                        )),
                        ..Default::default()
                    })),
                    Visibility::Visible,
                ))
                .insert(Name::new("WeaponLockerLoot1"))
                .insert(Loot)
                .insert(Interactable)
                .insert(LootName(String::from("M4")))
                .insert(WeaponSlot(0))
                .insert(LootType::Weapon)
                .insert(FakeLevelStuff);
            parent
                .spawn((
                    Mesh3d(loot_cube.clone()),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(texture_06.clone()),
                        base_color: Color::srgb(0.0, 1.0, 0.0),
                        uv_transform: Affine2::from_scale(Vec2::new(
                            loot_cube_size,
                            loot_cube_size,
                        )),
                        ..Default::default()
                    })),
                    Visibility::Visible,
                ))
                .insert(Name::new("WeaponLockerLoot2"))
                .insert(Loot)
                .insert(Interactable)
                .insert(LootName(String::from("Wrench")))
                .insert(LootType::Item(ItemType::Item))
                .insert(Price(100))
                .insert(ItemSlot(3))
                .insert(Stackable {
                    max_stack: 3,
                    current_stack: 2,
                })
                .insert(FakeLevelStuff);
            parent
                .spawn((
                    Mesh3d(loot_cube.clone()),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::srgb(0.0, 1.0, 0.0),
                        base_color_texture: Some(texture_06.clone()),
                        uv_transform: Affine2::from_scale(Vec2::new(
                            loot_cube_size,
                            loot_cube_size,
                        )),
                        ..Default::default()
                    })),
                ))
                .insert(Name::new("Durable Gasmask"))
                .insert(Loot)
                .insert(Interactable)
                .insert(LootName(String::from("Durable Gaskmask")))
                .insert(LootType::CombatDefense)
                .insert(ItemSlot(4))
                .insert(Rarity::Rare)
                .insert(FakeLevelStuff)
                .insert(Durability::default());
        });

    // loot cache locked
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(0.3, 0.3, 0.5))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.75, 0.0, 0.0),
                base_color_texture: Some(texture_06.clone()),
                ..Default::default()
            })),
            Transform::from_xyz(-2.0, 0.15, 0.0),
        ))
        .insert(Name::new("Toolbox Locked"))
        .insert(Inventory)
        .insert(LootCacheState::Locked)
        .insert(Lock { code: 123 })
        .insert(Interactable)
        .insert(ItemSlots(4))
        .insert(FakeLevelStuff);

    // light
    commands
        .spawn(PointLight {
            shadows_enabled: true,
            ..default()
        })
        .insert(Transform::from_xyz(4.0, 8.0, 4.0))
        .insert(Name::new("PointyLight"))
        .insert(FakeLevelStuff);

    // money
    commands
        .spawn((
            Mesh3d(loot_cube.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 1.0, 0.0),
                base_color_texture: Some(texture_06.clone()),
                uv_transform: Affine2::from_scale(Vec2::new(loot_cube_size, loot_cube_size)),
                ..Default::default()
            })),
            Transform::from_xyz(5.0, 1.1, -3.0),
        ))
        .insert(Name::new("Dineros"))
        .insert(Loot)
        .insert(Interactable)
        .insert(LootName(String::from("Dineros")))
        .insert(LootType::Cash)
        .insert(Money)
        .insert(Price(100))
        .insert(FakeLevelStuff);

    // contract phone
    // TODO: setup resources and link it to this contract phone
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(0.05, 0.15, 0.01))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 1.0, 0.0),
                base_color_texture: Some(texture_06.clone()),
                ..Default::default()
            })),
            Transform::from_xyz(16.0, 1.35, 2.0),
        ))
        .insert(Name::new("ContractPhone"))
        .insert(Interactable)
        .insert(ContractPhone)
        .insert(ContractId(123))
        .insert(ContractType::SecureSupplies)
        .insert(FakeLevelStuff);

    // supply contract loot cache 1
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 0.5, 0.5))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 0.0, 0.75),
                base_color_texture: Some(texture_06.clone()),
                ..Default::default()
            })),
            Transform::from_xyz(16.0, 0.25, 10.0),
        ))
        .insert(Name::new("SupplyContractBox1"))
        .insert(Inventory)
        .insert(LootCacheState::Locked)
        .insert(ItemSlots(4))
        .insert(Interactable)
        .insert(ContractId(123))
        .insert(FakeLevelStuff)
        .with_children(|parent| {
            parent
                .spawn((
                    Mesh3d(loot_cube.clone()),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::srgb(0.0, 1.0, 0.0),
                        base_color_texture: Some(texture_06.clone()),
                        uv_transform: Affine2::from_scale(Vec2::new(
                            loot_cube_size,
                            loot_cube_size,
                        )),
                        ..Default::default()
                    })),
                ))
                .insert(ItemSlot(0))
                .insert(Name::new("Durable Gasmask"))
                .insert(Loot)
                .insert(Interactable)
                .insert(LootName(String::from("Durable Gaskmask")))
                .insert(LootType::CombatDefense)
                .insert(Rarity::Rare)
                .insert(FakeLevelStuff)
                .insert(Durability::default());
        });

    // supply contract loot cache 2
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 0.5, 0.5))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 0.0, 0.75),
                base_color_texture: Some(texture_06.clone()),
                ..Default::default()
            })),
            Transform::from_xyz(18.0, 0.25, 10.0),
        ))
        .insert(Name::new("SupplyContractBox2"))
        .insert(Inventory)
        .insert(LootCacheState::Locked)
        .insert(Interactable)
        .insert(ContractId(123))
        .insert(ItemSlots(4))
        .insert(FakeLevelStuff);

    // supply contract loot cache 3
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 0.5, 0.5))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 0.0, 0.75),
                base_color_texture: Some(texture_06.clone()),
                ..Default::default()
            })),
            Transform::from_xyz(20.0, 0.25, 10.0),
        ))
        .insert(Name::new("SupplyContractBox3"))
        .insert(Inventory)
        .insert(LootCacheState::Locked)
        .insert(Interactable)
        .insert(ContractId(123))
        .insert(ItemSlots(4))
        .insert(FakeLevelStuff);

    // example projectile
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(0.2, 0.2, 0.2))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 1.0, 1.0),
                ..Default::default()
            })),
            Transform::from_xyz(-5.0, 1.0, 4.0),
        ))
        .insert(Name::new("Bullet"))
        .insert(Projectile::default())
        .insert(ProjectileVelocity::default())
        .insert(ProjectileTime::default())
        .insert(FakeLevelStuff);
}

fn start_fake_level_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            ..default()
        })
        .insert(Transform::from_xyz(0.0, 0.0, 10.0))
        .insert(Visibility::Hidden)
        .insert(Name::new("Crosshair POC"))
        .insert(FakeLevelStuff)
        .insert(Crosshair)
        .with_children(|parent| {
            parent.spawn(Sprite {
                image: asset_server.load("textures/crosshair.png"),
                custom_size: Some(Vec2 { x: 18.0, y: 18.0 }),
                ..default()
            });
        });
}

fn add_squad_id_to_my_operator(
    mut commands: Commands,
    query: Query<Entity, (With<PlayerControlled>, Added<Operator>)>,
) {
    for added in query.iter() {
        commands.entity(added).insert(SquadId(111));
    }
}

fn add_backpack_summary(mut commands: Commands, query: Query<Entity, Added<Operator>>) {
    for added in query.iter() {
        commands.entity(added).insert(BackpackSummary::default());
    }
}

fn add_weapon_to_operators(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, Added<Operator>>,
) {
    debug!("adding weapon to new operators");
    for added in query.iter() {
        commands
            .spawn((
                Mesh3d(meshes.add(Cuboid::new(0.125, 0.125, 1.0))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.75, 0.0, 0.0),
                    ..Default::default()
                })),
                Transform::from_xyz(0.0, 1.0, -0.5),
            ))
            .insert(Name::new("Weapon"))
            .insert(ProjectileEmitter::default())
            .insert(FakeLevelStuff)
            .set_parent(added);
    }
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
            .spawn((
                Mesh3d(meshes.add(Cuboid::new(0.4, 0.5, 0.25))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.0, 0.75, 0.0),
                    ..Default::default()
                })),
                Transform::from_xyz(0.0, 1.5, 0.25),
            ))
            .insert(Name::new("Large Backpack"))
            .insert(Inventory)
            .insert(ItemSlots(9))
            .insert(WeaponSlots(2))
            .insert(FakeLevelStuff)
            .set_parent(added);
    }
}

fn add_cubes_to_projectiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, Added<Projectile>>,
) {
    debug!("adding cubes to new projectiles");
    for added in query.iter() {
        commands
            .spawn((
                Mesh3d(meshes.add(Cuboid::new(0.2, 0.2, 0.2))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(1.0, 0.0, 0.0),
                    ..Default::default()
                })),
                Transform::from_xyz(0.0, 0.0, 0.0),
                FakeLevelStuff,
                Name::new("BulletCube"),
            ))
            .set_parent(added);
    }
}

// renders some fake level exclusive gizmos
fn update_fake_level(
    mut gizmos: Gizmos,
    images: ResMut<Assets<Image>>,
    query: Query<&GlobalTransform, With<Enemy>>,
    //loot_query: Query<&GlobalTransform, With<Loot>>,
) {
    debug!("updating fake level");
    images.iter().for_each(|i| {
        debug!("image: {:?}", i.1.size());
    });
    for global_transform in query.iter() {
        gizmos.ray(
            global_transform.to_scale_rotation_translation().2 + Vec3::new(0.0, 0.75, 0.0),
            (global_transform.to_scale_rotation_translation().1 * Vec3::Z).xyz() * -1.0,
            Color::srgb(1.0, 0.0, 0.0),
        );
    }
    // renders correctly at the cubes, but ...
    // FIXME: produces strange artifacts at the center of the screen
    // seems like the gizmos are rendered again at the far end of frustum
    /*
    for global_transform in loot_query.iter() {
        gizmos.ray(
            global_transform.translation(),
            Vec3::new(0.3, 0.0, 0.0),
            Color::RED,
        );
        gizmos.ray(
            global_transform.translation(),
            Vec3::new(0.0, 0.3, 0.0),
            Color::GREEN,
        );
        gizmos.ray(
            global_transform.translation(),
            Vec3::new(0.0, 0.0, 0.3),
            Color::BLUE,
        );
    }
     */
}

/// system to toggle cursor behaviors
fn manage_cursor(
    mut commands: Commands,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    key_input: Res<ButtonInput<KeyCode>>,
    crosshair: Query<Entity, With<Crosshair>>,
) {
    let mut primary_window = windows.single_mut();
    let crosshair_vis = crosshair.single();

    if key_input.pressed(KeyCode::F9) {
        primary_window.cursor_options.visible = false;
        commands.entity(crosshair_vis).insert(Visibility::Visible);
    }

    if key_input.pressed(KeyCode::F10) {
        primary_window.cursor_options.visible = true;
        commands.entity(crosshair_vis).insert(Visibility::Hidden);
    }

    if key_input.pressed(KeyCode::F11) {
        primary_window.cursor_options.grab_mode = CursorGrabMode::Confined;
    }

    if key_input.pressed(KeyCode::F12) {
        primary_window.cursor_options.grab_mode = CursorGrabMode::None;
    }
}

fn bye_fake_level(mut commands: Commands, query: Query<Entity, With<FakeLevelStuff>>) {
    debug!("stopping fake level");
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
