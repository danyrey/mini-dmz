use bevy::app::Plugin;

use crate::interaction::Interact;
use crate::inventory::{
    DropLoot, Inventory, ItemSlot, ItemSlots, StowLoot, WeaponSlot, WeaponSlots,
};
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

/// TODO: not sure about the subscribe, enrich and republishing the comand/event here. it produces a 1 frame delay that might be noticable.
fn stowing(
    mut interaction_commands: EventReader<Interact>,
    backpack_query: Query<(Entity, &Parent), With<Inventory>>,
    loot_query: Query<(Entity, &LootType), With<Loot>>,
    mut stow_command: EventWriter<StowLoot>,
) {
    for command in interaction_commands.read() {
        // filter for commands on Loot entities only
        if let Ok((loot, loot_type)) = loot_query.get(command.interaction_entity) {
            backpack_query
                .iter()
                .filter(|backpack| backpack.1.get().eq(&command.operator_entity))
                .for_each(|(stowing_entity, _)| {
                    stow_command.send(StowLoot {
                        stowing_entity,
                        loot,
                        loot_type: (*loot_type).clone(),
                    });
                });
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
