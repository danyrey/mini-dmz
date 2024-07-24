use bevy::app::Plugin;

use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "damage";

// Plugin
pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_damage_system)
            .add_systems(
                Update,
                (update_damage_system).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_damage_system);
    }
}

// Components

// Resources

// Events
#[derive(Event)]
pub struct ArmorDamageReceived {
    pub entity: Entity,
    pub damage: u8,
}

#[derive(Event)]
pub struct HealthDamageReceived {
    pub entity: Entity,
    pub damage: u8,
}

// Systems
fn start_damage_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}
fn update_damage_system(
    // TODO: query the components to which to decide damage events
    mut _health: EventWriter<HealthDamageReceived>,
    mut _armor: EventWriter<ArmorDamageReceived>,
) {
    debug!("updating {}", NAME);
}
fn bye_damage_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions
