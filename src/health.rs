///
/// HealthPlugin/-System only manages the actual damage book keeping.
/// A separate Damage System assigns damage to the according Plugins.
///
use bevy::app::Plugin;

use crate::damage::HealthDamageReceived;
use crate::death::EntityDie;
use crate::AppState;
use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;

// Constants
const _NAME: &str = "health";

// Plugin
pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app
            // types
            .register_type::<Health>()
            // systems
            .add_systems(
                Update,
                (damage_received_listener).run_if(in_state(AppState::Raid)),
            );
    }
}

// Components
#[derive(Component, Debug, PartialEq, Reflect, InspectorOptions)]
pub struct Health(pub i32);

impl Default for Health {
    fn default() -> Self {
        Health(100)
    }
}

// Resources

// Events

// Systems

#[allow(clippy::type_complexity)]
fn damage_received_listener(
    mut health_damage: EventReader<HealthDamageReceived>,
    mut query: Query<(Entity, &mut Health)>,
    mut dying: EventWriter<EntityDie>,
) {
    for event in health_damage.read() {
        debug!(
            "event received for entity {:?}, damage received: {}",
            event.entity, event.damage
        );
        for (entity, mut health) in &mut query {
            if entity == event.entity {
                health.0 -= event.damage;
                debug!(
                    "event applied to entity {:?}, damage applied: {}",
                    entity, health.0
                );
                if health.0 <= 0 {
                    dying.send(EntityDie {
                        dying: entity,
                        killer: event.dealer,
                    });
                }
            }
        }
    }
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_reduce_health() {
        // Setup app
        let mut app = App::new();

        // Add `DamageReceived` event
        app.add_event::<HealthDamageReceived>();
        app.add_event::<EntityDie>();

        // Add our two systems
        app.add_systems(Update, damage_received_listener);

        // Setup test entities
        let entity = app.borrow_mut().world_mut().spawn(Health(100)).id();

        // Send an `DamageReceived` event
        app.borrow_mut()
            .world_mut()
            .resource_mut::<Events<HealthDamageReceived>>()
            .send(HealthDamageReceived {
                entity,
                damage: 10,
                dealer: Option::None,
            });

        // Run systems
        app.update();

        // Check resulting changes
        assert!(app.world().get::<Health>(entity).is_some());
        assert_eq!(app.world().get::<Health>(entity).unwrap().0, 90);
    }
}
