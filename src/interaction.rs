use bevy::app::Plugin;
use bevy::math::bounding::{Aabb3d, RayCast3d};
use bevy::render::primitives::{Aabb, Frustum};

use crate::first_person_controller::FirstPersonCamera;
use crate::inventory::{Inventory, InventoryAccessed};
use crate::loot::LootCacheState;
use crate::raid::RaidState;
use crate::AppState;
use bevy::prelude::*;

// Constants
const NAME: &str = "interaction";

// Plugin
pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InventoryInteracted>()
            .add_event::<Interact>()
            .add_systems(
                Update,
                (interaction, inventory_interaction)
                    .run_if(in_state(AppState::Raid).and_then(in_state(RaidState::Raid))),
            );
    }
}

// Components

#[derive(Component)]
pub struct Interactable;

// Resources

// Events
#[derive(Event, Debug, PartialEq)]
pub struct InventoryInteracted {
    pub interaction_inventory: Entity,
    pub operator_inventory: Entity,
    pub operator: Entity,
}

/// generic comand message for further interaction processing
#[derive(Event, Debug, PartialEq)]
pub struct Interact {
    pub interaction_entity: Entity,
    pub operator_entity: Entity,
}

// Systems

/// system that checks for entities to interact with, render gizmo and sending out a generic command message that can be used to further process the interaction without having to do all the raycasting and stuff again.
/// emits a ```Interact``` command/event that can be used by other listeners to act on.
fn interaction(
    interact_probe: Query<(&Frustum, &GlobalTransform, Entity, &Parent), With<FirstPersonCamera>>,
    interactable_query: Query<(Entity, &Aabb, &GlobalTransform, &Name), With<Interactable>>,
    mut gizmos: Gizmos,
    key_input: Res<ButtonInput<KeyCode>>,
    mut interact_command: EventWriter<Interact>,
) {
    debug!("interaction {}", NAME);
    let probe = interact_probe.single();
    let mut closest: Vec<(f32, Entity, &Name)> = interactable_query
        .iter()
        // check if entity is in camera view frustum or not
        .filter(|inventory| {
            probe
                .0
                .intersects_obb(inventory.1, &inventory.2.affine(), true, true)
        })
        .filter_map(|inventory| {
            debug!("inventory probe_result: {}", inventory.3);
            let looking_at_direction = probe.0.half_spaces[4].normal();
            let position = probe.1.translation();
            let r = RayCast3d::new(
                position,
                Dir3::new(looking_at_direction.into()).unwrap(),
                2.0,
            );
            let aabb3d = Aabb3d::new(inventory.2.translation(), inventory.1.half_extents);
            let intersects = r.aabb_intersection_at(&aabb3d);
            if let Some(distance) = intersects {
                debug!(
                    "im allowed to interact with {}. distance: {}",
                    inventory.3, distance
                );
                let b: Vec3 = inventory.1.half_extents.into();
                gizmos.cuboid(
                    Transform::from_translation(inventory.2.translation()).with_scale(b * 2.05),
                    Srgba::rgb(1.0, 0.84, 0.0),
                );
            }
            intersects.map(|f| (f, inventory.0, inventory.3))
        })
        .collect();
    closest.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    debug!("closest: {:?}", closest);

    let first = closest.first();
    debug!("the closest one is: {:?}", first);
    if let Some((_, entity, name)) = first {
        if key_input.just_released(KeyCode::KeyF) {
            debug!("interacting with entity {:?}", name);
            interact_command.send(Interact {
                interaction_entity: *entity,
                operator_entity: probe.3.get(),
            });
        }
    }
}

fn inventory_interaction(
    mut commands: Commands,
    mut interaction_commands: EventReader<Interact>,
    interaction_inventory_query: Query<(Entity, &LootCacheState), With<Inventory>>,
    backpack_query: Query<(Entity, &Parent), With<Inventory>>,
    mut inventory_interacted: EventWriter<InventoryInteracted>,
    mut inventory_opened: EventWriter<InventoryAccessed>,
) {
    for command in interaction_commands.read() {
        // filter for commands on Inventory entities only
        if let Ok((interaction_inventory, loot_cache_state)) =
            interaction_inventory_query.get(command.interaction_entity)
        {
            backpack_query
                .iter()
                .map(|(backpack, parent)| (backpack, parent.get()))
                .filter(|(_, parent)| parent.eq(&command.operator_entity))
                .for_each(|(backpack, operator)| {
                    // this event we send regardless of locked inventories
                    inventory_interacted.send(InventoryInteracted {
                        interaction_inventory,
                        operator_inventory: backpack,
                        operator,
                    });
                    // TODO: put in token so only one operator can access loot caches at once
                    match loot_cache_state {
                        LootCacheState::Closed => {
                            commands
                                .entity(interaction_inventory)
                                .insert(LootCacheState::Open);
                            inventory_opened.send(InventoryAccessed {
                                operator,
                                backpack,
                                inventory: interaction_inventory,
                            });
                        }
                        LootCacheState::Open => {
                            inventory_opened.send(InventoryAccessed {
                                operator,
                                backpack,
                                inventory: interaction_inventory,
                            });
                        }
                        _ => (),
                    }
                });
        }
    }
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    /*
    #[test]
    fn should_test_something() {
        // given
        //let mut _app = App::new();

        // when
        //app.add_event::<HealthDamageReceived>();
        //app.add_systems(Update, damage_received_listener);
        //let entity = app.borrow_mut().world.spawn(Health(100)).id();
        //app.borrow_mut().world.resource_mut::<Events<HealthDamageReceived>>().send(HealthDamageReceived { entity, damage: 10 });
        //app.update();

        // then
        //assert!(app.world.get::<Health>(entity).is_some());
        //assert_eq!(app.world.get::<Health>(entity).unwrap().0, 90);
    }
    */
}
