use bevy::app::Plugin;

use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "projectile";

// Plugin
pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
            // register types
            .register_type::<Projectile>()
            .register_type::<ProjectileEmitter>()
            .register_type::<ProjectileCapacity>()
            // register events
            // add systems
            .add_systems(OnEnter(Raid), start_projectile_system)
            .add_systems(
                FixedUpdate,
                (update_projectile_system).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_projectile_system);
    }
}

// Components
#[derive(Component, Reflect)]
/// projectile component
/// mass for now only
pub struct Projectile {
    pub mass: u32,
}

#[derive(Component, Reflect)]
pub struct ProjectileEmitter {
    pub velocity: u32,
    pub rate: u32,
}

#[derive(Component, Reflect)]
pub struct ProjectileCapacity {
    pub capacity: u32,
}

// Resources

// Events

// Systems
fn start_projectile_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

/// note: this system runs in a FixedUpdate schedule as it is physics related
fn update_projectile_system() {
    debug!("updating {}", NAME);
}

fn bye_projectile_system(mut _commands: Commands) {
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
