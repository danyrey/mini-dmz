use bevy::app::Plugin;

use crate::contracts::{Contracts, FinishedContract};
use crate::exfil::{ExfilExitedAO, Operator};
use crate::inventory::{Inventory, ItemSlot, WeaponSlot};
use crate::squad::{SquadId, Squads};
use crate::AppState;
use crate::AppState::Raid;
use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::prelude::*;

// Constants
const NAME: &str = "raid_summary";

// Plugin
pub struct RaidSummaryPlugin;

impl Plugin for RaidSummaryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RaidSummaries>()
            .add_systems(OnEnter(Raid), start_raid_summary_system)
            .add_systems(
                Update,
                (
                    operator_added,
                    update_raid_summary_system,
                    exit_ao_received,
                    finished_contract_received,
                )
                    .run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_raid_summary_system);
    }
}

// Components

// Resources

#[allow(dead_code)]
#[derive(Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct RaidSummaries {
    pub map: HashMap<Entity, RaidSummary>,
}

#[allow(dead_code)]
#[derive(Default, Reflect, InspectorOptions, Debug, PartialEq)]
#[reflect(InspectorOptions)]
pub struct RaidSummary {
    // TODO: contracts, operator kills, money extracted
    // can be updated mid game via events and only at the end screen printed out somehow
    // items:
    // AI Kills
    // player revives
    // Loot Containers Opened
    // POIs visited
    // Misc Items extracted (do this one at the end)
    // Cash Value Extracted (included cooldown calculation, also at the end)
    /// contracts done
    pub contract_counter: u32,
    /// loot container opened
    pub loot_container_counter: u32,
}

// Events

// Systems

fn start_raid_summary_system(mut commands: Commands) {
    debug!("starting {}", NAME);
    commands.insert_resource(RaidSummaries::default());
}

fn operator_added(added: Query<Entity, Added<Operator>>, mut summaries: ResMut<RaidSummaries>) {
    for operator in added.iter() {
        summaries.map.insert(operator, RaidSummary::default());
    }
}

fn update_raid_summary_system() {
    debug!("updating {}", NAME);
}

fn finished_contract_received(
    mut finished_contract: EventReader<FinishedContract>,
    squads: Res<Squads>,
    contracts: Res<Contracts>,
    operators: Query<(Entity, &SquadId), With<Operator>>,
    mut summaries: ResMut<RaidSummaries>,
) {
    for event in finished_contract.read() {
        if let Some((contract_id, _)) = contracts
            .map
            .iter()
            .find(|(id, _)| event.contract_id.eq(id))
        {
            squads
                .map
                .iter()
                .filter(|(_, squad)| squad.current_contract.is_some())
                .map(|(id, squad)| (id, squad, squad.current_contract.unwrap()))
                .filter(|(_, _, current_contract_id)| contract_id.eq(current_contract_id))
                .for_each(|(squad_id, _, _)| {
                    operators.iter().filter(|(_, id)| squad_id.eq(id)).for_each(
                        |(operator, _id)| {
                            if let Some(summary) = summaries.map.get_mut(&operator) {
                                summary.contract_counter += 1;
                            }
                        },
                    );
                });
        }
    }
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
    //use super::*;

    /*
    #[test]
    fn should_test_something() {
        // given
        //let mut app = App::new();

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
    */
}
