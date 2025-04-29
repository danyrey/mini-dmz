use bevy::app::Plugin;
use bevy::utils::HashMap;

use crate::squad::SquadId;
use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

// Constants
const NAME: &str = "spawn";

#[derive(Reflect)]
pub enum Formation {
    Triangle,
    Staggered,
    Line,
}

// Plugin
pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_spawn)
            .add_systems(Update, (update_spawn).run_if(in_state(AppState::Raid)))
            .add_systems(OnExit(AppState::Raid), bye_spawn);
    }
}

// Components

#[derive(Component, Reflect)]
pub struct Spawn {
    pub direction: Dir3,
    pub formation: Formation,
}

#[derive(Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Spawns {
    pub map: HashMap<SquadId, Spawn>,
}

// Resources

// Events

// Systems
fn start_spawn(
    mut _commands: Commands,
    _spawn_added: Query<Entity, Added<Spawn>>,
    mut _gizmos: Gizmos,
) {
    debug!("starting {}", NAME);
    // TODO: debug gizmos and geometry
}

fn update_spawn(
    mut _commands: Commands,
    spawns: Query<(&Spawn, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    debug!("updating {}", NAME);

    for (_spawn, global_transform) in spawns.iter() {
        if cfg!(debug_assertions) {
            debug!("Debugging enabled");
            gizmos.arrow(
                global_transform.translation(),
                global_transform.translation() + Vec3::X,
                Srgba::rgb(0.0, 1.00, 0.0),
            );
        }
    }
}

fn bye_spawn(mut _commands: Commands) {
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
