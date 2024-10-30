use std::f32::consts::PI;

use bevy::app::Plugin;
use bevy_inspector_egui::inspector_options::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::exfil::Operator;
use crate::fake_level::start_fake_level;
use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "follow";

// Plugin
pub struct FollowPlugin;

impl Plugin for FollowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_follow_system.after(start_fake_level))
            .add_systems(
                Update,
                (update_follow_system).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_follow_system);
    }
}

// Components
#[derive(Component, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct FollowTarget(Entity);

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Zombie;

// Resources

// Events

// Systems
fn start_follow_system(
    mut commands: Commands,
    mut zombies: Query<Entity, (With<Zombie>, Without<Operator>)>,
    operators: Query<Entity, (With<Operator>, Without<Zombie>)>,
) {
    debug!("starting {}", NAME);
    for zombie in zombies.iter_mut() {
        debug!("zombie {}", zombie);
        let operator = operators.get_single();
        if let Ok(operator) = operator {
            debug!("operator {}", operator);
            commands.entity(zombie).insert(FollowTarget(operator));
        }
    }
}

fn update_follow_system(
    mut followers: Query<(&FollowTarget, &GlobalTransform, &mut Transform)>,
    targets: Query<(Entity, &GlobalTransform), Without<FollowTarget>>,
) {
    debug!("updating {}", NAME);
    for mut follower in followers.iter_mut() {
        if let Ok(target) = targets.get((follower.0).0) {
            // factor for translation/rotation differrence
            let factor = 0.01;
            //let rot_factor = 0.1;

            // translation
            let target_trans = target.1.translation();
            let follower_trans = follower.1.translation();
            let difference_trans = target_trans - follower_trans;
            debug!("difference trans: {}", difference_trans);

            if difference_trans.length() > 2.0 {
                debug!("factor {}", factor);
                debug!("difference * factor {}", difference_trans * factor);
                follower.2.translation.x += difference_trans.x * factor;
                follower.2.translation.z += difference_trans.z * factor;
            }

            // angle
            let angle = -(PI / 2.0) - Vec2::new(difference_trans.x, difference_trans.z).to_angle();
            follower.2.rotation = Quat::from_axis_angle(Vec3::Y, angle);
        }
    }
}

fn bye_follow_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    //#[test]
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
