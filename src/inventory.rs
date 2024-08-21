use bevy::app::Plugin;

use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "inventory";

// Plugin
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StowLoot>()
            .add_event::<StowedLoot>()
            .add_systems(OnEnter(Raid), start_inventory_system)
            .add_systems(Update, (stow_loot_system).run_if(in_state(AppState::Raid)))
            .add_systems(OnExit(AppState::Raid), bye_inventory_system);
    }
}

// Components

#[derive(Component)]
pub struct ItemSlots(pub u8);

#[derive(Component)]
pub struct WeaponSlots(pub u8);

// Resources

// Events

/// command for stowing loot
#[derive(Event, Debug, PartialEq)]
pub struct StowLoot {
    pub stowing_entity: Entity,
    pub loot: Entity,
}

/// event for stowed loot
#[derive(Event, Debug, PartialEq)]
pub struct StowedLoot {
    pub stowing_entity: Entity,
    pub loot: Entity,
}

// Systems
fn start_inventory_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

// TODO: system currently unbounded, could stow unlimited loot. needs to respect slot limits!
fn stow_loot_system(
    mut commands: Commands,
    mut command: EventReader<StowLoot>,
    mut event: EventWriter<StowedLoot>,
) {
    debug!("updating stow listener");
    for c in command.read() {
        commands.entity(c.stowing_entity).add_child(c.loot);
        event.send(StowedLoot {
            stowing_entity: c.stowing_entity,
            loot: c.loot,
        });
    }
}
fn bye_inventory_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    use crate::loot::Loot;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_stow_loot() {
        // given
        let mut app = App::new();
        app.add_event::<StowLoot>();
        app.add_event::<StowedLoot>();
        app.add_systems(Update, stow_loot_system);
        let loot_entity = app.world.spawn(Loot).id();
        let inventory_entity = app.world.spawn(ItemSlots(4)).id();
        let inventory_children = app.world.get::<Children>(inventory_entity);
        assert!(inventory_children.is_none());

        // when
        app.world.resource_mut::<Events<StowLoot>>().send(StowLoot {
            stowing_entity: inventory_entity,
            loot: loot_entity,
        });
        app.update();

        // then
        // assert inventory now has children
        let inventory_children = app.world.get::<Children>(inventory_entity);
        assert!(inventory_children.is_some());

        // assert the one child it the loot entity
        for &child in inventory_children.unwrap() {
            assert_eq!(child, loot_entity);
        }

        // assert the loot entity has not been duplicated and was really moved
        let entities = app
            .world
            .query::<(Entity, &Loot)>()
            .iter(&app.world)
            .collect::<Vec<_>>();
        assert_eq!(1, entities.len());

        // assert the event has been sent
        let stowed_loot_events = app.world.resource::<Events<StowedLoot>>();
        let mut stowed_loot_reader = stowed_loot_events.get_reader();
        let actual_stowed_loot = stowed_loot_reader.read(stowed_loot_events).next().unwrap();

        let expected_stowed_loot = StowedLoot {
            stowing_entity: inventory_entity,
            loot: loot_entity,
        };
        assert_eq!(&expected_stowed_loot, actual_stowed_loot);
    }
}
