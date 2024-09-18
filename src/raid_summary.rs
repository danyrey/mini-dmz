use bevy::app::Plugin;

use crate::exfil::ExfilExitedAO;
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
fn exit_ao_received(mut exited_ao: EventReader<ExfilExitedAO>) {
    for _ in exited_ao.read() {
        debug!("somebody exited the AO")
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
