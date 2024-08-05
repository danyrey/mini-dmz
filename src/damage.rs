use bevy::app::Plugin;
use bevy::math::bounding::{Aabb3d, BoundingSphere};

use crate::armor::Armor;
use crate::health::Health;
use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "damage";

// Plugin
pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_damage_system)
            .add_systems(
                Update,
                (update_damage_system).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_damage_system)
            .add_event::<ArmorDamageReceived>()
            .add_event::<HealthDamageReceived>();
    }
}

// Components
#[derive(Debug)]
pub enum CollisionVolume {
    Aabb(Aabb3d),
    Sphere(BoundingSphere),
}

/// component that deals the damage
#[derive(Component, Debug)]
pub struct HitBox(pub CollisionVolume);

/// component that receives the damage(hurt)
#[derive(Component, Debug)]
pub struct HurtBox(pub CollisionVolume);

/// damage component
#[derive(Component, Debug)]
pub struct Damage(u8);

// Resources

// Events
#[derive(Event)]
pub struct ArmorDamageReceived {
    pub entity: Entity,
    pub damage: u8,
}

#[derive(Event)]
pub struct HealthDamageReceived {
    pub entity: Entity,
    pub damage: u8,
}

// Systems
fn start_damage_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn update_damage_system(
    // TODO: query the components to which to decide damage events
    mut hitbox_query: Query<(Entity, &GlobalTransform, &HitBox, &Damage)>,
    mut hurtbox_query: Query<(
        Entity,
        &GlobalTransform,
        &HurtBox,
        Option<&Health>,
        Option<&Armor>,
    )>,
    mut _health: EventWriter<HealthDamageReceived>,
    mut _armor: EventWriter<ArmorDamageReceived>,
) {
    debug!("updating {}", NAME);
    // TODO: check for colliding hurt/hitbox combinations
    for (_hit_entity, _hit_transform, _hitbox, _damage) in hitbox_query.iter_mut() {
        for (_hurt_entity, _hurt_transform, _hurtbox, _health, _armor) in hurtbox_query.iter_mut() {
            // dont hit yourself if overlap occours
            if _hit_entity != _hurt_entity {
                // TODO: check for overlap and produce event
            }
        }
    }
}

fn bye_damage_system(mut _commands: Commands) {
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
