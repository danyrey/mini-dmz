use bevy::app::Plugin;

use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

pub struct FakeLevelPlugin;

impl Plugin for FakeLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_fake_level)
            .add_systems(Update, (update_fake_level).run_if(in_state(AppState::Raid)))
            .add_systems(OnExit(AppState::Raid), bye_fake_level);
    }
}

fn start_fake_level(mut _commands: Commands) {
    // TODO: some cubes and planes
    debug!("starting fake level");
}
fn update_fake_level() {
    // TODO: maybe just render them near any cameras
    // TODO: maybe put code here that moves the scene near cameras to maintain a reference for
    // movement
    debug!("updating fake level");
}
fn bye_fake_level(mut _commands: Commands) {
    // TODO: cleanup scene
    debug!("stopping fake level");
}
