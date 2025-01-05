use bevy::app::Plugin;
use bevy::math::bounding::{Aabb3d, RayCast3d};
use bevy::render::primitives::{Aabb, Frustum};

use crate::first_person_controller::FirstPersonCamera;
use crate::inventory::Inventory;
use crate::raid::RaidState;
use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "interaction";

// Plugin
pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_interaction)
            .add_event::<InventoryInteracted>()
            .add_systems(
                Update,
                (update_interaction)
                    .run_if(in_state(AppState::Raid).and_then(in_state(RaidState::Raid))),
            )
            .add_systems(OnExit(AppState::Raid), bye_interaction);
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
}

// Systems
fn start_interaction(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn update_interaction(
    interact_probe: Query<(&Frustum, &GlobalTransform, &Parent), With<FirstPersonCamera>>,
    interactable_query: Query<(&Aabb, &GlobalTransform, Entity), With<Interactable>>,
    operator_inventory_query: Query<(Entity, &Parent), With<Inventory>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut interaction_event: EventWriter<InventoryInteracted>,
) {
    debug!("updating {}", NAME);
    if key_input.just_released(KeyCode::KeyF) {
        let probe = interact_probe.single();
        let operator = probe.2.get();
        let mut closest: Vec<(f32, Entity)> = interactable_query
            .iter()
            .filter(|interactable| {
                probe
                    .0
                    .intersects_obb(interactable.0, &interactable.1.affine(), true, true)
            })
            .filter_map(|interactable| {
                let looking_at_direction = probe.0.half_spaces[4].normal();
                let position = probe.1.translation();
                let r = RayCast3d::new(
                    position,
                    Dir3::new(looking_at_direction.into()).unwrap(),
                    2.0,
                );
                let aabb3d = Aabb3d::new(interactable.1.translation(), interactable.0.half_extents);
                let intersects = r.aabb_intersection_at(&aabb3d);
                intersects.map(|f| (f, interactable.2))
            })
            .collect();
        closest.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let first = closest.first();
        debug!("the closest one is: {:?}", first);

        if let Some((_, interactable)) = first {
            debug!("lets find an operator inventory");
            let operator_inventory: Option<Entity> = operator_inventory_query
                .iter()
                .find(|(_, parent)| parent.get().eq(&operator))
                .map(|(entity, _)| entity);
            debug!("how about: {:?}", operator_inventory);

            if let Some(operator_inventory) = operator_inventory {
                debug!("lets send out an event then");
                interaction_event.send(InventoryInteracted {
                    interaction_inventory: *interactable,
                    operator_inventory,
                });
            }
        }
    }
}

fn bye_interaction(mut _commands: Commands) {
    debug!("stopping {}", NAME);
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
