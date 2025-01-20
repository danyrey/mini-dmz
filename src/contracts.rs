use bevy::app::Plugin;

use crate::interaction::Interact;
use crate::AppState;
use crate::AppState::Raid;
use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::prelude::*;

// Constants
const NAME: &str = "contracts";

// Plugin
pub struct ContractsPlugin;

impl Plugin for ContractsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_contract_system)
            .add_systems(
                Update,
                (update_contract_system, interaction_contract_phone)
                    .run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_contract_system);
    }
}

#[allow(dead_code)]
#[derive(Component, Reflect, InspectorOptions, Debug, PartialEq)]
#[reflect(Component, InspectorOptions)]
pub enum ContractType {
    SecureSupplies,
    SecureIntel,
    EliminateHVT,
    DestroySupplies,
    RescueHostage,
    RaidWeaponStash,
    CargoDelivery,
    CargoShipment,
    SecureNuclearMaterials,
    SignalIntelligence,
    HuntSquad,
}

// contract statemachines

#[allow(dead_code)]
#[derive(Default)]
enum ContractState {
    #[default]
    Started,
    SecureSupplies(SecureSuppliesState),
    Cancelled,
    Finished,
}

#[allow(dead_code)]
#[derive(Default)]
enum SecureSuppliesState {
    #[default]
    Started,
    FirstSupplySecured,
    SecondSupplySecured,
    ThirdSupplySecured,
}

// TODO: remaining statemachines

// state machine

trait ContractStateMachine {
    fn next(&mut self) -> ContractState;
}

impl ContractStateMachine for SecureSuppliesState {
    fn next(&mut self) -> ContractState {
        match self {
            Self::Started => ContractState::SecureSupplies(Self::FirstSupplySecured),
            Self::FirstSupplySecured => ContractState::SecureSupplies(Self::SecondSupplySecured),
            Self::SecondSupplySecured => ContractState::SecureSupplies(Self::ThirdSupplySecured),
            Self::ThirdSupplySecured => ContractState::Finished,
        }
    }
}

// Components
#[allow(dead_code)]
#[derive(Component, Debug)]
pub struct ContractPhone;

// Resources
#[allow(dead_code)]
#[derive(Resource, Reflect, InspectorOptions, Debug, PartialEq)]
#[reflect(Resource, InspectorOptions)]
pub struct Contract {
    contract_type: ContractType,
    // TODO: define full contract meta data
}

#[derive(Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Contracts {
    map: HashMap<Entity, Contract>,
}

// Events

// Systems
fn start_contract_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}
fn update_contract_system() {
    debug!("updating {}", NAME);
}

fn interaction_contract_phone(
    mut interaction_commands: EventReader<Interact>,
    contract_phone_query: Query<(Entity, &ContractType), With<ContractPhone>>,
) {
    for command in interaction_commands.read() {
        // filter for commands on ContractPhone entities only
        if let Ok((phone, contract_type)) = contract_phone_query.get(command.interaction_entity) {
            debug!(
                "interacted with contract phone: {:?}, type: {:?}",
                phone, contract_type
            );
        }
    }
}

fn bye_contract_system(mut _commands: Commands) {
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
