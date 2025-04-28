use active_duty_confirmation::ActiveDutyConfirmationScreenPlugin;
use active_missions::ActiveMissionsScreenPlugin;
use armor::ArmorPlugin;
use backpack_summary::BackpackSummaryPlugin;
use bevy::prelude::*;
use choose_location::ChooseLocationScreenPlugin;
use compass::CompassPlugin;
use contracts::ContractsPlugin;
use coordinates::CoordinatesPlugin;
use damage::DamagePlugin;
use exfil::ExfilPlugin;
use exfil_timers::ExfilTimersPlugin;
use fake_level::FakeLevelPlugin;
use first_person_controller::FirstPersonControllerPlugin;
use flee::FleePlugin;
use follow::FollowPlugin;
use health::HealthPlugin;
use heightmap::HeightmapPlugin;
use interaction::InteractionPlugin;
use inventory::InventoryPlugin;
use inventory_testing::InventoryTestingPlugin;
use inventory_ui::InventoryUIPlugin;
use loading_screen::MatchLoadingScreenPlugin;
use lock::LockPlugin;
use loot::LootPlugin;
use matchmake::{MatchmakeInProgressScreenPlugin, MatchmakeScreenPlugin};
use mission_objective_screen::MissionObjectivesScreenPlugin;
use out_of_bounds::OutOfBoundsPlugin;
use point_of_interest::PointOfInterestPlugin;
use raid::RaidPlugin;
use raid_summary::RaidSummaryPlugin;
use skybox::SkyboxPlugin;
use spawn::SpawnPlugin;
use squad::SquadPlugin;
use start_screen::StartScreenPlugin;
use wallet::WalletPlugin;

mod active_duty_confirmation;
mod active_missions;
mod armor;
mod backpack_summary;
mod choose_location;
mod compass;
mod contracts;
mod coordinates;
mod damage;
mod deploy;
mod exfil;
mod exfil_timers;
mod fake_level;
mod first_person_controller;
mod flee;
mod follow;
mod health;
mod heightmap;
mod interaction;
mod inventory;
mod inventory_testing;
mod inventory_ui;
mod loading_screen;
mod loadout;
mod lock;
mod loot;
mod matchmake;
mod mission_objective_screen;
mod out_of_bounds;
mod point_of_interest;
mod raid;
mod raid_summary;
mod skybox;
mod spawn;
mod squad;
mod start_screen;
mod template_plugin;
mod wallet;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
pub enum AppState {
    StartScreen,
    MissionObjectives(MissionObjectives),
    DeployScreen(DeployScreen),
    #[default]
    LoadingScreen,
    Raid,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
pub enum MissionObjectives {
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
pub enum DeployScreen {
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
    commands.spawn(Camera2d).insert(Camera {
        order: 1,
        ..default()
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
            OutOfBoundsPlugin,
            HealthPlugin,
            ArmorPlugin,
        ))
        .add_plugins((
            RaidPlugin,
            FirstPersonControllerPlugin,
            FakeLevelPlugin,
            ExfilPlugin,
            ExfilTimersPlugin,
            DamagePlugin,
            LootPlugin,
            InventoryPlugin,
            RaidSummaryPlugin,
            InventoryTestingPlugin,
            SkyboxPlugin,
            HeightmapPlugin,
            InventoryUIPlugin,
            InteractionPlugin,
        ))
        .add_plugins((
            FollowPlugin,
            FleePlugin,
            CompassPlugin,
            PointOfInterestPlugin,
            CoordinatesPlugin,
            WalletPlugin,
            BackpackSummaryPlugin,
            ContractsPlugin,
            SquadPlugin,
            SpawnPlugin,
            LockPlugin,
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
