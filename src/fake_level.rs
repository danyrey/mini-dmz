// TODO: how to make sure every operator has a backpack attached to it
//  TODO: transfer from the active loadout screen should be done
//  * transfer from state from one appstate to another: active dute layout -> ...load in -> raid
use crate::damage::HurtBox;
use crate::exfil::{ExfilArea, Operator};
use crate::first_person_controller::FirstPersonCamera;
use crate::inventory::{Inventory, ItemSlots, WeaponSlots};
use crate::loot::{Durability, ItemType, Loot, LootName, LootType, Price, Rarity, Stackable};
use crate::raid::Enemy;
use crate::AppState;
use crate::AppState::Raid;
use bevy::math::bounding::{Aabb3d, RayCast3d};
use bevy::prelude::*;
use bevy::render::primitives::{Aabb, Frustum};

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
                    fixup_prototype_textures,
                    probe_interact_volumes,
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
#[derive(Resource)]
struct PrototypeTextures {
    texture_01: Handle<Image>,
}

// Events

// Systems
fn fixup_prototype_textures(
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut images: ResMut<Assets<Image>>,
    proto_imgs: Res<PrototypeTextures>,
) {
    // TODO
    for ev in ev_asset.read() {
        if let AssetEvent::LoadedWithDependencies { id } = ev {
            // image is prototype texture
            if *id == proto_imgs.texture_01.id() {
                // image loaded, so unwrap should be ok
                let image = images.get_mut(*id).unwrap();
                debug!("image size: {}", image.size());
                // TODO: rescale here???
            }
        }
    }
}
fn start_fake_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    debug!("starting fake level");
    let texture_01 = asset_server.load("textures/prototype/Dark/texture_01.png");
    commands.insert_resource(PrototypeTextures {
        texture_01: texture_01.clone(),
    });

    // circular base
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Circle::new(8.0)),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_01.clone()),
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
                base_color_texture: Some(texture_01.clone()),
                base_color: Color::GOLD,
                ..Default::default()
            }),
            transform: Transform::from_xyz(20.0, 0.5, 0.0),
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
                base_color_texture: Some(texture_01.clone()),
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
                base_color_texture: Some(texture_01.clone()),
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
                base_color_texture: Some(texture_01.clone()),
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
                base_color_texture: Some(texture_01.clone()),
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
                base_color_texture: Some(texture_01.clone()),
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
                base_color_texture: Some(texture_01.clone()),
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
                base_color_texture: Some(texture_01.clone()),
                ..Default::default()
            }),
            transform: Transform::from_xyz(2.0, 0.1, -2.0),
            ..default()
        })
        .insert(Name::new("Loot4"))
        .insert(LootType::Item(ItemType::Item))
        .insert(Loot)
        .insert(FakeLevelStuff);
    // loot cache 1
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(0.3, 0.3, 0.5)),
            material: materials.add(StandardMaterial {
                base_color: Color::ALICE_BLUE,
                base_color_texture: Some(texture_01.clone()),
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
                base_color: Color::ALICE_BLUE,
                base_color_texture: Some(texture_01.clone()),
                ..Default::default()
            }),
            transform: Transform::from_xyz(-4.0, 1.0, -2.0),
            ..default()
        })
        .insert(Name::new("Weapon Locker"))
        .insert(Inventory)
        .insert(WeaponSlots(2))
        .insert(ItemSlots(6))
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

fn probe_interact_volumes(
    interact_probe: Query<(&Frustum, &GlobalTransform), With<FirstPersonCamera>>,
    aabbs: Query<(&Aabb, &GlobalTransform, &Name), With<Loot>>,
    mut gizmos: Gizmos,
) {
    let probe = interact_probe.single();
    debug!("probe_results:-----------");
    aabbs
        .iter()
        // check if loot are in camera or not
        .filter(|aabb| probe.0.intersects_obb(aabb.0, &aabb.1.affine(), true, true))
        .for_each(|aabb| {
            debug!("probe_result: {}", aabb.2);
            let looking_at_direction = probe.0.half_spaces[4].normal();
            let position = probe.1.translation();
            let r = RayCast3d::new(
                position,
                Direction3d::new(looking_at_direction.into()).unwrap(),
                2.0,
            );
            let aabb3d = Aabb3d::new(aabb.1.translation(), aabb.0.half_extents.into());
            let intersects = r.aabb_intersection_at(&aabb3d);
            if let Some(_) = intersects {
                debug!("im allowed to pick {} up", aabb.2);
                gizmos.cuboid(
                    Transform::from_translation(aabb.1.translation()).with_scale(Vec3::splat(0.25)),
                    Color::GOLD,
                );
            }
        })
}

// renders some fake level exclusive gizmos
fn update_fake_level(
    mut gizmos: Gizmos,
    images: ResMut<Assets<Image>>,
    query: Query<&GlobalTransform, With<Enemy>>,
    loot_query: Query<&GlobalTransform, With<Loot>>,
) {
    debug!("updating fake level");
    images.iter().for_each(|i| {
        debug!("image: {:?}", i.1.size());
    });
    for global_transform in query.iter() {
        gizmos.ray(
            global_transform.to_scale_rotation_translation().2 + Vec3::new(0.0, 0.75, 0.0),
            (global_transform.to_scale_rotation_translation().1 * Vec3::Z).xyz() * -1.0,
            Color::RED,
        );
    }
    for global_transform in loot_query.iter() {
        gizmos.ray(
            global_transform.translation(),
            Vec3::new(0.5, 0.0, 0.0),
            Color::RED,
        );
        gizmos.ray(
            global_transform.translation(),
            Vec3::new(0.0, 0.5, 0.0),
            Color::GREEN,
        );
        gizmos.ray(
            global_transform.translation(),
            Vec3::new(0.0, 0.0, 0.5),
            Color::BLUE,
        );
    }
}

fn bye_fake_level(mut commands: Commands, query: Query<Entity, With<FakeLevelStuff>>) {
    debug!("stopping fake level");
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
