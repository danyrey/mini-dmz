use bevy::app::Plugin;

use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "out of bounds";

// Plugin
pub struct OutOfBoundsPlugin;

impl Plugin for OutOfBoundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_out_of_bounds_system)
            .add_systems(
                Update,
                (update_out_of_bounds_system).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_out_of_bounds_system);
    }
}

// Components

// Resources

// Events

// Systems
fn start_out_of_bounds_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}
fn update_out_of_bounds_system() {
    debug!("updating {}", NAME);
}
fn bye_out_of_bounds_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions
