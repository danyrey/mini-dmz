use bevy::app::Plugin;
use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};

use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// TODO: generate heightmap and mesh
// Constants
const NAME: &str = "heightmap";

// Plugin
pub struct HeightmapPlugin;

impl Plugin for HeightmapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_heightmap_system)
            .add_systems(
                Update,
                (update_heightmap_system).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_heightmap_system);
    }
}

// Components

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
fn start_heightmap_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    debug!("starting {}", NAME);

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

    // terrain
    commands
        .spawn(SceneRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("models/terrain/Mountains.gltf"),
        )))
        .insert(Transform {
            scale: Vec3 {
                x: 16.0,
                y: 16.0,
                z: 16.0,
            },
            ..default()
        })
        .insert(Name::from("Mountains"));
}
fn update_heightmap_system() {
    debug!("updating {}", NAME);
}
fn bye_heightmap_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions

/// return the given y for a given x,z coordinate
pub trait YProbe {
    fn probe_y(x: f32, z: f32) -> f32;
}

pub struct FlatEarth;

impl YProbe for FlatEarth {
    fn probe_y(_x: f32, _z: f32) -> f32 {
        0.0
    }
}

pub struct XSineTerrain;

impl YProbe for XSineTerrain {
    fn probe_y(x: f32, _z: f32) -> f32 {
        x.sin()
    }
}

pub struct XZSineTerrain;

impl YProbe for XZSineTerrain {
    fn probe_y(x: f32, z: f32) -> f32 {
        x.sin() + z.sin()
    }
}

// tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    //#[test]
    //fn should_test_something() {
    // given
    //let mut _app = App::new();

    // when
    //app.add_event::<HealthDamageReceived>();
    //app.add_systems(Update, damage_received_listener);
    //let entity = app.borrow_mut().world.spawn(Health(100)).id();
    //app.borrow_mut().world.resource_mut::<Events<HealthDamageReceived>>().send(HealthDamageReceived { entity, damage: 10 });
    //app.update();

    // then
    //assert!(app.world.get::<Health>(entity).is_some());
    //assert_eq!(app.world.get::<Health>(entity).unwrap().0, 90);
    //}
}
