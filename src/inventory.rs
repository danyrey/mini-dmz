use bevy::app::Plugin;

use crate::loot::ItemType;
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
pub struct Inventory;

/// number of item slots
#[derive(Component)]
pub struct ItemSlots(pub u8);

/// position within the item slots
#[derive(Component)]
pub struct ItemSlot(pub u8);

/// number of weapon slots
#[derive(Component)]
pub struct WeaponSlots(pub u8);

/// position within the weapon slots
#[derive(Component)]
pub struct WeaponSlot(pub u8);

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
    inventories: Query<&ItemSlots, With<Inventory>>,
    inventory_items: Query<(&Parent, &ItemSlot)>,
) {
    debug!("updating stow listener");
    for c in command.read() {
        // TODO: currently no distinction between child types : items vs weapons
        let inventory = c.stowing_entity;
        let inventory_items: Vec<&ItemSlot> = inventory_items
            .iter()
            .filter(|ii| inventory == ii.0.get())
            .map(|ii| ii.1)
            .collect();
        let inventory_item_count = inventory_items.len();
        let item_slots: usize = inventories.get(inventory).map_or(0, |r| r.0.into());

        let mut target_slot: u8 = 0;

        if inventory_item_count != 0 {
            todo!();
        }

        if inventory_item_count < item_slots {
            // TODO: find next free, 0 for now
            let item_slot = ItemSlot(target_slot);
            commands.entity(c.stowing_entity).add_child(c.loot);
            commands.entity(c.loot).insert(item_slot);
            event.send(StowedLoot {
                stowing_entity: c.stowing_entity,
                loot: c.loot,
            });
        }
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
        let mut inventory = app.world.spawn(Inventory);
        inventory.insert(ItemSlots(1));
        let inventory_entity = inventory.id();
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

    #[test]
    fn should_stow_loot_and_respect_capacity() {
        // given
        let mut app = App::new();
        app.add_event::<StowLoot>();
        app.add_event::<StowedLoot>();
        app.add_systems(Update, stow_loot_system);
        let loot_entity_1 = app.world.spawn(Loot).id();
        let loot_entity_2 = app.world.spawn(Loot).id();
        let loot_entity_3 = app.world.spawn(Loot).id();
        let mut inventory = app.world.spawn(Inventory);
        inventory.insert(ItemSlots(2));
        let inventory_entity = inventory.id();
        let inventory_children = app.world.get::<Children>(inventory_entity);
        assert!(inventory_children.is_none());

        // when / then
        // first loot item should succeed
        app.world.resource_mut::<Events<StowLoot>>().send(StowLoot {
            stowing_entity: inventory_entity,
            loot: loot_entity_1,
        });
        app.update();

        // assert inventory now has 1 children
        let inventory_children = app.world.get::<Children>(inventory_entity);
        assert!(inventory_children.is_some());
        assert_eq!(1, inventory_children.unwrap().len());

        inventory_children.unwrap().iter().for_each(|c| {
            let item_slot = app.world.get::<ItemSlot>(*c);
            assert!(item_slot.is_some());
        });

        // assert the event for 1 has been sent
        let stowed_loot_events = app.world.resource::<Events<StowedLoot>>();
        let mut stowed_loot_reader = stowed_loot_events.get_reader();
        assert_eq!(1, stowed_loot_reader.len(stowed_loot_events));
        let actual_stowed_loot = stowed_loot_reader.read(stowed_loot_events).next().unwrap();

        let expected_stowed_loot = StowedLoot {
            stowing_entity: inventory_entity,
            loot: loot_entity_1,
        };
        assert_eq!(&expected_stowed_loot, actual_stowed_loot);

        // second loot item should succeed too
        app.world.resource_mut::<Events<StowLoot>>().send(StowLoot {
            stowing_entity: inventory_entity,
            loot: loot_entity_2,
        });
        app.update();

        let inventory_children = app.world.get::<Children>(inventory_entity);
        assert!(inventory_children.is_some());
        assert_eq!(2, inventory_children.unwrap().len());

        inventory_children.unwrap().iter().for_each(|c| {
            let item_slot = app.world.get::<ItemSlot>(*c);
            assert!(item_slot.is_some());
        });

        // assert the event for 2 has been sent
        let stowed_loot_events = app.world.resource::<Events<StowedLoot>>();
        let mut stowed_loot_reader = stowed_loot_events.get_reader();
        // TODO: why is the consumed previous event still in this new reader????
        assert_eq!(2, stowed_loot_reader.len(stowed_loot_events));
        stowed_loot_reader.read(stowed_loot_events).next(); // skip
        let actual_stowed_loot = stowed_loot_reader.read(stowed_loot_events).next().unwrap();

        let expected_stowed_loot = StowedLoot {
            stowing_entity: inventory_entity,
            loot: loot_entity_2,
        };
        assert_eq!(&expected_stowed_loot, actual_stowed_loot);

        // third loot item should not succeed as capacity is reached already
        app.world.resource_mut::<Events<StowLoot>>().send(StowLoot {
            stowing_entity: inventory_entity,
            loot: loot_entity_3,
        });
        app.update();

        let inventory_children = app.world.get::<Children>(inventory_entity);
        assert!(inventory_children.is_some());
        assert_eq!(2, inventory_children.unwrap().len());

        // assert the event for 3 has not been sent
        let stowed_loot_events = app.world.resource::<Events<StowedLoot>>();
        let mut stowed_loot_reader = stowed_loot_events.get_reader();
        // TODO: same here, why is the consumed previous event still in this new reader????
        assert_eq!(1, stowed_loot_reader.len(stowed_loot_events));
        stowed_loot_reader.read(stowed_loot_events).next(); // skip
        let actual_stowed_loot = stowed_loot_reader.read(stowed_loot_events).next();
        assert_eq!(None, actual_stowed_loot);
    }

    #[test]
    fn should_stow_loot_in_the_only_empty_slot() {
        // given
        let mut app = App::new();
        app.add_event::<StowLoot>();
        app.add_event::<StowedLoot>();
        app.add_systems(Update, stow_loot_system);
        let loot_entity = app.world.spawn(Loot).id();
        let mut inventory = app.world.spawn(Inventory);
        inventory.insert(ItemSlots(1));
        let inventory_entity = inventory.id();
        let inventory_children = app.world.get::<Children>(inventory_entity);
        assert!(inventory_children.is_none());

        // when
        // TODO: check that proper ItemSlot component was assigned to loot item within inventory
        app.world.resource_mut::<Events<StowLoot>>().send(StowLoot {
            stowing_entity: inventory_entity,
            loot: loot_entity,
        });
        app.update();

        // then
        let stowed_loot_events = app.world.resource::<Events<StowedLoot>>();
        let mut stowed_loot_reader = stowed_loot_events.get_reader();
        let actual_stowed_loot = stowed_loot_reader.read(stowed_loot_events).next().unwrap();
        let item_slot = app.world.get::<ItemSlot>(actual_stowed_loot.loot);
        assert_eq!(item_slot.unwrap().0, 0);
    }

    //#[test]
    fn should_stow_loot_in_the_second_slot_of_two() {
        // given
        let mut app = App::new();
        app.add_event::<StowLoot>();
        app.add_event::<StowedLoot>();
        app.add_systems(Update, stow_loot_system);
        let loot_in_inventory = app.world.spawn(Loot).id();
        let loot_from_ground = app.world.spawn(Loot).id();
        let mut inventory = app.world.spawn(Inventory);
        inventory.insert(ItemSlots(2));
        let inventory_entity = inventory.id();

        app.world.resource_mut::<Events<StowLoot>>().send(StowLoot {
            stowing_entity: inventory_entity,
            loot: loot_in_inventory,
        });
        app.update();
        app.update();

        let inventory_children = app.world.get::<Children>(inventory_entity);
        assert!(inventory_children.is_some());

        // when
        // TODO: check that proper ItemSlot component was assigned to loot item within inventory
        app.world.resource_mut::<Events<StowLoot>>().send(StowLoot {
            stowing_entity: inventory_entity,
            loot: loot_from_ground,
        });
        app.update();

        let inventory_children = app.world.get::<Children>(inventory_entity);
        assert!(inventory_children.is_some());

        // then
        let stowed_loot_events = app.world.resource::<Events<StowedLoot>>();
        let mut stowed_loot_reader = stowed_loot_events.get_reader();
        let actual_stowed_loot = stowed_loot_reader.read(stowed_loot_events).next().unwrap();
        let item_slot = app.world.get::<ItemSlot>(actual_stowed_loot.loot);
        assert_eq!(item_slot.unwrap().0, 1);
    }
}
