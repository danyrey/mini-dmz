/**

ArmorPlugin/-System only manages the actual damage book keeping.
A separate Damage System assigns damage to the according Plugins.

*/
use bevy::app::Plugin;

use crate::damage::ArmorDamageReceived;
use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "armor";

// Plugin
pub struct ArmorPlugin;

impl Plugin for ArmorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_armor_system)
            .add_systems(
                Update,
                (update_armor_system, damage_received_listener).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_armor_system);
    }
}

// Components
#[derive(Component)]
pub struct Armor(pub i32);

impl Default for Armor {
    fn default() -> Self {
        Armor(100)
    }
}

// Resources

// Events

// Systems
fn start_armor_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}
fn update_armor_system() {
    debug!("updating {}", NAME);
}
fn bye_armor_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

fn damage_received_listener(
    mut armor_damage: EventReader<ArmorDamageReceived>,
    mut query: Query<(Entity, &mut Armor)>,
) {
    for event in armor_damage.read() {
        debug!(
            "event received for operator {:?}, damage received: {}",
            event.entity, event.damage
        );
        for (entity, mut armor) in &mut query {
            if entity == event.entity {
                armor.0 = armor.0 - event.damage;
                debug!(
                    "event applied to operator {:?}, damage applied: {}",
                    entity, armor.0
                );
            }
        }
    }
}

// helper functions

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // tests
    #[test]
    fn should_reduce_armor() {
        // Setup app
        let mut app = App::new();

        // Add `DamageReceived` event
        app.add_event::<ArmorDamageReceived>();

        // Add event listener
        app.add_systems(Update, damage_received_listener);

        // Setup test entities
        let entity = app.borrow_mut().world.spawn(Armor(100)).id();

        // Send an `DamageReceived` event
        app.borrow_mut()
            .world
            .resource_mut::<Events<ArmorDamageReceived>>()
            .send(ArmorDamageReceived { entity, damage: 10 });

        // Run systems
        app.update();

        // Check resulting changes
        assert!(app.world.get::<Armor>(entity).is_some());
        assert_eq!(app.world.get::<Armor>(entity).unwrap().0, 90);
    }
}
