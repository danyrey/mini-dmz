use bevy::app::Plugin;

use crate::AppState;
use crate::AppState::Raid;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

use bevy::prelude::*;

// Constants
const NAME: &str = "wallet";

// Plugin
pub struct WalletPlugin;

impl Plugin for WalletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_wallet_system)
            .add_systems(
                Update,
                (update_wallet_system).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_wallet_system);
    }
}

// Components

#[derive(Component, Debug, PartialEq, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Wallet {
    pub money: u32,
    pub limit: u32,
}

impl Default for Wallet {
    fn default() -> Self {
        Wallet {
            money: 0,
            limit: 250_000,
        }
    }
}
// Resources

// Events

// Systems
fn start_wallet_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}
fn update_wallet_system() {
    debug!("updating {}", NAME);
}
fn bye_wallet_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    //#[test]
    fn should_test_something() {
        // given
        //let mut _app = App::new();

        // when
        //app.add_event::<HealthDamageReceived>();
        //app.add_systems(Update, damage_received_listener);
        //let entity = app.borrow_mut().world.spawn(Health(100)).id();
        //app.borrow_mut().world.resource_mut::<Events<HealthDamageReceived>>().send(HealthDamageReceived { entity, damage: 10 });
        //app.update();

        // then
        //assert!(app.world.get::<Health>(entity).is_some());
        //assert_eq!(app.world.get::<Health>(entity).unwrap().0, 90);
    }
}
