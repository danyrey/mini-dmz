use bevy::app::Plugin;

use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "health";

// Plugin
pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_health_system)
            .add_systems(
                Update,
                (update_health_system).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_health_system);
    }
}

// Components
#[derive(Component)]
pub struct Health(u8);

impl Default for Health {
    fn default() -> Self {
        Health(100)
    }
}

// Resources

// Events

// Systems
fn start_health_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}
fn update_health_system() {
    debug!("updating {}", NAME);
}
fn bye_health_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions
