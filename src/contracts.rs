use bevy::app::Plugin;

use crate::exfil::Operator;
use crate::interaction::InventoryInteracted;
use crate::squad::{SquadId, Squads};
use crate::wallet::ReceiveMoney;
use crate::AppState;
use crate::AppState::Raid;
use crate::{interaction::Interact, inventory::Inventory};
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
            .add_event::<ContractAccepted>()
            .add_event::<SecureSuppliesUpdated>()
            .add_event::<SecureSuppliesFinished>()
            .add_event::<FinishedContract>()
            .add_systems(OnEnter(Raid), start_contract_system)
            .add_systems(
                Update,
                (
                    interaction_contract_phone,
                    contract_accepted,
                    update_secure_supplies,
                    secure_supplies_interacted,
                    finished_secure_supply,
                )
                    .run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_contract_system);
    }
}

// Components

#[allow(dead_code)]
#[derive(Component, Copy, Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub struct ContractId(pub u32);

/// this access lock can be put on to limit access to only members of a contract
#[allow(dead_code)]
#[derive(Component, Copy, Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub struct ContractAccessLock(pub ContractId);

#[allow(dead_code)]
#[derive(Component, Debug)]
pub struct ContractPhone;

/// a precise highlight of a contract objective. marker for an entity to be used to render a location icon on tacmap or overlay hud icon in first person view
#[allow(dead_code)]
#[derive(Component, Debug)]
pub struct ContractSpotlight;

/// marks current contract objectie entity
#[allow(dead_code)]
#[derive(Component, Debug)]
pub struct CurrentContractObjective;

// TODO: maybe refactor this into its own, more generic plugin later.
/// a general, unprecise are of a contract objective around its general position.
/// important: the center of the area does not represent its actual location, only that somewhere within the radius the objective is to be found.
#[allow(dead_code)]
#[derive(Component, Debug)]
pub struct ContractGeneralArea {
    /// radius around the objective in meters
    radius: f32,
}

impl Default for ContractGeneralArea {
    fn default() -> Self {
        ContractGeneralArea { radius: 15.0 }
    }
}

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
#[derive(Default, Copy, Clone, Reflect, InspectorOptions, Debug, PartialEq)]
#[reflect(InspectorOptions)]
pub enum ContractState {
    #[default]
    Started,
    SecureSupplies(SecureSuppliesState),
    Cancelled,
    Finished,
}

#[allow(dead_code)]
#[derive(Default, Copy, Clone, Reflect, InspectorOptions, Debug, PartialEq)]
#[reflect(InspectorOptions)]
pub enum SecureSuppliesState {
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
    pub contract_type: ContractType,
    pub contract_state: ContractState,
    pub contract_payout: u32,
}

#[derive(Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Contracts {
    pub map: HashMap<ContractId, Contract>,
}

// Events

#[derive(Event, Debug, PartialEq)]
pub struct ContractPhoneInteracted {
    pub contract_id: ContractId,
    pub operator_entity: Entity,
}

#[derive(Event, Debug, PartialEq)]
pub struct ContractAccepted {
    pub contract_id: ContractId,
}

#[derive(Event, Debug, PartialEq)]
pub struct SecureSuppliesUpdated {
    pub global_transform: GlobalTransform,
    pub contract_id: ContractId,
}

#[derive(Event, Debug, PartialEq)]
pub struct SecureSuppliesFinished {
    pub contract_id: ContractId,
}

#[derive(Event, Debug, PartialEq)]
pub struct FinishedContract {
    pub contract_id: ContractId,
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

fn contract_accepted(
    mut commands: Commands,
    mut contract_accepted: EventReader<ContractAccepted>,
    phones: Query<(Entity, &ContractId, &GlobalTransform), With<ContractPhone>>,
    contracts: Res<Contracts>,
    mut secure_supplies_updated: EventWriter<SecureSuppliesUpdated>,
) {
    for accepted in contract_accepted.read() {
        phones
            .iter()
            .map(|(phone, id, transform)| (phone, *id, *transform))
            .filter(|(_, id, _)| accepted.contract_id.eq(id))
            .for_each(|(phone, contract_id, global_transform)| {
                commands.entity(phone).despawn_recursive();
                match contracts.map.get(&contract_id) {
                    Some(contract) => match contract.contract_type {
                        ContractType::SecureSupplies => {
                            secure_supplies_updated.send(SecureSuppliesUpdated {
                                global_transform,
                                contract_id,
                            });
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
                    },
                    None => todo!(),
                }
            });
    }
}

#[allow(clippy::type_complexity)]
fn update_secure_supplies(
    mut commands: Commands,
    mut update_events: EventReader<SecureSuppliesUpdated>,
    mut current_supply: Query<
        (Entity, &ContractId, &GlobalTransform),
        (
            With<Inventory>,
            With<CurrentContractObjective>,
            With<ContractSpotlight>,
        ),
    >,
    mut supplies: Query<
        (Entity, &ContractId, &GlobalTransform),
        (
            With<Inventory>,
            Without<CurrentContractObjective>,
            Without<ContractSpotlight>,
        ),
    >,
    contracts: Res<Contracts>,
    mut finish: EventWriter<SecureSuppliesFinished>,
) {
    for event in update_events.read() {
        if let Some(contract) = contracts.map.get(&event.contract_id) {
            current_supply
                .iter_mut()
                .filter(|supply| supply.1.eq(&event.contract_id))
                .map(|(entity, _, transform)| (entity, transform))
                .for_each(|(entity, _global_transform)| {
                    commands.entity(entity).remove::<ContractSpotlight>();
                    commands.entity(entity).remove::<CurrentContractObjective>();
                });

            match contract.contract_state {
                ContractState::SecureSupplies(SecureSuppliesState::Started)
                | ContractState::SecureSupplies(SecureSuppliesState::FirstSupplySecured)
                | ContractState::SecureSupplies(SecureSuppliesState::SecondSupplySecured) => {
                    debug!("update secure supplies contract");
                    supplies
                        .iter_mut()
                        .filter(|supply| supply.1.eq(&event.contract_id))
                        .map(|(entity, _, transform)| (entity, transform))
                        // TODO: add current objective and spotlight on the nearest inventory, just take the first one for now
                        .take(1)
                        .for_each(|(entity, _global_transform)| {
                            commands.entity(entity).insert(ContractSpotlight);
                            commands.entity(entity).insert(CurrentContractObjective);
                        });
                }
                ContractState::SecureSupplies(SecureSuppliesState::ThirdSupplySecured) => {
                    finish.send(SecureSuppliesFinished {
                        contract_id: event.contract_id,
                    });
                }
                _ => (),
            }
        }
    }
}

/// payout to each squad member, finish contract
fn finished_secure_supply(
    mut events: EventReader<SecureSuppliesFinished>,
    contracts: Res<Contracts>,
    squads: Res<Squads>,
    mut squad_operators: Query<(Entity, &SquadId), With<Operator>>,
    mut receive_money: EventWriter<ReceiveMoney>,
    mut finished_contract: EventWriter<FinishedContract>,
) {
    for finish in events.read() {
        debug!("secure supplies contract considered finished, trigger payout and finish the contract state!");
        if let (Some(contract), contract_id) =
            (contracts.map.get(&finish.contract_id), finish.contract_id)
        {
            if let Some(squad) = squads
                .map
                .iter()
                .filter(|(_, squad)| squad.current_contract.is_some())
                .map(|(id, squad)| (id, squad.current_contract.unwrap()))
                .find(|(_, current_contract)| current_contract.eq(&contract_id))
            {
                squad_operators
                    .iter_mut()
                    .filter(|(_operator, squad_id)| (*squad_id).eq(squad.0))
                    .for_each(|(operator, _squad_id)| {
                        receive_money.send(ReceiveMoney {
                            amount: contract.contract_payout,
                            receiver: operator,
                        });
                        finished_contract.send(FinishedContract { contract_id });
                    });
            }
        }
    }
}

/// checks for squad/contract and removes spotlight/current marker and assign it to one remaining supply
// TODO: decide where to implement the check for the current contract holder. InventoryInteracted
// seems a bit late to do this. see ContractAccessLock component.
#[allow(clippy::type_complexity)]
fn secure_supplies_interacted(
    mut commands: Commands,
    mut interacted: EventReader<InventoryInteracted>,
    supplies: Query<
        (
            Entity,
            &ContractId,
            Option<&ContractSpotlight>,
            Option<&CurrentContractObjective>,
            &GlobalTransform,
        ),
        With<Inventory>,
    >,
    squads: Res<Squads>,
    mut contracts: ResMut<Contracts>,
    operators: Query<(Entity, &SquadId), With<Operator>>,
    mut update_event: EventWriter<SecureSuppliesUpdated>,
) {
    for interact in interacted.read() {
        if let (
            Ok((supply, contract_id, spotlight, current, global_transform)),
            Ok((_, squad_id)),
        ) = (
            supplies.get(interact.interaction_inventory),
            operators.get(interact.operator),
        ) {
            if let Some(squad) = squads.map.get(squad_id) {
                if squad
                    .current_contract
                    .filter(|c| c.eq(contract_id))
                    .is_some()
                {
                    // remove contract id as it was interacted with
                    commands.entity(supply).remove::<ContractId>();

                    if let Some(contract) = contracts.map.get_mut(contract_id) {
                        if let ContractState::SecureSupplies(mut state) = &contract.contract_state {
                            contract.contract_state = state.next();
                        }
                    }

                    if spotlight.is_some() && current.is_some() {
                        commands.entity(supply).remove::<ContractSpotlight>();
                        commands.entity(supply).remove::<CurrentContractObjective>();
                    }

                    update_event.send(SecureSuppliesUpdated {
                        global_transform: *global_transform,
                        contract_id: *contract_id,
                    });
                }
            }
        }
    }
}

// TODO: maybe implement a system for each other contract that reacts to state changes similar to the one above

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
