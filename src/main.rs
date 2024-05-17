use active_duty_confirmation::ActiveDutyConfirmationScreenPlugin;
use bevy::prelude::*;
use choose_location::ChooseLocationScreenPlugin;
use mission_objective_screen::MissionObjectivesScreenPlugin;
use start_screen::StartScreenPlugin;

mod active_duty_confirmation;
mod choose_location;
mod deploy;
mod matchmake;
mod mission_objective_screen;
mod missions;
mod start_screen;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    StartScreen,
    MissionObjectives(MissionObjectives),
    DeployScreen(DeployScreen),
    Match,
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
}

// TODO : enable proper inspector output, currently it shows:
// "ButtonTargetState is not registered in the TypeRegistry"
#[derive(Component, Debug)]
struct ButtonTargetState(AppState);

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

/*
fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        debug!("hello {}!", name.0);
    }
}
*/

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
            ActiveDutyConfirmationScreenPlugin,
        ))
        .init_state::<AppState>()
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, (add_people, setup))
        //.add_systems(Update, greet_people)
        .run();
}
