use bevy::app::Plugin;

use crate::exfil::Operator;
use crate::interaction::Interact;
use crate::inventory::Inventory;
use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "lock";

// Plugin
pub struct LockPlugin;

impl Plugin for LockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_lock_system)
            .add_systems(
                Update,
                (update_lock_system, unlock_system).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_lock_system);
    }
}

// Components

#[allow(dead_code)]
#[derive(Component)]
pub struct Lock {
    pub code: u32,
}

#[allow(dead_code)]
#[derive(Component, Debug)]
pub enum Key {
    RegularKey(RegularKey),
    SkeletonKey,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct RegularKey {
    pub code: u32,
}

// Resources

// Events
#[derive(Event, Debug, PartialEq)]
pub struct Unlocked {
    pub unlocked_entity: Entity,
    pub operator_entity: Entity,
}

#[derive(Event, Debug, PartialEq)]
pub struct StillLocked {
    pub still_locked_entity: Entity,
    pub operator_entity: Entity,
}

// Systems

fn start_lock_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

/// upon interaction check for existing locks and keys to initiate an unlock
#[allow(clippy::too_many_arguments)]
fn unlock_system(
    mut commands: Commands,
    mut interact: EventReader<Interact>,
    operators: Query<Entity, With<Operator>>,
    inventories: Query<(&Parent, Entity), With<Inventory>>,
    keys: Query<(&Parent, &Key)>,
    locks: Query<(Entity, &Lock)>,
    mut unlocked: EventWriter<Unlocked>,
    mut still_locked: EventWriter<StillLocked>,
) {
    for interact in interact.read() {
        debug!("unlocking");

        if let Ok(operator) = operators.get(interact.operator_entity) {
            if let Some((_, backpack)) = inventories
                .iter()
                .find(|(parent, _)| operator.eq(&parent.get()))
            {
                if let Ok((locked_object, lock)) = locks.get(interact.interaction_entity) {
                    let operator_keys: Vec<&Key> = keys
                        .iter()
                        .filter(|(parent, _)| backpack.eq(&parent.get()))
                        .filter(|(_, key)| match key {
                            Key::RegularKey(regular_key) => lock.code.eq(&regular_key.code),
                            Key::SkeletonKey => true,
                        })
                        .map(|(_, key)| key)
                        .collect();

                    if !operator_keys.is_empty() {
                        for key in operator_keys.iter().take(1) {
                            match key {
                                Key::RegularKey(regular_key) => {
                                    if lock.code.eq(&regular_key.code) {
                                        commands.entity(locked_object).remove::<Lock>();
                                        unlocked.send(Unlocked {
                                            unlocked_entity: locked_object,
                                            operator_entity: operator,
                                        });
                                    } else {
                                        still_locked.send(StillLocked {
                                            still_locked_entity: locked_object,
                                            operator_entity: operator,
                                        });
                                    }
                                }
                                Key::SkeletonKey => {
                                    commands.entity(locked_object).remove::<Lock>();
                                    unlocked.send(Unlocked {
                                        unlocked_entity: locked_object,
                                        operator_entity: operator,
                                    });
                                }
                            }
                        }
                    } else {
                        still_locked.send(StillLocked {
                            still_locked_entity: locked_object,
                            operator_entity: operator,
                        });
                    }
                }
            }
        }
    }
}

fn update_lock_system() {
    debug!("updating {}", NAME);
}

fn bye_lock_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    use crate::{exfil::Operator, inventory::Inventory};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_not_unlock_without_matching_key() {
        // given
        let code = 123;
        let wrong_code = 234;
        let mut app = App::new();
        app.add_event::<Interact>();
        app.add_event::<Unlocked>();
        app.add_event::<StillLocked>();
        app.add_systems(Update, unlock_system);
        let locked = app.world_mut().spawn(Lock { code }).id();
        let key_holder = app
            .world_mut()
            .spawn(Operator)
            .with_children(|operator| {
                operator.spawn(Inventory).with_children(|backpack| {
                    backpack.spawn(Key::RegularKey(RegularKey { code: wrong_code }));
                });
            })
            .id();

        // when
        app.world_mut()
            .resource_mut::<Events<Interact>>()
            .send(Interact {
                interaction_entity: locked,
                operator_entity: key_holder,
            });
        app.update();

        // then
        assert!(app.world().entity(locked).contains::<Lock>());

        // check if still_locked event was sent
        let still_locked_events = app.world().resource::<Events<StillLocked>>();
        let mut still_locked_reader = still_locked_events.get_reader();
        let actual_locked = still_locked_reader
            .read(still_locked_events)
            .next()
            .unwrap();
        let expected_locked = StillLocked {
            still_locked_entity: locked,
            operator_entity: key_holder,
        };
        assert_eq!(&expected_locked, actual_locked);
    }

    #[test]
    fn should_unlock_with_matching_regular_key() {
        // given
        let code = 123;
        let mut app = App::new();
        app.add_event::<Interact>();
        app.add_event::<Unlocked>();
        app.add_event::<StillLocked>();
        app.add_systems(Update, unlock_system);
        let locked = app.world_mut().spawn(Lock { code }).id();
        let key_holder = app
            .world_mut()
            .spawn(Operator)
            .with_children(|operator| {
                operator.spawn(Inventory).with_children(|backpack| {
                    backpack.spawn(Key::RegularKey(RegularKey { code }));
                });
            })
            .id();

        // when
        app.world_mut()
            .resource_mut::<Events<Interact>>()
            .send(Interact {
                interaction_entity: locked,
                operator_entity: key_holder,
            });
        app.update();

        // then
        assert!(!app.world().entity(locked).contains::<Lock>());

        // check if unlocked event was sent
        let unlocked_events = app.world().resource::<Events<Unlocked>>();
        let mut unlocked_reader = unlocked_events.get_reader();
        let actual_unlocked = unlocked_reader.read(unlocked_events).next().unwrap();
        let expected_unlocked = Unlocked {
            unlocked_entity: locked,
            operator_entity: key_holder,
        };
        assert_eq!(&expected_unlocked, actual_unlocked);
    }
}
