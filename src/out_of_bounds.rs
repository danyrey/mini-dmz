use core::fmt;

use bevy::app::Plugin;

use crate::exfil::Operator;
use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "out of bounds";

// Plugin
pub struct OutOfBoundsPlugin;

impl Plugin for OutOfBoundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_out_of_bounds_system)
            .add_systems(
                FixedUpdate,
                (update_out_of_bounds_system).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_out_of_bounds_system);
    }
}

// Components
#[derive(Component, Deref, DerefMut, Default, Debug)]
pub struct OutOfBounds(bool);

#[derive(Component, Default, Debug)]
pub enum Bounds {
    /// no bounds, roam freely
    #[default]
    None,
    /// distance from origin
    Distance(f32),
}

trait IsOutOfBounds {
    fn is_out_of_bounds(&self, distance: &f32) -> bool;
}

impl fmt::Display for Bounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self)
    }
}

impl IsOutOfBounds for Bounds {
    fn is_out_of_bounds(&self, distance: &f32) -> bool {
        match self {
            Bounds::None => false,
            Bounds::Distance(d) => distance > d,
        }
    }
}

// Resources

// Events

// Systems
fn start_out_of_bounds_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn update_out_of_bounds_system(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &GlobalTransform,
            Option<&mut OutOfBounds>,
            Option<&mut Bounds>,
        ),
        (With<Operator>, With<GlobalTransform>),
    >,
) {
    debug!("updating {}", NAME);
    let default_oob_distance = 15.0;
    for (entity, transform, out_of_bounds, bounds) in &mut query {
        let mut out2 = false;
        // distance to origin
        let distance = transform.translation().distance(Vec3::default());

        if let Some(b) = bounds {
            out2 = b.is_out_of_bounds(&distance);
        } else {
            let bounds2 = Bounds::Distance(default_oob_distance);
            commands.entity(entity).insert(bounds2);
        }

        if let Some(mut oob) = out_of_bounds {
            oob.0 = out2;
        } else {
            commands.entity(entity).insert(OutOfBounds(out2));
        }

        debug!("distance from origin: {}", distance);
        debug!("out of bounds: {}", out2);
        // TODO: what to do when out of bounds? timer
        // TODO: define events
    }
}

fn bye_out_of_bounds_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions
