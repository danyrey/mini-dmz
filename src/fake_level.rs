// TODO: how to make sure every operator has a backpack attached to it
//  TODO: transfer from the active loadout screen should be done
//  * transfer from state from one appstate to another: active dute layout -> ...load in -> raid
use crate::damage::HurtBox;
use crate::exfil::{ExfilArea, Operator};
use crate::interaction::Interactable;
use crate::inventory::{Inventory, ItemSlot, ItemSlots, WeaponSlot, WeaponSlots};
use crate::loot::{Durability, ItemType, Loot, LootName, LootType, Price, Rarity, Stackable};
use crate::raid::Enemy;
use crate::AppState;
use crate::AppState::Raid;
use bevy::math::Affine2;
use bevy::prelude::*;
use bevy::render::texture::{
    ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor,
};
use bevy::window::{CursorGrabMode, PrimaryWindow};

// Plugin
pub struct FakeLevelPlugin;

impl Plugin for FakeLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), (start_fake_level_ui, start_fake_level))
            .add_systems(
                Update,
                (update_fake_level, add_inventory_to_operators, manage_cursor)
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
fn start_fake_level(
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

    // circular base
    let disc_size = 8.0;

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Circle::new(disc_size)),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_06.clone()),
                base_color: Color::WHITE,
                uv_transform: Affine2::from_scale(Vec2::new(2. * disc_size, 2. * disc_size)),
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
                base_color_texture: Some(texture_06.clone()),
                base_color: Color::srgb(1.0, 1.0, 0.0),
                ..Default::default()
            }),
            transform: Transform::from_xyz(2.0, 0.5, 2.0),
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
                base_color_texture: Some(texture_06.clone()),
                base_color: Color::srgb(1.0, 1.0, 0.0),
                ..Default::default()
            }),
            transform: Transform::from_xyz(4.0, 0.5, 5.0),
            ..default()
        })
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
        .spawn(PbrBundle {
            mesh: enemy_cube,
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_06.clone()),
                base_color: Color::srgb(1.0, 0.0, 0.0),
                uv_transform: Affine2::from_scale(Vec2::new(enemy_cube_size_x, enemy_cube_size_y)),
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
        .spawn(PbrBundle {
            mesh: meshes.add(Capsule3d::new(capsule_radius, capsule_height)),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_06.clone()),
                base_color: Color::srgb(0.75, 0.0, 0.0),
                uv_transform: Affine2::from_scale(Vec2::new(
                    6.0 * capsule_radius,
                    capsule_height + (2.0 * capsule_radius),
                )),
                ..Default::default()
            }),
            transform: Transform::from_xyz(3.5, 1.0, 5.0).with_scale(Vec3::new(1.0, 1.0, 0.5)),
            ..default()
        })
        .insert(Enemy)
        .insert(Name::new("Enemy2"))
        .insert(FakeLevelStuff);
    // loot cube 1

    let loot_cube_size = 0.2;
    let loot_cube = meshes.add(Cuboid::new(loot_cube_size, loot_cube_size, loot_cube_size));

    commands
        .spawn(PbrBundle {
            mesh: loot_cube.clone(),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_06.clone()),
                base_color: Color::srgb(0.0, 1.0, 0.0),
                uv_transform: Affine2::from_scale(Vec2::new(loot_cube_size, loot_cube_size)),
                ..Default::default()
            }),
            transform: Transform::from_xyz(5.0, 1.1, -2.0),
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
            mesh: loot_cube.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 1.0, 0.0),
                base_color_texture: Some(texture_06.clone()),
                uv_transform: Affine2::from_scale(Vec2::new(loot_cube_size, loot_cube_size)),
                ..Default::default()
            }),
            transform: Transform::from_xyz(4.0, 1.1, -2.0),
            ..default()
        })
        .insert(Name::new("Loot2"))
        .insert(LootName(String::from("Batteries")))
        .insert(Loot)
        .insert(LootName(String::from("Durable Gaskmask")))
        .insert(LootType::CombatDefense)
        .insert(Rarity::Rare)
        .insert(FakeLevelStuff)
        .insert(Durability::default());
    // loot cube 3
    commands
        .spawn(PbrBundle {
            mesh: loot_cube.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 0.75, 0.0),
                base_color_texture: Some(texture_06.clone()),
                uv_transform: Affine2::from_scale(Vec2::new(loot_cube_size, loot_cube_size)),
                ..Default::default()
            }),
            transform: Transform::from_xyz(3.0, 0.1, -2.0),
            ..default()
        })
        .insert(Name::new("Loot3"))
        .insert(Loot)
        .insert(LootName(String::from("P890")))
        .insert(LootType::Weapon)
        .insert(FakeLevelStuff);
    // loot cube 4
    commands
        .spawn(PbrBundle {
            mesh: loot_cube.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 1.0, 0.0),
                base_color_texture: Some(texture_06.clone()),
                uv_transform: Affine2::from_scale(Vec2::new(loot_cube_size, loot_cube_size)),
                ..Default::default()
            }),
            transform: Transform::from_xyz(2.0, 0.1, -2.0),
            ..default()
        })
        .insert(Name::new("Loot4"))
        .insert(LootName(String::from("Harddrive")))
        .insert(LootType::Item(ItemType::Item))
        .insert(Loot)
        .insert(Stackable {
            max_stack: 5,
            current_stack: 1,
        })
        .insert(FakeLevelStuff);
    // loot cache 1
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(0.3, 0.3, 0.5)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 0.0, 0.75),
                base_color_texture: Some(texture_06.clone()),
                ..Default::default()
            }),
            transform: Transform::from_xyz(-4.0, 0.15, -4.0),
            ..default()
        })
        .insert(Name::new("Toolbox"))
        .insert(Inventory)
        .insert(ItemSlots(4))
        .insert(FakeLevelStuff);
    // loot cache 2
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(0.4, 2.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 0.0, 0.75),
                base_color_texture: Some(texture_06.clone()),
                ..Default::default()
            }),
            transform: Transform::from_xyz(-4.0, 1.0, -2.0),
            ..default()
        })
        .insert(Name::new("Weapon Locker"))
        .insert(Inventory)
        .insert(Interactable)
        .insert(WeaponSlots(2))
        .insert(ItemSlots(6))
        .insert(FakeLevelStuff)
        .with_children(|parent| {
            parent
                .spawn(PbrBundle {
                    mesh: loot_cube.clone(),
                    material: materials.add(StandardMaterial {
                        base_color: Color::srgb(0.0, 0.75, 0.0),
                        base_color_texture: Some(texture_06.clone()),
                        uv_transform: Affine2::from_scale(Vec2::new(
                            loot_cube_size,
                            loot_cube_size,
                        )),
                        ..Default::default()
                    }),
                    visibility: Visibility::Hidden,
                    transform: Transform::from_xyz(3.0, 0.1, -2.0),
                    ..default()
                })
                .insert(Name::new("WeaponLockerLoot1"))
                .insert(Loot)
                .insert(LootName(String::from("M4")))
                .insert(WeaponSlot(0))
                .insert(LootType::Weapon)
                .insert(FakeLevelStuff);
            parent
                .spawn(PbrBundle {
                    mesh: loot_cube.clone(),
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some(texture_06.clone()),
                        base_color: Color::srgb(0.0, 1.0, 0.0),
                        uv_transform: Affine2::from_scale(Vec2::new(
                            loot_cube_size,
                            loot_cube_size,
                        )),
                        ..Default::default()
                    }),
                    visibility: Visibility::Hidden,
                    transform: Transform::from_xyz(5.0, 1.1, -2.0),
                    ..default()
                })
                .insert(Name::new("WeaponLockerLoot2"))
                .insert(Loot)
                .insert(LootName(String::from("Wrench")))
                .insert(LootType::Item(ItemType::Item))
                .insert(Price(100))
                .insert(ItemSlot(3))
                .insert(Stackable {
                    max_stack: 3,
                    current_stack: 1,
                })
                .insert(FakeLevelStuff);
        });

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

fn start_fake_level_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui
    commands
        .spawn(NodeBundle {
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            style: Style {
                position_type: PositionType::Absolute,
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        })
        .insert(Name::new("Crosshair POC"))
        .insert(FakeLevelStuff)
        .insert(Crosshair)
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture: asset_server.load("textures/crosshair.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: 18.0, y: 18.0 }),
                    ..default()
                },
                ..default()
            });
        });
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
                    base_color: Color::srgb(0.0, 0.75, 0.0),
                    ..Default::default()
                }),
                transform: Transform::from_xyz(0.0, 1.5, 0.25),
                ..default()
            })
            .insert(Name::new("Medium Backpack"))
            .insert(Inventory)
            .insert(ItemSlots(5))
            .insert(WeaponSlots(1))
            .insert(FakeLevelStuff)
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
        primary_window.cursor.visible = false;
        commands.entity(crosshair_vis).insert(Visibility::Visible);
    }

    if key_input.pressed(KeyCode::F10) {
        primary_window.cursor.visible = true;
        commands.entity(crosshair_vis).insert(Visibility::Hidden);
    }

    if key_input.pressed(KeyCode::F11) {
        primary_window.cursor.grab_mode = CursorGrabMode::Confined;
    }

    if key_input.pressed(KeyCode::F12) {
        primary_window.cursor.grab_mode = CursorGrabMode::None;
    }
}

fn bye_fake_level(mut commands: Commands, query: Query<Entity, With<FakeLevelStuff>>) {
    debug!("stopping fake level");
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
