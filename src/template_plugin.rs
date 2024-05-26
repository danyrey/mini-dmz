use bevy::app::Plugin;

use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "template";

// Plugin
pub struct TemplatePlugin;

impl Plugin for TemplatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_template_system)
            .add_systems(
                Update,
                (update_template_system).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_template_system);
    }
}

// Components

// Resources

// Events

fn start_template_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}
fn update_template_system() {
    debug!("updating {}", NAME);
}
fn bye_template_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}
