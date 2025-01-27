use bevy::app::Plugin;
use bevy::utils::HashMap;

use crate::contracts::{ContractAccepted, ContractId, ContractPhoneInteracted};
use crate::exfil::Operator;
use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

// Constants
const NAME: &str = "squad";

// Plugin
pub struct SquadPlugin;

impl Plugin for SquadPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SquadId>()
            .register_type::<Squads>()
            .add_systems(OnEnter(Raid), start_squad_system)
            .add_systems(
                Update,
                (
                    operator_added_to_squad,
                    update_squad_system,
                    contract_phone_interacted,
                )
                    .run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_squad_system);
    }
}

// Components

/// squad id component to group squads and items/vehicles/contracts together by id
#[allow(dead_code)]
#[derive(Component, Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub struct SquadId(pub u32);

// Resources
#[allow(dead_code)]
#[derive(Reflect, Clone, InspectorOptions, Debug, PartialEq)]
#[reflect(InspectorOptions)]
pub struct Squad {
    pub max_size: u32,
    pub current_contract: Option<ContractId>,
}

impl Default for Squad {
    fn default() -> Self {
        Squad {
            max_size: 4,
            current_contract: None,
        }
    }
}

#[derive(Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Squads {
    pub map: HashMap<SquadId, Squad>,
}

// Events

// TODO: squad created/added
// TODO: joined squad
// TODO: left squad
// TODO: squad terminated
// ...

// Systems
fn start_squad_system(mut commands: Commands) {
    debug!("starting {}", NAME);
    commands.insert_resource(Squads::default());
}

#[allow(clippy::type_complexity)]
fn operator_added_to_squad(
    added: Query<(Entity, &SquadId), (With<Operator>, Added<SquadId>)>,
    mut squads: ResMut<Squads>,
) {
    for new_squad_member in added.iter() {
        debug!("new squadmember added {:?}", new_squad_member);
        // TODO: check for squad limit, create a new squad if necessary or just add it regardless for now, not sure
        squads.map.entry(new_squad_member.1.clone()).or_default();
    }
}

fn update_squad_system() {
    debug!("updating {}", NAME);
}

fn contract_phone_interacted(
    mut contract_phone_interacted: EventReader<ContractPhoneInteracted>,
    operators: Query<(Entity, &SquadId), With<Operator>>,
    mut squads: ResMut<Squads>,
    mut contract_accepted: EventWriter<ContractAccepted>,
) {
    for event in contract_phone_interacted.read() {
        debug!("contract phone interacted {:?}", event);
        if let Ok((_operator, squad_id)) = operators.get(event.operator_entity) {
            if let Some(squad) = squads.map.get_mut(squad_id) {
                let contract_id = squad.current_contract.get_or_insert(event.contract_id);
                if contract_id.0 == event.contract_id.0 {
                    // contract accepted, otherwise you would have a contract already with different id
                    contract_accepted.send(ContractAccepted {
                        contract_id: *contract_id,
                    });
                }
            }
        }
    }
}

fn bye_squad_system(mut _commands: Commands) {
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
    */
}
