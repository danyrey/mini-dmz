// loading screen for loading into a match
// TODO: maybe rename this for later when we have other types of loading screens

use crate::AppState::{self, LoadingScreen, Raid};
use bevy::prelude::*;

#[derive(Event)]
struct LoadingStarted;

#[derive(Event)]
struct LoadingFinished;

pub struct MatchLoadingScreenPlugin;

impl Plugin for MatchLoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LoadingScreen), start_matchmake_screen)
            .add_systems(
                Update,
                (update_matchmake_screen).run_if(in_state(LoadingScreen)),
            )
            .add_systems(OnExit(LoadingScreen), bye_matchmake_screen);
    }
}

fn start_matchmake_screen(mut _commands: Commands) {
    debug!("starting loading screen");
}

fn update_matchmake_screen(mut next_state: ResMut<NextState<AppState>>) {
    debug!("updating loading screen");
    next_state.set(Raid);
}

fn bye_matchmake_screen(mut _commands: Commands) {
    debug!("bye loading screen");
}
