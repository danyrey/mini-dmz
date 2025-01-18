use bevy::app::Plugin;
use bevy::math::bounding::{Aabb3d, RayCast3d};
use bevy::render::primitives::{Aabb, Frustum};

use crate::first_person_controller::FirstPersonCamera;
use crate::inventory::{DropLoot, ItemSlot, ItemSlots, StowLoot, WeaponSlot, WeaponSlots};
use crate::loot::{Loot, LootType};
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "inventory testing";

// Plugin
pub struct InventoryTestingPlugin;

impl Plugin for InventoryTestingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (stowing, dropping).run_if(in_state(Raid)));
    }
}

// Components

// Resources

// Events

// Systems

#[allow(clippy::type_complexity)]
fn stowing(
    interact_probe: Query<(&Frustum, &GlobalTransform), With<FirstPersonCamera>>,
    loot_query: Query<(&Aabb, &GlobalTransform, &Name, Entity, &LootType), With<Loot>>,
    mut stow_command: EventWriter<StowLoot>,
    key_input: Res<ButtonInput<KeyCode>>,
    inventory_query: Query<Entity, (With<ItemSlots>, With<Parent>)>,
) {
    // TODO: refactor this using receiving Interact command event
    let probe = interact_probe.single();
    debug!("probe_results:-----------");
    let mut closest: Vec<(f32, Entity, &LootType)> = loot_query
        .iter()
        // check if loot are in camera or not
        .filter(|loot| probe.0.intersects_obb(loot.0, &loot.1.affine(), true, true))
        .filter_map(|loot| {
            debug!("probe_result: {}", loot.2);
            let looking_at_direction = probe.0.half_spaces[4].normal();
            let position = probe.1.translation();
            let r = RayCast3d::new(
                position,
                Dir3::new(looking_at_direction.into()).unwrap(),
                2.0,
            );
            let aabb3d = Aabb3d::new(loot.1.translation(), loot.0.half_extents);
            let intersects = r.aabb_intersection_at(&aabb3d);
            intersects.map(|f| (f, loot.3, loot.4))
        })
        .collect();

    closest.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let first = closest.first();
    debug!("the closest one is: {:?}", first);
    if let Ok(stowing_entity) = inventory_query.get_single() {
        if let Some((_, loot, loot_type)) = first {
            if key_input.just_released(KeyCode::KeyF) {
                debug!("stowing inventory ...");
                stow_command.send(StowLoot {
                    stowing_entity,
                    loot: *loot,
                    loot_type: (*loot_type).clone(),
                });
            }
        }
    }
}

fn dropping(
    key_input: Res<ButtonInput<KeyCode>>,
    inventory_with_items_query: Query<Entity, (With<ItemSlots>, With<Parent>)>,
    inventory_with_weapons_query: Query<Entity, (With<WeaponSlots>, With<Parent>)>,
    inventory_items_query: Query<Entity, (With<Loot>, With<ItemSlot>)>,
    inventory_weapons_query: Query<Entity, (With<Loot>, With<WeaponSlot>)>,
    mut drop_command: EventWriter<DropLoot>,
) {
    debug!("dropping {}", NAME);

    if let Ok(dropping_entity) = inventory_with_items_query.get_single() {
        debug!("have inventory");
        if let Some(loot) = (&inventory_items_query).into_iter().next() {
            debug!("have item loot");
            if key_input.just_released(KeyCode::KeyG) {
                debug!("dropping inventory ...");
                drop_command.send(DropLoot {
                    dropping_entity,
                    loot,
                });
            }
        }
    }

    if let Ok(dropping_entity) = inventory_with_weapons_query.get_single() {
        debug!("have inventory");
        if let Some(loot) = (&inventory_weapons_query).into_iter().next() {
            debug!("have weapon loot");
            if key_input.just_released(KeyCode::KeyH) {
                debug!("dropping inventory ...");
                drop_command.send(DropLoot {
                    dropping_entity,
                    loot,
                });
            }
        }
    }
}
