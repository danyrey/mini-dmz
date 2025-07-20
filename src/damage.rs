use std::cmp::max;

use bevy::app::Plugin;
use bevy::math::bounding::{Aabb3d, BoundingVolume, IntersectsVolume};

use crate::armor::Armor;
use crate::health::Health;
use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;

// Constants
const NAME: &str = "damage";

// Plugin
pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app
            // types
            .register_type::<Damage>()
            .register_type::<DamageOrigin>()
            .register_type::<HurtBox>()
            .register_type::<HitBox>()
            // systems
            .add_systems(OnEnter(Raid), start_damage_system)
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

/// component that deals the damage
#[derive(Component, Debug, Reflect, InspectorOptions)]
pub struct HitBox(pub Aabb3d);

/// component that receives the damage(hurt)
#[derive(Component, Debug, Reflect, InspectorOptions)]
pub struct HurtBox(pub Aabb3d);

/// damage component
#[derive(Component, Debug, PartialEq, Reflect, InspectorOptions)]
pub struct Damage(pub i32);

/// origin of damage
#[derive(Component, Debug, PartialEq, Reflect, InspectorOptions)]
pub struct DamageOrigin(pub Entity);

// Resources

// Events
#[derive(Event, Debug, PartialEq)]
pub struct ArmorDamageReceived {
    pub entity: Entity,
    pub damage: i32,
    pub dealer: Option<Entity>,
}

#[derive(Event, Debug, PartialEq)]
pub struct HealthDamageReceived {
    pub entity: Entity,
    pub damage: i32,
    pub dealer: Option<Entity>,
}

// Systems
fn start_damage_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

#[allow(clippy::type_complexity)]
fn update_damage_system(
    mut hitbox_query: Query<(
        Entity,
        &HitBox,
        &Damage,
        &GlobalTransform,
        Option<&DamageOrigin>,
    )>,
    mut hurtbox_query: Query<(
        Entity,
        &HurtBox,
        Option<&Health>,
        Option<&Armor>,
        &GlobalTransform,
    )>,
    mut health_sender: EventWriter<HealthDamageReceived>,
    mut armor_sender: EventWriter<ArmorDamageReceived>,
    mut commands: Commands,
) {
    debug!("updating {}", NAME);
    for (hit_entity, hitbox, damage, hit_transform, damage_origin) in hitbox_query.iter_mut() {
        let transformed_hit_box = Aabb3d::new(hit_transform.translation(), hitbox.0.half_size());
        for (hurt_entity, hurtbox, health, armor, hurt_transform) in hurtbox_query.iter_mut() {
            let transformed_hurt_box =
                Aabb3d::new(hurt_transform.translation(), hurtbox.0.half_size());
            debug!("we have a potential hit on entity({hurt_entity}) from entity({hit_entity})");
            // dont hit yourself if overlap occours
            if hit_entity != hurt_entity && transformed_hit_box.intersects(&transformed_hurt_box) {
                let dealer = damage_origin.map(|dealer_ref| dealer_ref.0);
                debug!("we have a definite hit on entity({hurt_entity}) from entity({hit_entity}), original damage dealer: ({0})", dealer.map_or(String::from("-"), |dealer| dealer.to_string()));
                let mut remaining_damage = damage.0;
                if let Some(a) = armor {
                    let x = max(0, a.0 - damage.0);
                    let y = a.0 - x;
                    armor_sender.send(ArmorDamageReceived {
                        entity: hurt_entity,
                        damage: y,
                        dealer,
                    });
                    remaining_damage = damage.0 - y;
                }
                if let Some(h) = health {
                    let x = max(0, h.0 - remaining_damage);
                    let y = h.0 - x;
                    health_sender.send(HealthDamageReceived {
                        entity: hurt_entity,
                        damage: y,
                        dealer,
                    });
                    remaining_damage -= y;
                }
                debug!("remaining_damage: {}", remaining_damage);
                commands.entity(hit_entity).remove::<Damage>();
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
    use std::borrow::BorrowMut;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_update_armor_partial_damage() {
        // given
        let mut app = App::new();

        // when
        app.add_event::<ArmorDamageReceived>();
        app.add_event::<HealthDamageReceived>();
        app.add_systems(Update, update_damage_system);
        let hit_entity = app
            .borrow_mut()
            .world_mut()
            .spawn((
                HitBox(Aabb3d::new(
                    Vec3::default(),
                    Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                )),
                Damage(10),
                Transform::default(),
            ))
            .id();
        let hurt_entity = app
            .borrow_mut()
            .world_mut()
            .spawn((
                HurtBox(Aabb3d::new(
                    Vec3::default(),
                    Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                )),
                Armor(100),
                Health(100),
                Transform::default(),
            ))
            .id();

        app.update();

        // then
        let armor_damage_received_events = app.world().resource::<Events<ArmorDamageReceived>>();
        let mut armor_damage_received_reader = armor_damage_received_events.get_cursor();
        let armor_damage_received = armor_damage_received_reader
            .read(armor_damage_received_events)
            .next();

        // Check the event has been sent and damage component has been removed
        assert_eq!(
            Some(&ArmorDamageReceived {
                entity: hurt_entity,
                damage: 10,
                dealer: Option::None,
            }),
            armor_damage_received
        );
        assert!(app.world().get::<Damage>(hit_entity).is_none());
    }

    #[test]
    fn should_update_full_armor_and_partial_health_damage() {
        // given
        let mut app = App::new();

        // when
        app.add_event::<ArmorDamageReceived>();
        app.add_event::<HealthDamageReceived>();
        app.add_systems(Update, update_damage_system);
        let hit_entity = app
            .borrow_mut()
            .world_mut()
            .spawn((
                HitBox(Aabb3d::new(
                    Vec3::default(),
                    Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                )),
                Damage(110),
                Transform::default(),
            ))
            .id();
        let hurt_entity = app
            .borrow_mut()
            .world_mut()
            .spawn((
                HurtBox(Aabb3d::new(
                    Vec3::default(),
                    Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                )),
                Armor(100),
                Health(100),
                Transform::default(),
            ))
            .id();

        app.update();

        // then
        let armor_damage_received_events = app.world().resource::<Events<ArmorDamageReceived>>();
        let mut armor_damage_received_reader = armor_damage_received_events.get_cursor();
        let armor_damage_received = armor_damage_received_reader
            .read(armor_damage_received_events)
            .next();

        let health_damage_received_events = app.world().resource::<Events<HealthDamageReceived>>();
        let mut health_damage_received_reader = health_damage_received_events.get_cursor();
        let health_damage_received = health_damage_received_reader
            .read(health_damage_received_events)
            .next();

        // Check the event has been sent and damage component has been removed
        assert_eq!(
            Some(&ArmorDamageReceived {
                entity: hurt_entity,
                damage: 100,
                dealer: Option::None,
            }),
            armor_damage_received
        );
        assert_eq!(
            Some(&HealthDamageReceived {
                entity: hurt_entity,
                damage: 10,
                dealer: Option::None,
            }),
            health_damage_received
        );
        assert!(app.world().get::<Damage>(hit_entity).is_none());
    }

    #[test]
    fn should_update_armor_and_health_full_damage() {
        // given
        let mut app = App::new();

        // when
        app.add_event::<ArmorDamageReceived>();
        app.add_event::<HealthDamageReceived>();
        app.add_systems(Update, update_damage_system);
        let hit_entity = app
            .borrow_mut()
            .world_mut()
            .spawn((
                HitBox(Aabb3d::new(
                    Vec3::default(),
                    Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                )),
                Damage(210),
                Transform::default(),
            ))
            .id();
        let hurt_entity = app
            .borrow_mut()
            .world_mut()
            .spawn((
                HurtBox(Aabb3d::new(
                    Vec3::default(),
                    Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                )),
                Armor(100),
                Health(100),
                Transform::default(),
            ))
            .id();

        app.update();

        // then
        let armor_damage_received_events = app.world().resource::<Events<ArmorDamageReceived>>();
        let mut armor_damage_received_reader = armor_damage_received_events.get_cursor();
        let armor_damage_received = armor_damage_received_reader
            .read(armor_damage_received_events)
            .next();

        let health_damage_received_events = app.world().resource::<Events<HealthDamageReceived>>();
        let mut health_damage_received_reader = health_damage_received_events.get_cursor();
        let health_damage_received = health_damage_received_reader
            .read(health_damage_received_events)
            .next();

        // Check the event has been sent and damage component has been removed
        assert_eq!(
            Some(&ArmorDamageReceived {
                entity: hurt_entity,
                damage: 100,
                dealer: Option::None,
            }),
            armor_damage_received
        );
        assert_eq!(
            Some(&HealthDamageReceived {
                entity: hurt_entity,
                damage: 100,
                dealer: Option::None,
            }),
            health_damage_received
        );
        assert!(app.world().get::<Damage>(hit_entity).is_none());
    }
}
