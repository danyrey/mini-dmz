use bevy::app::Plugin;

use crate::exfil::Operator;
use crate::first_person_controller::PlayerControlled;
use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "compass";

// Plugin
pub struct CompassPlugin;

impl Plugin for CompassPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_compass_system)
            .add_systems(
                Update,
                (update_compass_system).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_compass_system);
    }
}

// Components
#[derive(Component)]
pub struct Compass;

#[derive(Debug)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl From<i32> for Direction {
    fn from(value: i32) -> Self {
        let angle = value % 360;
        match angle {
            0..=22 => Self::N,
            23..=67 => Self::NE,
            68..=112 => Self::E,
            113..=157 => Self::SE,
            158..=202 => Self::S,
            203..=247 => Self::SW,
            248..=292 => Self::W,
            293..=337 => Self::NW,
            338..=359 => Self::N,
            _ => Self::N,
        }
    }
}

// Resources

// Events

// Systems
fn start_compass_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

#[allow(clippy::type_complexity)]
fn update_compass_system(
    operator_query: Query<
        &GlobalTransform,
        (With<Compass>, With<Operator>, With<PlayerControlled>),
    >,
) {
    for o in operator_query.iter() {
        debug!("updating {}", NAME);
        let angle = -o
            .to_scale_rotation_translation()
            .1
            .to_euler(EulerRot::YXZ)
            .0
            .to_degrees() as i32;
        let angle = if angle < 0 { 360 + angle } else { angle };
        debug!("compass angle {}", angle);
        debug!("compass direction {:?}", Direction::from(angle));
    }
}

fn bye_compass_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

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
}
