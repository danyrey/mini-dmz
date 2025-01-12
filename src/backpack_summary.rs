use bevy::app::Plugin;

use crate::exfil::Operator;
use crate::wallet::StowedMoney;
use crate::AppState;
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
        }
    }
}

fn on_stowed_loot(
    mut events: EventReader<StowedMoney>,
    mut operators: Query<&mut BackpackSummary, With<Operator>>,
) {
    for event in events.read() {
        debug!(
            "{}: received stowed loot event for {}",
            NAME, event.stowing_entity,
        );
        if let Ok(mut _summary) = operators.get_mut(event.stowing_entity) {
            debug!("TODO");
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
        assert!(app.world().get::<BackpackSummary>(operator_id).is_some());
        assert_eq!(
            100,
            app.world().get::<BackpackSummary>(operator_id).unwrap().0,
        );
    }
}
