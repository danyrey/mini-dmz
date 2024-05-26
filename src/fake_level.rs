use bevy::app::Plugin;

use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

pub struct FakeLevelPlugin;

impl Plugin for FakeLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_matchmake_screen)
            .add_systems(
                Update,
                (update_matchmake_screen).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_matchmake_screen);
    }
}

fn start_matchmake_screen(mut _commands: Commands) {
    // TODO: some cubes and planes
}
fn update_matchmake_screen() {
    // TODO: maybe just render them near any cameras
    // TODO: maybe put code here that moves the scene near cameras to maintain a reference for
    // movement
}
fn bye_matchmake_screen(mut _commands: Commands) {
    // TODO: cleanup scene
}
