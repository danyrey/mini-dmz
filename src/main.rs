use active_duty_confirmation::ActiveDutyConfirmationScreenPlugin;
use active_missions::ActiveMissionsScreenPlugin;
use armor::ArmorPlugin;
use bevy::prelude::*;
use cache::CachePlugin;
use camera_look::CameraLookPlugin;
use camera_move::CameraMovePlugin;
use choose_location::ChooseLocationScreenPlugin;
use damage::DamagePlugin;
use exfil::ExfilPlugin;
use exfil_timers::ExfilTimersPlugin;
use fake_level::FakeLevelPlugin;
use health::HealthPlugin;
use inventory::InventoryPlugin;
use inventory_testing::InventoryTestingPlugin;
use loading_screen::MatchLoadingScreenPlugin;
use loot::LootPlugin;
use matchmake::{MatchmakeInProgressScreenPlugin, MatchmakeScreenPlugin};
use mission_objective_screen::MissionObjectivesScreenPlugin;
use out_of_bounds::OutOfBoundsPlugin;
use raid::RaidPlugin;
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
mod health;
mod inventory;
mod inventory_testing;
mod loading_screen;
mod loadout;
mod loot;
mod matchmake;
mod mission_objective_screen;
mod operator_controller;
mod out_of_bounds;
mod raid;
mod start_screen;
mod template_plugin;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    StartScreen,
    MissionObjectives(MissionObjectives),
    DeployScreen(DeployScreen),
    LoadingScreen,
    Raid,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
enum MissionObjectives {
    #[default]
    Start,
    Missions,
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
    commands.spawn(Camera2dBundle::default());
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
            CameraLookPlugin,
            CameraMovePlugin,
            OutOfBoundsPlugin,
            HealthPlugin,
            ArmorPlugin,
            TemplatePlugin,
        ))
        .add_plugins((
            RaidPlugin,
            FakeLevelPlugin,
            ExfilPlugin,
            ExfilTimersPlugin,
            DamagePlugin,
            LootPlugin,
            CachePlugin,
            InventoryPlugin,
            InventoryTestingPlugin,
        ))
        .init_state::<AppState>()
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .run();
}
