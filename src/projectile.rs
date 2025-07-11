use std::time::Duration;

use bevy::app::Plugin;

use crate::AppState;
use bevy::prelude::*;

// Constants
const NAME: &str = "projectile";
const _GRAVITY: f32 = 9.81;
/// assume 9mm as a default which would be around 300 m/s
const BULLET_VELOCITY: u32 = 300;
/// default bullet mass, 9mm assumed
const BULLET_MASS: u32 = 8;
/// time to live in seconds
const BULLET_TTL: f32 = 3.0;
/// rate of fire per second
const RATE_OF_FIRE: u32 = 13;

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
            .add_event::<SingleShot>()
            // add systems
            .add_systems(
                FixedUpdate,
                (emit_single_shot, flying_projectiles, projectile_timers)
                    .run_if(in_state(AppState::Raid)),
            );
    }
}

// Components
#[derive(Component, Reflect)]
/// projectile component, ballistic.
pub struct Projectile {
    /// mass in g for now only
    pub mass: u32,
    // TODO: ballistic coefficient here
}

/// represents a 9mm projectile
impl Default for Projectile {
    fn default() -> Self {
        Projectile { mass: BULLET_MASS }
    }
}

#[derive(Component, Reflect)]
pub struct ProjectileTime {
    pub timer: Timer,
}

impl Default for ProjectileTime {
    fn default() -> Self {
        ProjectileTime {
            timer: Timer::new(Duration::from_secs_f32(BULLET_TTL), TimerMode::Once),
        }
    }
}

/// projectile velocity direction per second
#[derive(Component, Reflect)]
pub struct ProjectileVelocity {
    pub velocity: Vec3,
}

impl Default for ProjectileVelocity {
    fn default() -> Self {
        ProjectileVelocity {
            velocity: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0 * BULLET_VELOCITY as f32,
            },
        }
    }
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

/// represents defaults for a mp5 smg
impl Default for ProjectileEmitter {
    fn default() -> Self {
        ProjectileEmitter {
            velocity: BULLET_VELOCITY,
            rate: RATE_OF_FIRE,
        }
    }
}

// Resources

// Events
#[derive(Event, Debug, PartialEq)]
pub struct SingleShot {
    pub shooter: Entity,
}

// Systems
fn emit_single_shot(
    mut commands: Commands,
    mut single_shot_triggered: EventReader<SingleShot>,
    projectile_emitters: Query<(&Parent, &ProjectileEmitter, &GlobalTransform)>,
) {
    // TODO: prevent fire rate that is not possible
    for event in single_shot_triggered.read() {
        for (shooter, pewpew, g_transform) in projectile_emitters.iter() {
            if shooter.get().eq(&event.shooter) {
                commands
                    .spawn(Projectile::default())
                    .insert(Name::new("Bullet"))
                    .insert(ProjectileTime::default())
                    .insert(Transform::from(g_transform.clone()))
                    .insert(ProjectileVelocity {
                        velocity: g_transform.forward() * pewpew.velocity as f32,
                    });
            }
        }
    }
}

/// note: this system runs in a FixedUpdate schedule as it is physics related
fn flying_projectiles(
    time: Res<Time<Fixed>>,
    mut projectiles: Query<(&Projectile, &ProjectileVelocity, &mut Transform)>,
) {
    debug!("updating {}", NAME);
    for (_projectile, velocity, mut transform) in projectiles.iter_mut() {
        // TODO: gravity
        // TODO: drag, slowing down velocity
        transform.translation += velocity.velocity * time.delta_secs();
    }
}

/// this system is just managing a projectiles lifetime and
/// despanws entities upon its end
fn projectile_timers(
    mut commands: Commands,
    mut q: Query<(Entity, &mut ProjectileTime)>,
    time: Res<Time<Fixed>>,
) {
    for (entity, mut projectile_timer) in q.iter_mut() {
        // timers gotta be ticked, to work
        projectile_timer.timer.tick(time.delta());

        // if it finished, despawn the bomb
        if projectile_timer.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
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
