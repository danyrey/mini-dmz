use bevy::app::Plugin;
use bevy::utils::HashMap;

use crate::contracts::{ContractId, ContractPhoneInteracted};
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
        app.add_systems(OnEnter(Raid), start_squad_system)
            .add_systems(
                Update,
                (update_squad_system, contract_phone_interacted).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_squad_system);
    }
}

// Components
#[allow(dead_code)]
#[derive(Component, Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub struct SquadId(pub u32);

// Resources
#[allow(dead_code)]
#[derive(Reflect, InspectorOptions, Debug, PartialEq)]
#[reflect(InspectorOptions)]
pub struct Squad {
    max_size: u32,
}

#[derive(Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Squads {
    map: HashMap<SquadId, Squad>,
    current_contract: Option<ContractId>,
}

// Events

// TODO: squad created
// TODO: joined squad
// TODO: left squad
// TODO: squad terminated
// ...

// Systems
fn start_squad_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn update_squad_system() {
    debug!("updating {}", NAME);
}

fn contract_phone_interacted(mut contract_phone_interacted: EventReader<ContractPhoneInteracted>) {
    for event in contract_phone_interacted.read() {
        debug!("contract phone interacted {:?}", event);
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
