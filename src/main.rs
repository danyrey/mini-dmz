use active_duty_confirmation::ActiveDutyConfirmationScreenPlugin;
use active_missions::ActiveMissionsScreenPlugin;
use armor::ArmorPlugin;
use bevy::prelude::*;
use cache::CachePlugin;
//use camera_look::CameraLookPlugin;
//use camera_move::CameraMovePlugin;
use choose_location::ChooseLocationScreenPlugin;
use damage::DamagePlugin;
use exfil::ExfilPlugin;
use exfil_timers::ExfilTimersPlugin;
use fake_level::FakeLevelPlugin;
use first_person_controller::FirstPersonControllerPlugin;
use health::HealthPlugin;
use inventory::InventoryPlugin;
use inventory_testing::InventoryTestingPlugin;
use loading_screen::MatchLoadingScreenPlugin;
use loot::LootPlugin;
use matchmake::{MatchmakeInProgressScreenPlugin, MatchmakeScreenPlugin};
use mission_objective_screen::MissionObjectivesScreenPlugin;
use out_of_bounds::OutOfBoundsPlugin;
use raid::RaidPlugin;
use raid_summary::RaidSummaryPlugin;
use skybox::SkyboxPlugin;
use start_screen::StartScreenPlugin;
use template_plugin::TemplatePlugin;

mod active_duty_confirmation;
mod active_missions;
mod armor;
mod cache;
mod camera_look;
mod camera_move;
mod choose_location;
mod damage;
mod deploy;
mod exfil;
mod exfil_timers;
mod fake_level;
mod first_person_controller;
mod health;
mod inventory;
mod inventory_testing;
mod loading_screen;
mod loadout;
mod loot;
mod matchmake;
mod mission_objective_screen;
mod out_of_bounds;
mod raid;
mod raid_summary;
mod skybox;
mod start_screen;
mod template_plugin;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
enum AppState {
    StartScreen,
    MissionObjectives(MissionObjectives),
    DeployScreen(DeployScreen),
    LoadingScreen,
    #[default]
    Raid,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
enum MissionObjectives {
    #[default]
    Start,
    Missions,
    #[allow(dead_code)]
    EditMissions, // how to remove redundancy as this screen exists also in DeployScreen
    Upgrades,
    LocationObjectives,
    Notes,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
enum DeployScreen {
    #[default]
    ChooseLocation,
    ActiveMissions,
    EditMissions, // how to remove redundancy as this screen exists also in MissionObjectives
    ActiveDutyConfirmation,
    EditLoadout, // how to remove redundancy as this screen exists also in MissionObjectives
    MatchMake,
    MatchMakeInProgress,
}

// TODO : enable proper inspector output, currently it shows:
// "ButtonTargetState is not registered in the TypeRegistry"
#[derive(Component, Debug)]
struct ButtonTargetState(AppState);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 1,
            ..default()
        },
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            StartScreenPlugin,
            MissionObjectivesScreenPlugin,
            ChooseLocationScreenPlugin,
            ActiveMissionsScreenPlugin,
            ActiveDutyConfirmationScreenPlugin,
            MatchmakeScreenPlugin,
            MatchmakeInProgressScreenPlugin,
            MatchLoadingScreenPlugin,
            //CameraLookPlugin,
            //CameraMovePlugin,
            OutOfBoundsPlugin,
            HealthPlugin,
            ArmorPlugin,
            TemplatePlugin,
        ))
        .add_plugins((
            RaidPlugin,
            FirstPersonControllerPlugin,
            FakeLevelPlugin,
            ExfilPlugin,
            ExfilTimersPlugin,
            DamagePlugin,
            LootPlugin,
            CachePlugin,
            InventoryPlugin,
            RaidSummaryPlugin,
            InventoryTestingPlugin,
            SkyboxPlugin,
        ))
        .init_state::<AppState>()
        .add_systems(Update, close_on_esc)
        .add_systems(Startup, setup)
        .run();
}

pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}
