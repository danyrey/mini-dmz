use bevy::app::Plugin;

use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "projectile";
const GRAVITY: f32 = 9.81;

// Plugin
pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
            // register types
            .register_type::<Projectile>()
            .register_type::<ProjectileVelocity>()
            .register_type::<ProjectileEmitter>()
            // register events
            // add systems
            .add_systems(OnEnter(Raid), start_projectile_system)
            .add_systems(
                FixedUpdate,
                (flying_projectiles).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_projectile_system);
    }
}

// Components
#[derive(Component, Reflect)]
/// projectile component, ballistic.
pub struct Projectile {
    /// mass for now only
    pub mass: u32,
}

#[derive(Component, Reflect)]
pub struct ProjectileVelocity {
    pub velocity: Vec3,
}

/// this component is attached to all entities that
/// emit projectiles of some kind
#[derive(Component, Reflect)]
pub struct ProjectileEmitter {
    /// velocity in meters per second
    pub velocity: u32,
    /// rate per second
    pub rate: u32,
}

// Resources

// Events

// Systems
fn start_projectile_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

/// note: this system runs in a FixedUpdate schedule as it is physics related
fn flying_projectiles(
    time: Res<Time>,
    mut projectiles: Query<(Entity, &Projectile, &ProjectileVelocity, &mut Transform)>,
) {
    debug!("updating {}", NAME);
    for (_entity, _projectile, _velocity, mut transform) in projectiles.iter_mut() {
        // TODO: fly, you fools!
        // Take projectile speed & velocity and apply it to transform and velocity
        // update
        // FIXME: only move along x axis as MVP
        transform.translation.x += 0.01 * time.elapsed_secs()
    }
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
