use active_duty_confirmation::ActiveDutyConfirmationScreenPlugin;
use active_missions::ActiveMissionsScreenPlugin;
use bevy::prelude::*;
use camera_look::CameraLookPlugin;
use camera_move::CameraMovePlugin;
use choose_location::ChooseLocationScreenPlugin;
use exfil::ExfilPlugin;
use fake_level::FakeLevelPlugin;
use loading_screen::MatchLoadingScreenPlugin;
use matchmake::{MatchmakeInProgressScreenPlugin, MatchmakeScreenPlugin};
use mission_objective_screen::MissionObjectivesScreenPlugin;
use raid::RaidPlugin;
use start_screen::StartScreenPlugin;
use template_plugin::TemplatePlugin;

mod active_duty_confirmation;
mod active_missions;
mod camera_look;
mod camera_move;
mod choose_location;
mod deploy;
mod exfil;
mod fake_level;
mod loading_screen;
mod matchmake;
mod mission_objective_screen;
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
            FakeLevelPlugin,
            RaidPlugin,
            ExfilPlugin,
            CameraLookPlugin,
            CameraMovePlugin,
            TemplatePlugin,
        ))
        .init_state::<AppState>()
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .run();
}
