use bevy::app::Plugin;

use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "death";

// Plugin
/// plugin to deal with death related components/systems/events
pub struct DeathPlugin;

impl Plugin for DeathPlugin {
    fn build(&self, app: &mut App) {
        app
            // events
            .add_event::<EntityDie>()
            .add_event::<EntityDied>()
            // systems
            .add_systems(OnEnter(Raid), start_death)
            .add_systems(
                Update,
                (update_dying, update_death).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_death);
    }
}

// Components

// Resources

// Events
#[derive(Event, Debug, PartialEq)]
pub struct EntityDie {
    pub dying: Entity,
    pub killer: Option<Entity>,
}

#[derive(Event, Debug, PartialEq)]
pub struct EntityDied {
    pub death: Entity,
    pub killer: Option<Entity>,
}

// Systems
fn start_death(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

/// system for book keeping.
fn update_dying(mut dying: EventReader<EntityDie>, mut deaths: EventWriter<EntityDied>) {
    debug!("updating {}", NAME);
    for event in dying.read() {
        debug!("somebody is about to die: {}", event.dying);
        // FIXME: for now just relay the event, could be more sophisticated later
        deaths.send(EntityDied {
            death: event.dying,
            killer: event.killer,
        });
    }
}

/// for now just despawning entityies. will be the job of other systems in the future.
fn update_death(mut deaths: EventReader<EntityDied>, mut commands: Commands) {
    debug!("updating {}", NAME);
    for event in deaths.read() {
        debug!("somebody died: {}", event.death);
        // FIXME: for now just despawn entity recursively, but in the future different
        // plugins handle the despawning on their own
        commands.entity(event.death).despawn_recursive();
        debug!("entity despanwed: {}", event.death);
    }
}

fn bye_death(mut _commands: Commands) {
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
