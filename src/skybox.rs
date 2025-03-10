use crate::first_person_controller::{start_first_person_controller_system, FirstPersonCamera};
use crate::AppState;
use crate::AppState::Raid;
use bevy::image::CompressedImageFormats;
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};
use bevy::{app::Plugin, core_pipeline::Skybox, prelude::*};
use bevy_inspector_egui::prelude::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;

// Constants
const NAME: &str = "skybox";

// straight from the bevy examples, just using the first one for now
const CUBEMAPS: &[(&str, CompressedImageFormats)] = &[(
    "textures/Ryfjallet_cubemap.png",
    CompressedImageFormats::NONE,
)];

// Plugin
pub struct SkyboxPlugin;

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(Raid),
            start_skybox_system.after(start_first_person_controller_system),
        )
        .add_systems(
            Update,
            (update_skybox_system, asset_loaded).run_if(in_state(AppState::Raid)),
        )
        .add_systems(OnExit(AppState::Raid), bye_skybox_system)
        .init_resource::<Cubemap>()
        .register_type::<Cubemap>();
    }
}

// Components

// Resources
#[derive(Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Cubemap {
    is_loaded: bool,
    index: usize,
    image_handle: Handle<Image>,
}

// Events

// Systems
fn start_skybox_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<Entity, With<FirstPersonCamera>>,
) {
    debug!("starting {}", NAME);

    let skybox_handle = asset_server.load(CUBEMAPS[0].0);

    let cam = camera_query.single();
    commands.entity(cam).insert(Skybox {
        image: skybox_handle.clone(),
        brightness: 1000.0,
        ..default()
    });

    commands.insert_resource(Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: skybox_handle,
    });
}

fn asset_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle).is_loaded() {
        info!("Swapping to {}...", CUBEMAPS[cubemap.index].0);
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.image = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}

fn update_skybox_system() {
    debug!("updating {}", NAME);
}

fn bye_skybox_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions
