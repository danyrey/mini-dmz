use bevy::app::Plugin;

use crate::inventory::Inventory;
use crate::loot::{Loot, Price, Stackable};
use crate::wallet::StowedMoney;
use crate::AppState;
use crate::{exfil::Operator, inventory::StowedLoot};
use bevy::prelude::*;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

// Constants
const NAME: &str = "backpack_summary";

// Plugin
/// This plugin is related to the cash summary of the backpack.
/// all cash and all valuables will be summarized for display.
pub struct BackpackSummaryPlugin;

impl Plugin for BackpackSummaryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BackpackSummary>().add_systems(
            Update,
            (on_stowed_money, on_stowed_loot).run_if(in_state(AppState::Raid)),
        );
    }
}

// Components
#[derive(Component, Debug, PartialEq, Reflect, InspectorOptions, Default)]
#[reflect(Component, InspectorOptions)]
pub struct BackpackSummary(u32);

// Resources

// Events

// Systems
fn on_stowed_money(
    mut events: EventReader<StowedMoney>,
    mut operators: Query<&mut BackpackSummary, With<Operator>>,
) {
    for event in events.read() {
        debug!(
            "{}: received stowed money event for {}",
            NAME, event.stowing_entity,
        );
        if let Ok(mut summary) = operators.get_mut(event.stowing_entity) {
            summary.0 += event.amount;
            debug!("new backpack summary: {}", summary.0);
        }
    }
}

fn on_stowed_loot(
    mut events: EventReader<StowedLoot>,
    mut operators: Query<&mut BackpackSummary, With<Operator>>,
    loot: Query<(&Price, Option<&Stackable>), With<Loot>>,
    inventories: Query<&Parent, With<Inventory>>,
) {
    for event in events.read() {
        debug!(
            "{}: received stowed loot event for {}",
            NAME, event.stowing_entity,
        );
        // TODO: get parent of stowing entity, which is the inventory child of operator
        if let Ok(operator) = inventories.get(event.stowing_entity) {
            if let Ok(mut summary) = operators.get_mut(operator.get()) {
                if let Ok((price, maybe_stack)) = loot.get(event.loot) {
                    if let Some(stack) = maybe_stack {
                        summary.0 += stack.current_stack * price.0;
                    } else {
                        summary.0 += price.0;
                    }
                }
                debug!("new backpack summary: {}", summary.0);
            }
        }
    }
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_update_on_money_stow() {
        // given
        let mut app = App::new();

        // when
        app.add_event::<StowedMoney>();
        app.add_systems(Update, on_stowed_money);

        let mut operator = app.world_mut().spawn(Operator);
        operator.insert(BackpackSummary::default());
        let operator_id = operator.id();

        app.world_mut()
            .resource_mut::<Events<StowedMoney>>()
            .send(StowedMoney {
                stowing_entity: operator_id,
                amount: 100,
            });
        app.update();

        // then
        assert_eq!(
            100,
            app.world().get::<BackpackSummary>(operator_id).unwrap().0,
        );
    }

    #[test]
    fn should_update_on_solo_item_priced_loot() {
        // given
        let mut app = App::new();

        // when
        app.add_event::<StowedLoot>();
        app.add_systems(Update, on_stowed_loot);

        let inventory_id = app.world_mut().spawn(Inventory).id();
        let mut operator = app.world_mut().spawn(Operator);
        operator.insert(BackpackSummary::default());
        operator.add_child(inventory_id);
        let operator_id = operator.id();

        let mut loot = app.world_mut().spawn(Loot);
        loot.insert(Price(100));
        let loot_id = loot.id();

        app.world_mut()
            .resource_mut::<Events<StowedLoot>>()
            .send(StowedLoot {
                stowing_entity: inventory_id,
                loot: loot_id,
            });
        app.update();

        // then
        assert_eq!(
            100,
            app.world().get::<BackpackSummary>(operator_id).unwrap().0,
        );
    }

    #[test]
    fn should_update_on_stacked_item_priced_loot() {
        // given
        let mut app = App::new();

        // when
        app.add_event::<StowedLoot>();
        app.add_systems(Update, on_stowed_loot);

        let inventory_id = app.world_mut().spawn(Inventory).id();
        let mut operator = app.world_mut().spawn(Operator);
        operator.insert(BackpackSummary::default());
        operator.add_child(inventory_id);
        let operator_id = operator.id();

        let mut loot = app.world_mut().spawn(Loot);
        loot.insert(Price(100));
        loot.insert(Stackable {
            max_stack: 2,
            current_stack: 2,
        });
        let loot_id = loot.id();

        app.world_mut()
            .resource_mut::<Events<StowedLoot>>()
            .send(StowedLoot {
                stowing_entity: inventory_id,
                loot: loot_id,
            });
        app.update();

        // then
        assert_eq!(
            200,
            app.world().get::<BackpackSummary>(operator_id).unwrap().0,
        );
    }
}
