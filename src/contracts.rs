use bevy::app::Plugin;

use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "contracts";

// Plugin
pub struct ContractsPlugin;

impl Plugin for ContractsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_contract_system)
            .add_systems(
                Update,
                (update_contract_system).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_contract_system);
    }
}

#[allow(dead_code)]
#[derive(Debug)]
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
enum SecureSuppliesState {
    Started,
    FirstSupplySecured,
    SecondSupplySecured,
    ThirdSupplySecured,
}

// TODO: remaining statemachines

// Components
#[allow(dead_code)]
#[derive(Component)]
pub struct ContractPhone;

// Resources
#[allow(dead_code)]
#[derive(Resource)]
pub struct Contract {
    id: u32,
    contract_type: ContractType,
    // TODO: define full contract meta data
}

// Events

#[derive(Event, Debug, PartialEq)]
pub struct StowMoney {
    pub stowing_entity: Entity,
    pub money_entity: Entity,
}

// Systems
fn start_contract_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}
fn update_contract_system() {
    debug!("updating {}", NAME);
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
