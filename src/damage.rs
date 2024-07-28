use bevy::app::Plugin;
use bevy::math::bounding::{Aabb3d, BoundingSphere};

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
enum CollisionVolume {
    Aabb(Aabb3d),
    Sphere(BoundingSphere),
}

#[derive(Component, Debug)]
struct HitBox(CollisionVolume);

#[derive(Component, Debug)]
struct HurtBox(CollisionVolume);

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
    _hitbox_query: Query<(Entity, &GlobalTransform, &HitBox)>,
    _hurtbox_query: Query<(Entity, &GlobalTransform, &HurtBox)>,
    mut _health: EventWriter<HealthDamageReceived>,
    mut _armor: EventWriter<ArmorDamageReceived>,
) {
    debug!("updating {}", NAME);
    // TODO: check for colliding hurt/hitbox combinations
}

fn bye_damage_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions
