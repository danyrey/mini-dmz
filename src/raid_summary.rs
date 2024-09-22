use bevy::app::Plugin;

use crate::exfil::{ExfilExitedAO, Operator};
use crate::inventory::{Inventory, ItemSlot, WeaponSlot};
use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "raid_summary";

// Plugin
pub struct RaidSummaryPlugin;

impl Plugin for RaidSummaryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_raid_summary_system)
            .add_systems(
                Update,
                (update_raid_summary_system, exit_ao_received).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_raid_summary_system);
    }
}

// Components

// Resources

// Events

// Systems
fn start_raid_summary_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn update_raid_summary_system() {
    debug!("updating {}", NAME);
}

// TODO: query for the actual exfilled operator
#[allow(clippy::type_complexity)]
fn exit_ao_received(
    mut exited_ao: EventReader<ExfilExitedAO>,
    operator_query: Query<(Entity, Option<&Name>, &Children), With<Operator>>,
    backpack_query: Query<(Entity, &Children), With<Inventory>>,
    items_query: Query<(Entity, Option<&Name>), With<ItemSlot>>,
    weapons_query: Query<(Entity, Option<&Name>), With<WeaponSlot>>,
) {
    for event in exited_ao.read() {
        debug!(
            "operator '{:?}' exited the AO, let me do a summary:",
            event.operator_entity
        );

        let event_op = operator_query.get(event.operator_entity).unwrap(); // maybe we dont need operator query at
                                                                           // all since it given from the event
        for (_, _, inventories) in operator_query.iter().filter(|(op, _, _)| *op == event_op.0) {
            // TODO
            for inventory in inventories {
                // TODO
                for (_, contents) in backpack_query.iter().filter(|(inv, _)| inv == inventory) {
                    // TODO
                    for content in contents {
                        let item = items_query.get(*content);
                        let weapon = weapons_query.get(*content);
                        if let Ok(i) = item {
                            debug!("item: {:?} : {:?}", i.0, i.1);
                        }
                        if let Ok(w) = weapon {
                            debug!("weapon: {:?} : {:?}", w.0, w.1);
                        }
                    }
                }
            }
        }
    }
}

fn bye_raid_summary_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::borrow::BorrowMut;

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
