use bevy::app::Plugin;

use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "inventory";

// Plugin
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_inventory_system)
            .add_systems(
                Update,
                (update_inventory_system).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_inventory_system);
    }
}

// Components

// Resources

// Events
#[derive(Event, Debug, PartialEq)]
pub struct DroppedFromInventory {
    pub dropping_entity: Entity,
    pub dropped_position: Vec3,
    pub loot: Entity,
}

#[derive(Event, Debug, PartialEq)]
pub struct PickedUpLoot {
    pub pickup_entity: Entity,
    pub loot: Entity,
}

// Systems
fn start_inventory_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}
fn update_inventory_system() {
    debug!("updating {}", NAME);
    // TODO: manage events
}
fn bye_inventory_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_test_something() {
        // given
        let mut app = App::new();

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
