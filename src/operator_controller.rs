use bevy::app::Plugin;

use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// TODO: controller actions:
// * move
// * jump
// * interact
// * crouch
// * shoot
// * melee
// * aim down sites
// * ... more TBD

// Constants
const NAME: &str = "operator_controller";

// Plugin
pub struct OperatorControllerPlugin;

impl Plugin for OperatorControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_operator_controller_system)
            .add_systems(
                Update,
                (update_operator_controller_system).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_operator_controller_system);
    }
}

// Components
#[derive(Component)]
struct OperatorController;

// Resources

// Events

// Systems
fn start_operator_controller_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}
fn update_operator_controller_system() {
    debug!("updating {}", NAME);
}
fn bye_operator_controller_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

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
