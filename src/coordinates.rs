use bevy::app::Plugin;

use crate::exfil::Operator;
use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "coordinates";

// Plugin
pub struct CoordinatesPlugin;

impl Plugin for CoordinatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_coordinate_system)
            .add_systems(
                Update,
                (update_coordinate_system).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_coordinate_system);
    }
}

// General Structs/Enums
/// Grid Coordinates that allow for partial coordinates.
/// examples:
/// ```
/// // complete known grid coordinate
/// GridCoordinate {
///     row: Some(Row(2)),
///     column: Some(Column('B')),
/// }
///
/// // partial known coordinates
/// GridCoordinate {
///     row: None,
///     column: Some(Column('C')),
/// }
///
/// GridCoordinate {
///     row: Some(Row(3)),
///     column: None,
/// }
/// ```
#[derive(Component)]
pub struct GridCoordinate {
    row: Option<Row>,
    column: Option<Column>,
}

#[derive(Debug, PartialEq)]
pub struct Row(i32);

#[derive(Debug, PartialEq)]
pub struct Column(char);

/// GridPosition will follow the bevy ui 2D coordinate system:
/// top left is origin
/// y to down is positive
/// x to right is positive
#[derive(Component, Default, Debug, PartialEq)]
struct GridPosition {
    pub position: Vec2,
}

#[derive(Resource)]
struct GridOffset(Vec2);

#[derive(Resource)]
struct GridScale(f32);

#[derive(Resource)]
struct GridRotation(f32);

// TODO: find out scaling factor on al mazrah and other maps, just assume a random number for now
// TODO: making it scaleable later, first go with a fixed version
impl From<Vec3> for Row {
    fn from(value: Vec3) -> Self {
        let z = value.z / 100.0;
        if z > 0.0 {
            Row(z as i32)
        } else {
            Row(0)
        }
    }
}

impl From<Vec3> for Column {
    fn from(value: Vec3) -> Self {
        let start = 'A';
        let x = value.x / 100.0;
        if (x <= 0.0) && (x > -26.0) {
            let offset = -x as u32;
            let char_number = start as u32 + offset;
            let c = char::from_u32(char_number);
            Column(c.unwrap_or('-'))
        } else {
            Column('-')
        }
    }
}

// Components

// Resources

// Events

// Systems
fn start_coordinate_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn update_coordinate_system(
    mut operators: Query<(&GlobalTransform, &mut GridPosition), With<Operator>>,
    offset: Option<Res<GridOffset>>,
) {
    debug!("updating {}", NAME);
    for (transform, mut position) in operators.iter_mut() {
        let offset_x = offset.as_ref().map_or(0.0, |o| o.0.x);
        let offset_y = offset.as_ref().map_or(0.0, |o| o.0.y);
        position.position.x = transform.translation().x + offset_x;
        position.position.y = transform.translation().z + offset_y;
    }
}

fn bye_coordinate_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    use crate::exfil::Operator;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_convert_row_from_vec3_positive_range() {
        // given
        let y0 = Vec3::new(0.0, 0.0, 0.0);
        let y50 = Vec3::new(0.0, 0.0, 50.0);
        let y150 = Vec3::new(0.0, 0.0, 150.0);
        let y250 = Vec3::new(0.0, 0.0, 250.0);

        // when / then
        assert_eq!(Row(0), Row::from(y0));
        assert_eq!(Row(0), Row::from(y50));
        assert_eq!(Row(1), Row::from(y150));
        assert_eq!(Row(2), Row::from(y250));
    }

    #[test]
    fn should_convert_row_from_vec3_negative_range() {
        // given
        let y50 = Vec3::new(0.0, 0.0, -50.0);
        let y150 = Vec3::new(0.0, 0.0, -150.0);
        let y250 = Vec3::new(0.0, 0.0, -250.0);

        // when / then
        assert_eq!(Row(0), Row::from(y50));
        assert_eq!(Row(0), Row::from(y150));
        assert_eq!(Row(0), Row::from(y250));
    }

    #[test]
    fn should_convert_column_from_vec3_negative_range() {
        // given
        let x0 = Vec3::new(0.0, 0.0, 0.0);
        let x50 = Vec3::new(-50.0, 0.0, 0.0);
        let x150 = Vec3::new(-150.0, 0.0, 0.0);
        let x250 = Vec3::new(-250.0, 0.0, 0.0);

        // when / then
        assert_eq!(Column('A'), Column::from(x0));
        assert_eq!(Column('A'), Column::from(x50));
        assert_eq!(Column('B'), Column::from(x150));
        assert_eq!(Column('C'), Column::from(x250));
    }

    #[test]
    fn should_convert_column_from_vec3_positive_range() {
        // given
        let x50 = Vec3::new(50.0, 0.0, 0.0);
        let x150 = Vec3::new(150.0, 0.0, 0.0);
        let x250 = Vec3::new(250.0, 0.0, 0.0);

        // when / then
        assert_eq!(Column('-'), Column::from(x50));
        assert_eq!(Column('-'), Column::from(x150));
        assert_eq!(Column('-'), Column::from(x250));
    }

    // exceeding 26 characters should result in '-'
    #[test]
    fn should_convert_column_from_vec3_exceeding_negative_range() {
        // given
        let x2550 = Vec3::new(-2550.0, 0.0, 0.0);
        let x2650 = Vec3::new(-2650.0, 0.0, 0.0);

        // when / then
        assert_eq!(Column('Z'), Column::from(x2550));
        assert_eq!(Column('-'), Column::from(x2650));
    }

    #[test]
    fn should_convert_grid_position_from_global_transform_no_modifiers() {
        // given
        let mut app = App::new();
        app.add_systems(Update, update_coordinate_system);
        let transform = Transform {
            translation: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
            ..default()
        };
        let operator = app
            .world_mut()
            .spawn(Operator)
            .insert(SpatialBundle {
                transform,
                global_transform: GlobalTransform::from(transform),
                ..default()
            })
            .insert(GridPosition::default())
            .id();

        // when
        app.update();

        // then
        let grid_position = app.world().get::<GridPosition>(operator).unwrap();
        assert_eq!(Vec2 { x: 1.0, y: 1.0 }, grid_position.position);
    }

    #[test]
    fn should_convert_grid_position_from_global_transform_offset_modifier() {
        // given
        let mut app = App::new();
        app.add_systems(Update, update_coordinate_system);
        let transform = Transform {
            translation: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
            ..default()
        };
        let offset = GridOffset(Vec2 { x: 1.0, y: 1.0 });
        app.insert_resource(offset);
        let operator = app
            .world_mut()
            .spawn(Operator)
            .insert(SpatialBundle {
                transform,
                global_transform: GlobalTransform::from(transform),
                ..default()
            })
            .insert(GridPosition::default())
            .id();

        // when
        app.update();

        // then
        let grid_position = app.world().get::<GridPosition>(operator).unwrap();
        assert_eq!(Vec2 { x: 2.0, y: 2.0 }, grid_position.position);
    }

    //#[test]
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
}
