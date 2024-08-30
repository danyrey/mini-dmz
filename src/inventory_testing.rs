use bevy::app::Plugin;

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
    key_input: Res<ButtonInput<KeyCode>>,
    inventory_query: Query<Entity, With<ItemSlots>>,
    loot_query: Query<(Entity, &LootType), (With<Loot>, Without<ItemSlot>)>, // TODO?: weapons
    mut stow_command: EventWriter<StowLoot>,
) {
    debug!("stowing {}", NAME);

    if let Ok(stowing_entity) = inventory_query.get_single() {
        debug!("have inventory");
        if let Some((loot, loot_type)) = (&loot_query).into_iter().next() {
            debug!("have loot");
            if key_input.just_released(KeyCode::KeyF) {
                debug!("stowing inventory ...");
                stow_command.send(StowLoot {
                    stowing_entity,
                    loot,
                    loot_type: loot_type.clone(),
                });
            }
        }
    }
}

fn dropping(
    key_input: Res<ButtonInput<KeyCode>>,
    inventory_with_items_query: Query<Entity, With<ItemSlots>>,
    inventory_with_weapons_query: Query<Entity, With<WeaponSlots>>,
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
