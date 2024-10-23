use std::collections::HashSet;
use std::ops::Range;

use bevy::app::Plugin;
use bevy_inspector_egui::prelude::*;

use crate::loot::{DroppedLoot, Loot, LootType};
use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "inventory";

// Plugin
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ItemSlot>()
            .register_type::<ItemSlots>()
            .register_type::<WeaponSlot>()
            .register_type::<WeaponSlots>()
            .add_event::<StowLoot>()
            .add_event::<StowedLoot>()
            .add_event::<DropLoot>()
            .add_systems(OnEnter(Raid), start_inventory_system)
            .add_systems(
                Update,
                (stow_loot_system, drop_loot_system).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_inventory_system);
    }
}

// Components

#[derive(Component)]
pub struct Inventory;

/// number of item slots
#[derive(Component, Clone, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct ItemSlots(pub u8);

/// position within the item slots
#[derive(Component, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct ItemSlot(pub u8);

/// number of weapon slots
#[derive(Component, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct WeaponSlots(pub u8);

/// position within the weapon slots
#[derive(Component, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct WeaponSlot(pub u8);

// Resources

// Events

/// command for stowing loot
#[derive(Event, Debug, PartialEq)]
pub struct StowLoot {
    pub stowing_entity: Entity,
    pub loot: Entity,
    pub loot_type: LootType,
}

/// event for stowed loot
#[derive(Event, Debug, PartialEq)]
pub struct StowedLoot {
    pub stowing_entity: Entity,
    pub loot: Entity,
}

#[derive(Event, Debug, PartialEq)]
pub struct DropLoot {
    pub dropping_entity: Entity,
    pub loot: Entity,
}

// Systems
fn start_inventory_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn stow_loot_system(
    mut commands: Commands,
    mut command: EventReader<StowLoot>,
    inventories_with_items: Query<&ItemSlots, With<Inventory>>,
    inventory_items: Query<(&Parent, &ItemSlot), With<Loot>>,
    inventories_with_weapons: Query<&WeaponSlots, With<Inventory>>,
    inventory_weapons: Query<(&Parent, &WeaponSlot), With<Loot>>,
    mut event: EventWriter<StowedLoot>,
) {
    debug!("updating stow listener");

    for c in command.read() {
        let inventory = c.stowing_entity;

        let inventory_items: Vec<&ItemSlot> = inventory_items
            .iter()
            .filter(|ii| inventory == ii.0.get())
            .map(|ii| ii.1)
            .collect();
        let item_slots: usize = inventories_with_items
            .get(inventory)
            .map_or(0, |r| r.0.into());

        let weapons: Vec<&WeaponSlot> = inventory_weapons
            .iter()
            .filter(|ii| inventory == ii.0.get())
            .map(|ii| ii.1)
            .collect();
        let weapon_slots: usize = inventories_with_weapons
            .get(inventory)
            .map_or(0, |r| r.0.into());

        match c.loot_type {
            LootType::Item(_)
            | LootType::Ammo
            | LootType::Lethal
            | LootType::Tactical
            | LootType::CombatDefense
            | LootType::FieldUpgrade
            | LootType::KillStreak
            | LootType::CircleDefense
            | LootType::RadiationProtection
            | LootType::LastStand
            | LootType::Intel
            | LootType::Cash
            | LootType::Key => {
                if let Some(slot) = calc_stow_item_slot(&inventory_items, item_slots) {
                    stow_item(&mut commands, c.loot, c.stowing_entity, slot, &mut event);
                }
            }
            LootType::Weapon => {
                if let Some(slot) = calc_stow_weapon_slot(&weapons, weapon_slots) {
                    stow_weapon(&mut commands, c.loot, c.stowing_entity, slot, &mut event);
                }
            }
        }
    }
}

fn calc_stow_item_slot(items: &[&ItemSlot], item_slots: usize) -> Option<u8> {
    if items.len() < item_slots {
        let item_count = items.len();
        let mut target_slot: u8 = 0;
        if item_count < item_slots && item_count != 0 {
            let range: Range<usize> = Range {
                start: 0,
                end: item_count + 1,
            };
            let mut set: HashSet<usize> = range.into_iter().collect();
            items.iter().for_each(|i| {
                set.remove(&i.0.into());
            });
            let x: Vec<&usize> = set.iter().collect();
            if let Some(target) = x.first() {
                target_slot = **target as u8;
            }
        }
        Some(target_slot)
    } else {
        None
    }
}

fn calc_stow_weapon_slot(weapons: &[&WeaponSlot], weapon_slots: usize) -> Option<u8> {
    if weapons.len() < weapon_slots {
        let weapon_count = weapons.len();
        let mut target_slot: u8 = 0;
        if weapon_count < weapon_slots && weapon_count != 0 {
            let range: Range<usize> = Range {
                start: 0,
                end: weapon_count + 1,
            };
            let mut set: HashSet<usize> = range.into_iter().collect();
            weapons.iter().for_each(|i| {
                set.remove(&i.0.into());
            });
            let x: Vec<&usize> = set.iter().collect();
            if let Some(target) = x.first() {
                target_slot = **target as u8;
            }
        }
        Some(target_slot)
    } else {
        None
    }
}

fn stow_item(
    commands: &mut Commands,
    loot: Entity,
    stowing_entity: Entity,
    slot: u8,
    event: &mut EventWriter<StowedLoot>,
) {
    debug!("button stowed loot received");
    commands.entity(loot).remove::<GlobalTransform>();
    commands.entity(loot).remove::<Transform>();
    commands.entity(stowing_entity).add_child(loot);
    commands.entity(loot).insert(ItemSlot(slot));
    debug!("button stowed loot finished");
    event.send(StowedLoot {
        stowing_entity,
        loot,
    });
}

fn stow_weapon(
    commands: &mut Commands,
    loot: Entity,
    stowing_entity: Entity,
    slot: u8,
    event: &mut EventWriter<StowedLoot>,
) {
    commands.entity(loot).remove::<GlobalTransform>();
    commands.entity(loot).remove::<Transform>();
    commands.entity(stowing_entity).add_child(loot);
    commands.entity(loot).insert(WeaponSlot(slot));
    event.send(StowedLoot {
        stowing_entity,
        loot,
    });
}

fn drop_loot_system(
    mut commands: Commands,
    mut command: EventReader<DropLoot>,
    inventories_with_items: Query<&GlobalTransform, (With<Inventory>, With<ItemSlots>)>,
    inventory_items: Query<(&Parent, &ItemSlot), With<Loot>>,
    inventories_with_weapons: Query<&GlobalTransform, (With<Inventory>, With<WeaponSlots>)>,
    inventory_weapons: Query<(&Parent, &WeaponSlot), With<Loot>>,
    mut event: EventWriter<DroppedLoot>,
) {
    debug!("dropping loot with {}", NAME);
    for c in command.read() {
        // drop item ...
        if let Ok((inventory, _item_slot)) = inventory_items.get(c.loot) {
            // check if the correct inventory was addressed in command
            if inventory.get() == c.dropping_entity {
                if let Ok(global_transform) = inventories_with_items.get(inventory.get()) {
                    commands.entity(c.loot).remove_parent();
                    commands.entity(c.loot).remove::<ItemSlot>();
                    commands
                        .entity(c.loot)
                        .insert(global_transform.compute_transform());
                    commands.entity(c.loot).insert(*global_transform);
                    event.send(DroppedLoot {
                        dropping_entity: inventory.get(),
                        dropped_position: global_transform.translation(),
                        loot: c.loot,
                    });
                };
            }
        }

        // ... or drop weapon
        if let Ok((inventory, _weapon_slot)) = inventory_weapons.get(c.loot) {
            if inventory.get() == c.dropping_entity {
                if let Ok(global_transform) = inventories_with_weapons.get(inventory.get()) {
                    commands.entity(c.loot).remove_parent();
                    commands.entity(c.loot).remove::<WeaponSlot>();
                    commands
                        .entity(c.loot)
                        .insert(global_transform.compute_transform());
                    commands.entity(c.loot).insert(*global_transform);
                    event.send(DroppedLoot {
                        dropping_entity: inventory.get(),
                        dropped_position: global_transform.translation(),
                        loot: c.loot,
                    });
                };
            }
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
    use crate::loot::{ItemType, Loot};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_stow_item_loot() {
        // given
        let mut app = App::new();
        app.add_event::<StowLoot>();
        app.add_event::<StowedLoot>();
        app.add_systems(Update, stow_loot_system);
        let loot_entity = app.world_mut().spawn(Loot).id();
        let mut inventory = app.world_mut().spawn(Inventory);
        inventory.insert(ItemSlots(1));
        let inventory_entity = inventory.id();
        let inventory_children = app.world().get::<Children>(inventory_entity);
        assert!(inventory_children.is_none());

        // when
        app.world_mut()
            .resource_mut::<Events<StowLoot>>()
            .send(StowLoot {
                stowing_entity: inventory_entity,
                loot: loot_entity,
                loot_type: LootType::Item(ItemType::Item),
            });
        app.update();

        // then
        // assert inventory now has children
        let inventory_children = app.world().get::<Children>(inventory_entity);
        assert!(inventory_children.is_some());

        // assert the one child it the loot entity
        for &child in inventory_children.unwrap() {
            assert_eq!(child, loot_entity);
        }

        // assert the loot entity has not been duplicated and was really moved
        let entities = app
            .world_mut()
            .query::<(Entity, &Loot)>()
            .iter(&app.world())
            .collect::<Vec<_>>();
        assert_eq!(1, entities.len());

        // assert the event has been sent
        let stowed_loot_events = app.world().resource::<Events<StowedLoot>>();
        let mut stowed_loot_reader = stowed_loot_events.get_reader();
        let actual_stowed_loot = stowed_loot_reader.read(stowed_loot_events).next().unwrap();

        let expected_stowed_loot = StowedLoot {
            stowing_entity: inventory_entity,
            loot: loot_entity,
        };
        assert_eq!(&expected_stowed_loot, actual_stowed_loot);
    }

    #[test]
    fn should_stow_weapon_loot() {
        // given
        let mut app = App::new();
        app.add_event::<StowLoot>();
        app.add_event::<StowedLoot>();
        app.add_systems(Update, stow_loot_system);
        let loot_entity = app.world_mut().spawn(Loot).id();
        let mut inventory = app.world_mut().spawn(Inventory);
        inventory.insert(WeaponSlots(1));
        let inventory_entity = inventory.id();
        let inventory_children = app.world().get::<Children>(inventory_entity);
        assert!(inventory_children.is_none());

        // when
        app.world_mut()
            .resource_mut::<Events<StowLoot>>()
            .send(StowLoot {
                stowing_entity: inventory_entity,
                loot: loot_entity,
                loot_type: LootType::Weapon,
            });
        app.update();

        // then
        // assert inventory now has children
        let inventory_children = app.world().get::<Children>(inventory_entity);
        assert!(inventory_children.is_some());

        // assert the one child it the loot entity
        for &child in inventory_children.unwrap() {
            assert_eq!(child, loot_entity);
        }

        // assert the loot entity has not been duplicated and was really moved
        let entities = app
            .world_mut()
            .query::<(Entity, &Loot)>()
            .iter(&app.world())
            .collect::<Vec<_>>();
        assert_eq!(1, entities.len());

        // assert the event has been sent
        let stowed_loot_events = app.world().resource::<Events<StowedLoot>>();
        let mut stowed_loot_reader = stowed_loot_events.get_reader();
        let actual_stowed_loot = stowed_loot_reader.read(stowed_loot_events).next().unwrap();

        let expected_stowed_loot = StowedLoot {
            stowing_entity: inventory_entity,
            loot: loot_entity,
        };
        assert_eq!(&expected_stowed_loot, actual_stowed_loot);
    }

    #[test]
    fn should_stow_item_loot_and_respect_capacity() {
        // given
        let mut app = App::new();
        app.add_event::<StowLoot>();
        app.add_event::<StowedLoot>();
        app.add_systems(Update, stow_loot_system);
        let loot_entity_1 = app.world_mut().spawn(Loot).id();
        let loot_entity_2 = app.world_mut().spawn(Loot).id();
        let loot_entity_3 = app.world_mut().spawn(Loot).id();
        let mut inventory = app.world_mut().spawn(Inventory);
        inventory.insert(ItemSlots(2));
        let inventory_entity = inventory.id();
        let inventory_children = app.world().get::<Children>(inventory_entity);
        assert!(inventory_children.is_none());

        // when / then
        // first loot item should succeed
        app.world_mut()
            .resource_mut::<Events<StowLoot>>()
            .send(StowLoot {
                stowing_entity: inventory_entity,
                loot: loot_entity_1,
                loot_type: LootType::Item(ItemType::Item),
            });
        app.update();

        // assert inventory now has 1 children
        let inventory_children = app.world().get::<Children>(inventory_entity);
        assert!(inventory_children.is_some());
        assert_eq!(1, inventory_children.unwrap().len());

        inventory_children.unwrap().iter().for_each(|c| {
            let item_slot = app.world().get::<ItemSlot>(*c);
            assert!(item_slot.is_some());
        });

        // assert the event for 1 has been sent
        let stowed_loot_events = app.world().resource::<Events<StowedLoot>>();
        let mut stowed_loot_reader = stowed_loot_events.get_reader();
        assert_eq!(1, stowed_loot_reader.len(stowed_loot_events));
        let actual_stowed_loot = stowed_loot_reader.read(stowed_loot_events).next().unwrap();

        let expected_stowed_loot = StowedLoot {
            stowing_entity: inventory_entity,
            loot: loot_entity_1,
        };
        assert_eq!(&expected_stowed_loot, actual_stowed_loot);

        // second loot item should succeed too
        app.world_mut()
            .resource_mut::<Events<StowLoot>>()
            .send(StowLoot {
                stowing_entity: inventory_entity,
                loot: loot_entity_2,
                loot_type: LootType::Item(ItemType::Item),
            });
        app.update();

        let inventory_children = app.world().get::<Children>(inventory_entity);
        assert!(inventory_children.is_some());
        assert_eq!(2, inventory_children.unwrap().len());

        inventory_children.unwrap().iter().for_each(|c| {
            let item_slot = app.world().get::<ItemSlot>(*c);
            assert!(item_slot.is_some());
        });

        // assert the event for 2 has been sent
        let stowed_loot_events = app.world().resource::<Events<StowedLoot>>();
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
        app.world_mut()
            .resource_mut::<Events<StowLoot>>()
            .send(StowLoot {
                stowing_entity: inventory_entity,
                loot: loot_entity_3,
                loot_type: LootType::Item(ItemType::Item),
            });
        app.update();

        let inventory_children = app.world().get::<Children>(inventory_entity);
        assert!(inventory_children.is_some());
        assert_eq!(2, inventory_children.unwrap().len());

        // assert the event for 3 has not been sent
        let stowed_loot_events = app.world().resource::<Events<StowedLoot>>();
        let mut stowed_loot_reader = stowed_loot_events.get_reader();
        // TODO: same here, why is the consumed previous event still in this new reader????
        assert_eq!(1, stowed_loot_reader.len(stowed_loot_events));
        stowed_loot_reader.read(stowed_loot_events).next(); // skip
        let actual_stowed_loot = stowed_loot_reader.read(stowed_loot_events).next();
        assert_eq!(None, actual_stowed_loot);
    }

    #[test]
    fn should_stow_weapon_loot_and_respect_capacity() {
        // given
        let mut app = App::new();
        app.add_event::<StowLoot>();
        app.add_event::<StowedLoot>();
        app.add_systems(Update, stow_loot_system);
        let loot_entity_1 = app.world_mut().spawn(Loot).id();
        let loot_entity_2 = app.world_mut().spawn(Loot).id();
        let loot_entity_3 = app.world_mut().spawn(Loot).id();
        let mut inventory = app.world_mut().spawn(Inventory);
        inventory.insert(WeaponSlots(2));
        let inventory_entity = inventory.id();
        let inventory_children = app.world().get::<Children>(inventory_entity);
        assert!(inventory_children.is_none());

        // when / then
        // first loot item should succeed
        app.world_mut()
            .resource_mut::<Events<StowLoot>>()
            .send(StowLoot {
                stowing_entity: inventory_entity,
                loot: loot_entity_1,
                loot_type: LootType::Weapon,
            });
        app.update();

        // assert inventory now has 1 children
        let inventory_children = app.world().get::<Children>(inventory_entity);
        assert!(inventory_children.is_some());
        assert_eq!(1, inventory_children.unwrap().len());

        inventory_children.unwrap().iter().for_each(|c| {
            let weapon_slot = app.world().get::<WeaponSlot>(*c);
            assert!(weapon_slot.is_some());
        });

        // assert the event for 1 has been sent
        let stowed_loot_events = app.world().resource::<Events<StowedLoot>>();
        let mut stowed_loot_reader = stowed_loot_events.get_reader();
        assert_eq!(1, stowed_loot_reader.len(stowed_loot_events));
        let actual_stowed_loot = stowed_loot_reader.read(stowed_loot_events).next().unwrap();

        let expected_stowed_loot = StowedLoot {
            stowing_entity: inventory_entity,
            loot: loot_entity_1,
        };
        assert_eq!(&expected_stowed_loot, actual_stowed_loot);

        // second loot item should succeed too
        app.world_mut()
            .resource_mut::<Events<StowLoot>>()
            .send(StowLoot {
                stowing_entity: inventory_entity,
                loot: loot_entity_2,
                loot_type: LootType::Weapon,
            });
        app.update();

        let inventory_children = app.world().get::<Children>(inventory_entity);
        assert!(inventory_children.is_some());
        assert_eq!(2, inventory_children.unwrap().len());

        inventory_children.unwrap().iter().for_each(|c| {
            let weapon_slot = app.world().get::<WeaponSlot>(*c);
            assert!(weapon_slot.is_some());
        });

        // assert the event for 2 has been sent
        let stowed_loot_events = app.world().resource::<Events<StowedLoot>>();
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
        app.world_mut()
            .resource_mut::<Events<StowLoot>>()
            .send(StowLoot {
                stowing_entity: inventory_entity,
                loot: loot_entity_3,
                loot_type: LootType::Weapon,
            });
        app.update();

        let inventory_children = app.world().get::<Children>(inventory_entity);
        assert!(inventory_children.is_some());
        assert_eq!(2, inventory_children.unwrap().len());

        // assert the event for 3 has not been sent
        let stowed_loot_events = app.world().resource::<Events<StowedLoot>>();
        let mut stowed_loot_reader = stowed_loot_events.get_reader();
        // TODO: same here, why is the consumed previous event still in this new reader????
        assert_eq!(1, stowed_loot_reader.len(stowed_loot_events));
        stowed_loot_reader.read(stowed_loot_events).next(); // skip
        let actual_stowed_loot = stowed_loot_reader.read(stowed_loot_events).next();
        assert_eq!(None, actual_stowed_loot);
    }

    #[test]
    fn should_stow_item_loot_in_the_only_empty_slot() {
        // given
        let mut app = App::new();
        app.add_event::<StowLoot>();
        app.add_event::<StowedLoot>();
        app.add_systems(Update, stow_loot_system);
        let loot_entity = app.world_mut().spawn(Loot).id();
        let mut inventory = app.world_mut().spawn(Inventory);
        inventory.insert(ItemSlots(1));
        let inventory_entity = inventory.id();
        let inventory_children = app.world().get::<Children>(inventory_entity);
        assert!(inventory_children.is_none());

        // when
        // TODO: check that proper ItemSlot component was assigned to loot item within inventory
        app.world_mut()
            .resource_mut::<Events<StowLoot>>()
            .send(StowLoot {
                stowing_entity: inventory_entity,
                loot: loot_entity,
                loot_type: LootType::Item(ItemType::Item),
            });
        app.update();

        // then
        let stowed_loot_events = app.world().resource::<Events<StowedLoot>>();
        let mut stowed_loot_reader = stowed_loot_events.get_reader();
        let actual_stowed_loot = stowed_loot_reader.read(stowed_loot_events).next().unwrap();
        let item_slot = app.world().get::<ItemSlot>(actual_stowed_loot.loot);
        assert_eq!(item_slot.unwrap().0, 0);
    }

    #[test]
    fn should_stow_weapon_loot_in_the_only_empty_slot() {
        // given
        let mut app = App::new();
        app.add_event::<StowLoot>();
        app.add_event::<StowedLoot>();
        app.add_systems(Update, stow_loot_system);
        let loot_entity = app.world_mut().spawn(Loot).id();
        let mut inventory = app.world_mut().spawn(Inventory);
        inventory.insert(WeaponSlots(1));
        let inventory_entity = inventory.id();
        let inventory_children = app.world().get::<Children>(inventory_entity);
        assert!(inventory_children.is_none());

        // when
        // TODO: check that proper ItemSlot component was assigned to loot item within inventory
        app.world_mut()
            .resource_mut::<Events<StowLoot>>()
            .send(StowLoot {
                stowing_entity: inventory_entity,
                loot: loot_entity,
                loot_type: LootType::Weapon,
            });
        app.update();

        // then
        let stowed_loot_events = app.world().resource::<Events<StowedLoot>>();
        let mut stowed_loot_reader = stowed_loot_events.get_reader();
        let actual_stowed_loot = stowed_loot_reader.read(stowed_loot_events).next().unwrap();
        let item_slot = app.world().get::<WeaponSlot>(actual_stowed_loot.loot);
        assert_eq!(item_slot.unwrap().0, 0);
    }

    #[test]
    fn should_stow_item_loot_in_the_second_slot_of_two() {
        // given
        let mut app = App::new();
        app.add_event::<StowLoot>();
        app.add_event::<StowedLoot>();
        app.add_systems(Update, stow_loot_system);
        let loot_in_inventory = app.world_mut().spawn(Loot).id();
        let loot_from_ground = app.world_mut().spawn(Loot).id();
        let mut inventory = app.world_mut().spawn(Inventory);
        inventory.insert(ItemSlots(2));
        let inventory_entity = inventory.id();

        app.world_mut()
            .resource_mut::<Events<StowLoot>>()
            .send(StowLoot {
                stowing_entity: inventory_entity,
                loot: loot_in_inventory,
                loot_type: LootType::Item(ItemType::Item),
            });
        app.update();
        app.update();

        let inventory_children = app.world().get::<Children>(inventory_entity);
        assert!(inventory_children.is_some());

        // when
        app.world_mut()
            .resource_mut::<Events<StowLoot>>()
            .send(StowLoot {
                stowing_entity: inventory_entity,
                loot: loot_from_ground,
                loot_type: LootType::Item(ItemType::Item),
            });
        app.update();

        let inventory_children = app.world().get::<Children>(inventory_entity);
        assert!(inventory_children.is_some());

        // then
        let stowed_loot_events = app.world().resource::<Events<StowedLoot>>();
        let mut stowed_loot_reader = stowed_loot_events.get_reader();
        let actual_stowed_loot = stowed_loot_reader.read(stowed_loot_events).next().unwrap();
        let item_slot = app.world().get::<ItemSlot>(actual_stowed_loot.loot);
        assert_eq!(item_slot.unwrap().0, 1);
    }

    #[test]
    fn should_stow_weapon_loot_in_the_second_slot_of_two() {
        // given
        let mut app = App::new();
        app.add_event::<StowLoot>();
        app.add_event::<StowedLoot>();
        app.add_systems(Update, stow_loot_system);
        let loot_in_inventory = app.world_mut().spawn(Loot).id();
        let loot_from_ground = app.world_mut().spawn(Loot).id();
        let mut inventory = app.world_mut().spawn(Inventory);
        inventory.insert(WeaponSlots(2));
        let inventory_entity = inventory.id();

        app.world_mut()
            .resource_mut::<Events<StowLoot>>()
            .send(StowLoot {
                stowing_entity: inventory_entity,
                loot: loot_in_inventory,
                loot_type: LootType::Weapon,
            });
        app.update();
        app.update();

        let inventory_children = app.world().get::<Children>(inventory_entity);
        assert!(inventory_children.is_some());

        // when
        app.world_mut()
            .resource_mut::<Events<StowLoot>>()
            .send(StowLoot {
                stowing_entity: inventory_entity,
                loot: loot_from_ground,
                loot_type: LootType::Weapon,
            });
        app.update();

        let inventory_children = app.world().get::<Children>(inventory_entity);
        assert!(inventory_children.is_some());

        // then
        let stowed_loot_events = app.world().resource::<Events<StowedLoot>>();
        let mut stowed_loot_reader = stowed_loot_events.get_reader();
        let actual_stowed_loot = stowed_loot_reader.read(stowed_loot_events).next().unwrap();
        let item_slot = app.world().get::<WeaponSlot>(actual_stowed_loot.loot);
        assert_eq!(item_slot.unwrap().0, 1);
    }

    #[test]
    fn should_drop_item_loot() {
        // given
        let mut app = App::new();
        app.add_event::<DropLoot>();
        app.add_event::<DroppedLoot>();
        app.add_systems(Update, drop_loot_system);
        let loot_in_inventory = app.world_mut().spawn(Loot).insert(ItemSlot(0)).id();
        let mut inventory = app.world_mut().spawn(Inventory);
        inventory.add_child(loot_in_inventory);
        inventory.insert(ItemSlots(1));
        let inventory_position = Vec3::new(1.0, 2.0, 3.0);
        let inventory_transform = Transform::from_translation(inventory_position);
        inventory.insert(inventory_transform);
        inventory.insert(GlobalTransform::from(inventory_transform));
        let inventory_entity = inventory.id();

        // when
        app.world_mut()
            .resource_mut::<Events<DropLoot>>()
            .send(DropLoot {
                dropping_entity: inventory_entity,
                loot: loot_in_inventory,
            });
        app.update();

        // then
        // check if item is no longer in inventory
        let inventory_children = app.world().get::<Children>(inventory_entity);
        assert!(inventory_children.is_none());
        // check if item is on ground at position of dropping entity
        let dropped_item_transform = app.world().get::<Transform>(loot_in_inventory);
        assert!(dropped_item_transform.is_some());
        assert_eq!(
            inventory_position,
            dropped_item_transform.unwrap().translation
        );
        let dropped_item_global_transform = app.world().get::<GlobalTransform>(loot_in_inventory);
        assert!(dropped_item_global_transform.is_some());
        assert_eq!(
            inventory_position,
            dropped_item_global_transform.unwrap().translation()
        );
        // check if dropped item event was sent
        let dropped_loot_events = app.world().resource::<Events<DroppedLoot>>();
        let mut dropped_loot_reader = dropped_loot_events.get_reader();
        let actual_dropped_loot = dropped_loot_reader
            .read(dropped_loot_events)
            .next()
            .unwrap();
        let expected_dropped_loot = DroppedLoot {
            dropping_entity: inventory_entity,
            dropped_position: inventory_position,
            loot: loot_in_inventory,
        };
        assert_eq!(&expected_dropped_loot, actual_dropped_loot);
    }

    #[test]
    fn should_drop_weapon_loot() {
        // given
        let mut app = App::new();
        app.add_event::<DropLoot>();
        app.add_event::<DroppedLoot>();
        app.add_systems(Update, drop_loot_system);
        let loot_in_inventory = app.world_mut().spawn(Loot).insert(WeaponSlot(0)).id();
        let mut inventory = app.world_mut().spawn(Inventory);
        inventory.add_child(loot_in_inventory);
        inventory.insert(WeaponSlots(1));
        let inventory_position = Vec3::new(1.0, 2.0, 3.0);
        let inventory_transform = Transform::from_translation(inventory_position);
        inventory.insert(inventory_transform);
        inventory.insert(GlobalTransform::from(inventory_transform));
        let inventory_entity = inventory.id();

        // when
        app.world_mut()
            .resource_mut::<Events<DropLoot>>()
            .send(DropLoot {
                dropping_entity: inventory_entity,
                loot: loot_in_inventory,
            });
        app.update();

        // then
        // check if item is no longer in inventory
        let inventory_children = app.world().get::<Children>(inventory_entity);
        assert!(inventory_children.is_none());
        // check if item is on ground at position of dropping entity
        let dropped_weapon_transform = app.world().get::<Transform>(loot_in_inventory);
        assert!(dropped_weapon_transform.is_some());
        assert_eq!(
            inventory_position,
            dropped_weapon_transform.unwrap().translation
        );
        let dropped_weapon_global_transform = app.world().get::<GlobalTransform>(loot_in_inventory);
        assert!(dropped_weapon_global_transform.is_some());
        assert_eq!(
            inventory_position,
            dropped_weapon_global_transform.unwrap().translation()
        );
        // check if dropped item event was sent
        let dropped_loot_events = app.world().resource::<Events<DroppedLoot>>();
        let mut dropped_loot_reader = dropped_loot_events.get_reader();
        let actual_dropped_loot = dropped_loot_reader
            .read(dropped_loot_events)
            .next()
            .unwrap();
        let expected_dropped_loot = DroppedLoot {
            dropping_entity: inventory_entity,
            dropped_position: inventory_position,
            loot: loot_in_inventory,
        };
        assert_eq!(&expected_dropped_loot, actual_dropped_loot);
    }
}
