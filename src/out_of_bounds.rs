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
        app.add_event::<OperaterOutOfBounds>()
            .add_event::<OperaterInOfBounds>()
            .add_event::<OperaterOutOfBoundsExpired>()
            .add_systems(OnEnter(Raid), start_out_of_bounds_system)
            .add_systems(
                FixedUpdate,
                (update_out_of_bounds_system).run_if(in_state(AppState::Raid)),
            )
            .add_systems(
                FixedUpdate,
                (
                    out_of_bounds_handler,
                    in_of_bounds_handler,
                    count_down_out_of_bounds,
                )
                    .run_if(in_state(AppState::Raid)),
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

#[derive(Component)]
pub struct OutOfBoundsCountdown(Timer);

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
#[derive(Event)]
pub struct OperaterOutOfBounds {
    pub operator_entity: Entity,
}

#[derive(Event)]
pub struct OperaterInOfBounds {
    pub operator_entity: Entity,
}

#[derive(Event)]
pub struct OperaterOutOfBoundsExpired {
    pub operator_entity: Entity,
}

// Systems
fn start_out_of_bounds_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn update_out_of_bounds_system(
    mut commands: Commands,
    mut out_of_bound_event: EventWriter<OperaterOutOfBounds>,
    mut in_of_bound_event: EventWriter<OperaterInOfBounds>,
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
    for (operator_entity, transform, out_of_bounds, bounds) in &mut query {
        let mut old_out = false;
        let mut new_out = false;
        // distance to origin
        let distance = transform.translation().distance(Vec3::default());

        if let Some(b) = bounds {
            new_out = b.is_out_of_bounds(&distance);
        } else {
            let bounds2 = Bounds::Distance(default_oob_distance);
            commands.entity(operator_entity).insert(bounds2);
        }

        if let Some(mut oob) = out_of_bounds {
            old_out = oob.0;
            oob.0 = new_out;
        } else {
            commands
                .entity(operator_entity)
                .insert(OutOfBounds(old_out));
        }

        debug!("distance from origin: {}", distance);
        debug!("out of bounds: {}", new_out);

        if old_out != new_out {
            debug!(
                "change in out of bounds status: old: {}, new: {}",
                old_out, new_out
            );

            if new_out {
                out_of_bound_event.send(OperaterOutOfBounds { operator_entity });
            } else {
                in_of_bound_event.send(OperaterInOfBounds { operator_entity });
            }
        }
    }
}

fn out_of_bounds_handler(
    mut commands: Commands,
    mut out_of_bound_event: EventReader<OperaterOutOfBounds>,
) {
    for event in out_of_bound_event.read() {
        debug!(
            "received out of bound event for operator: {:?}",
            event.operator_entity
        );
        commands
            .entity(event.operator_entity)
            .insert(OutOfBoundsCountdown(Timer::from_seconds(
                3.0,
                TimerMode::Once,
            )));
    }
}

fn in_of_bounds_handler(
    mut commands: Commands,
    mut in_of_bound_event: EventReader<OperaterInOfBounds>,
) {
    for event in in_of_bound_event.read() {
        debug!(
            "received in of bound event for operator: {:?}",
            event.operator_entity
        );
        commands
            .entity(event.operator_entity)
            .remove::<OutOfBoundsCountdown>();
    }
}

fn count_down_out_of_bounds(
    mut commands: Commands,
    mut cooldowns: Query<(Entity, &mut OutOfBoundsCountdown)>,
    time: Res<Time>,
    mut out_of_bound_event: EventWriter<OperaterOutOfBoundsExpired>,
) {
    for (operator_entity, mut countdown) in &mut cooldowns {
        countdown.0.tick(time.delta());

        debug!(
            "remaining out of bound timer for entity {:?} : {:?}",
            operator_entity,
            countdown.0.remaining()
        );

        if countdown.0.finished() {
            // send out event for out of bound expired (==death)
            commands
                .entity(operator_entity)
                .remove::<OutOfBoundsCountdown>();
            // cleanup
            out_of_bound_event.send(OperaterOutOfBoundsExpired { operator_entity });
        }
    }
}

fn bye_out_of_bounds_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions
