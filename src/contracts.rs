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
        app.register_type::<Contracts>()
            .add_event::<ContractPhoneInteracted>()
            .add_systems(OnEnter(Raid), start_contract_system)
            .add_systems(
                Update,
                interaction_contract_phone.run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_contract_system);
    }
}

// Components

#[allow(dead_code)]
#[derive(Component, Copy, Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub struct ContractId(pub u32);

#[allow(dead_code)]
#[derive(Component, Debug)]
pub struct ContractPhone;

#[allow(dead_code)]
#[derive(Component, Clone, Reflect, InspectorOptions, Debug, PartialEq)]
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

#[allow(dead_code)]
#[derive(Component, Debug)]
pub struct ContractPayout(u32);

impl Default for ContractPayout {
    fn default() -> Self {
        ContractPayout(2000)
    }
}

// contract statemachines

#[allow(dead_code)]
#[derive(Component, Default, Reflect, InspectorOptions, Debug, PartialEq)]
#[reflect(Component, InspectorOptions)]
enum ContractState {
    #[default]
    Started,
    SecureSupplies(SecureSuppliesState),
    Cancelled,
    Finished,
}

#[allow(dead_code)]
#[derive(Default, Reflect, InspectorOptions, Debug, PartialEq)]
#[reflect(InspectorOptions)]
enum SecureSuppliesState {
    #[default]
    Started,
    FirstSupplySecured,
    SecondSupplySecured,
    ThirdSupplySecured,
}

// TODO: remaining statemachines

// state machine

#[allow(dead_code)]
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

// Resources
#[allow(dead_code)]
#[derive(Reflect, InspectorOptions, Debug, PartialEq)]
#[reflect(InspectorOptions)]
pub struct Contract {
    contract_type: ContractType,
    contract_state: ContractState,
    contract_payout: u32,
}

#[derive(Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Contracts {
    map: HashMap<ContractId, Contract>,
}

// Events

#[derive(Event, Debug, PartialEq)]
pub struct ContractPhoneInteracted {
    pub contract_id: ContractId,
    pub operator_entity: Entity,
}

// Systems
fn start_contract_system(mut commands: Commands) {
    debug!("starting {}", NAME);
    commands.insert_resource(Contracts::default());
}

/// interaction with contract phone to start a contract
fn interaction_contract_phone(
    mut interaction_commands: EventReader<Interact>,
    contract_phone_query: Query<
        (Entity, &ContractId, &ContractType, Option<&ContractPayout>),
        With<ContractPhone>,
    >,
    mut contracts: ResMut<Contracts>,
    mut interacted: EventWriter<ContractPhoneInteracted>,
) {
    for command in interaction_commands.read() {
        // filter for commands on ContractPhone entities only
        if let Ok((phone, contract_id, contract_type, payout)) =
            contract_phone_query.get(command.interaction_entity)
        {
            debug!(
                "interacted with contract phone: {:?}, type: {:?}",
                phone, contract_type
            );
            contracts.map.insert(
                *contract_id,
                Contract {
                    contract_type: contract_type.clone(),
                    contract_state: initial_state(contract_type.clone()),
                    contract_payout: payout.unwrap_or(&ContractPayout::default()).0, // TODO: apply user upgrade level modifier later
                },
            );
            debug!("added contract to contracts resource");
            interacted.send(ContractPhoneInteracted {
                contract_id: *contract_id,
                operator_entity: command.operator_entity,
            });
        }
    }
}

fn bye_contract_system(mut commands: Commands) {
    debug!("stopping {}", NAME);
    commands.remove_resource::<Contracts>();
}

// helper functions
fn initial_state(contract_type: ContractType) -> ContractState {
    match contract_type {
        ContractType::SecureSupplies => {
            ContractState::SecureSupplies(SecureSuppliesState::default())
        }
        ContractType::SecureIntel => todo!(),
        ContractType::EliminateHVT => todo!(),
        ContractType::DestroySupplies => todo!(),
        ContractType::RescueHostage => todo!(),
        ContractType::RaidWeaponStash => todo!(),
        ContractType::CargoDelivery => todo!(),
        ContractType::CargoShipment => todo!(),
        ContractType::SecureNuclearMaterials => todo!(),
        ContractType::SignalIntelligence => todo!(),
        ContractType::HuntSquad => todo!(),
    }
}

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
