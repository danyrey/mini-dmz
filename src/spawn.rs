use bevy::app::Plugin;
use bevy::utils::HashMap;

use crate::exfil::Operator;
use crate::squad::SquadId;
use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

// Constants
const NAME: &str = "spawn";

#[derive(Reflect)]
pub enum Formation {
    Triangle,
    Staggered,
    Line,
}

// Plugin
pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Spawn>()
            .register_type::<SpawnId>()
            .add_systems(OnEnter(Raid), start_spawn)
            .add_systems(
                Update,
                (update_spawn, added_squad_id_to_operator).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_spawn);
    }
}

// Components

#[allow(dead_code)]
#[derive(Component, Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub struct SpawnId(pub u32);

#[derive(Component, Reflect)]
pub struct Spawn {
    pub formation: Formation,
}

#[derive(Component, Reflect)]
pub struct SpawnPosition;

#[derive(Component, Reflect)]
pub struct SpawnPositionOccupied;

#[derive(Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Spawns {
    pub map: HashMap<SpawnId, Spawn>,
}

// Resources

// Events

// Systems
fn start_spawn(mut _commands: Commands, _spawn_added: Query<Entity, Added<Spawn>>) {
    debug!("starting {}", NAME);
}

/// this system moves operator to their respective spawn position positions and orientations
#[allow(clippy::type_complexity)]
fn added_squad_id_to_operator(
    mut commands: Commands,
    added_operators: Query<(Entity, &SquadId), (With<Operator>, Added<SquadId>)>,
    spawn_query: Query<
        (Entity, &SquadId, &Transform, Option<&SpawnPositionOccupied>),
        With<SpawnPosition>,
    >,
) {
    let mut all_spawns: HashMap<&SquadId, Vec<(Entity, &Transform)>> = HashMap::default();

    // filter out occupied spawns
    let unoccupied_spawns: Vec<(Entity, &SquadId, &Transform, Option<&SpawnPositionOccupied>)> =
        spawn_query
            .iter()
            .filter(|(_, _, _, occupied)| occupied.is_none())
            .collect();

    // build a map of spawns using squad id as key
    unoccupied_spawns
        .iter()
        .for_each(|(spawn, squad_id, transform, _occupied)| {
            let vec = all_spawns.entry(*squad_id).or_default();
            vec.push((*spawn, *transform));
        });

    for (operator, operator_squad_id) in added_operators.iter() {
        if let Some(squad_id_spawns) = all_spawns.get_mut(operator_squad_id) {
            if let Some((spawn, transform)) = squad_id_spawns.first() {
                // TODO: reconsider instead of simple replacing, maybe copy only xz and
                // orientation instaed. not sure yet whos responsibility would it be.
                let new_transform = (*transform).clone();
                //new_transform.translation.y += 1.0;
                // currently the backpack is still wrong after above correction
                commands.entity(operator).insert(new_transform);
                commands.entity(*spawn).insert(SpawnPositionOccupied);
                squad_id_spawns.remove(0); // remove vec entry to avoid reuse
            }
        }
    }
}

fn update_spawn(
    mut _commands: Commands,
    spawns: Query<(&SpawnPosition, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    debug!("updating {}", NAME);

    for (_spawn, global_transform) in spawns.iter() {
        if cfg!(debug_assertions) {
            debug!("Debugging enabled");
            gizmos.arrow(
                global_transform.translation(),
                global_transform.translation() + Vec3::X,
                Srgba::rgb(0.0, 1.00, 0.0),
            );
            gizmos.circle(
                Isometry3d {
                    translation: global_transform.translation().into(),
                    rotation: global_transform.rotation(),
                    // TODO: correct rotation
                },
                1.,
                Srgba::rgb(0.0, 1.00, 0.0),
            );
        }
    }
}

fn bye_spawn(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_move_single_operator_to_single_spawn_position() {
        // given
        let mut app = App::new();
        let operator = app
            .world_mut()
            .spawn(Operator)
            .insert(Transform::from_xyz(0.0, 0.0, 0.0))
            .insert(SquadId(123))
            .id();
        let spawn_position = app
            .world_mut()
            .spawn(SpawnPosition)
            .insert(Transform::from_xyz(1.0, 2.0, 3.0))
            .insert(SquadId(123))
            .id();

        // when
        app.add_systems(Update, added_squad_id_to_operator);
        app.update();

        // then
        //assert!(app.world.get::<Health>(entity).is_some());
        assert_eq!(
            app.world().get::<Transform>(operator),
            app.world().get::<Transform>(spawn_position)
        );
    }

    #[test]
    fn should_move_two_operators_to_two_spawn_positions() {
        // given
        let mut app = App::new();
        let operator1 = app
            .world_mut()
            .spawn(Operator)
            .insert(Transform::from_xyz(0.0, 0.0, 0.0))
            .insert(SquadId(123))
            .id();
        let spawn_position1 = app
            .world_mut()
            .spawn(SpawnPosition)
            .insert(Transform::from_xyz(1.0, 2.0, 3.0))
            .insert(SquadId(123))
            .id();
        let operator2 = app
            .world_mut()
            .spawn(Operator)
            .insert(Transform::from_xyz(0.0, 0.0, 0.0))
            .insert(SquadId(123))
            .id();
        let spawn_position2 = app
            .world_mut()
            .spawn(SpawnPosition)
            .insert(Transform::from_xyz(2.0, 4.0, 6.0))
            .insert(SquadId(123))
            .id();

        // when
        app.add_systems(Update, added_squad_id_to_operator);
        app.update();

        // then
        assert_eq!(
            app.world().get::<Transform>(spawn_position1),
            app.world().get::<Transform>(operator1),
        );
        assert!(app
            .world()
            .get::<SpawnPositionOccupied>(spawn_position1)
            .is_some());
        assert_eq!(
            app.world().get::<Transform>(spawn_position2),
            app.world().get::<Transform>(operator2),
        );
        assert!(app
            .world()
            .get::<SpawnPositionOccupied>(spawn_position2)
            .is_some());
    }
}
