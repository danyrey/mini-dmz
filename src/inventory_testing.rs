use bevy::app::Plugin;

use crate::inventory::{ItemSlot, ItemSlots, StowLoot};
use crate::loot::{ItemType, Loot, LootType};
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "inventory testing";

// Plugin
pub struct InventoryTestingPlugin;

impl Plugin for InventoryTestingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_inventory_testing).run_if(in_state(Raid)));
    }
}

// Components

// Resources

// Events

// Systems

fn update_inventory_testing(
    key_input: Res<ButtonInput<KeyCode>>,
    inventory_query: Query<Entity, With<ItemSlots>>,
    loot_query: Query<(Entity, &LootType), (With<Loot>, Without<ItemSlot>)>,
    mut stow_command: EventWriter<StowLoot>,
) {
    debug!("updating {}", NAME);

    if let Ok(stowing_entity) = inventory_query.get_single() {
        debug!("have inventory");
        for (loot, loot_type) in &loot_query {
            debug!("have loot");
            if key_input.just_released(KeyCode::KeyF) {
                debug!("stowing inventory ...");
                stow_command.send(StowLoot {
                    stowing_entity,
                    loot,
                    loot_type: loot_type.clone(),
                });
            }
            break;
        }
    }
}
