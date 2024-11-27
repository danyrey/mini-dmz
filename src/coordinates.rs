use bevy::app::Plugin;

use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "template";

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
struct GridCoordinate {
    row: Option<Row>,
    column: Option<Column>,
}

#[derive(Debug, PartialEq)]
struct Row(i32);

#[derive(Debug, PartialEq)]
struct Column(char);

struct MapGrid {
    // TODO: offset, scale, orientation?
}

// TODO: find out scaling factor on al mazrah and other maps, just assume a random number for now
// TODO: making it scaleable later, first go with a fixed version
impl From<Vec3> for Row {
    fn from(value: Vec3) -> Self {
        let z = (value.z / 100.0) + 1.0;
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
        if x <= 0.0 {
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

fn update_coordinate_system() {
    debug!("updating {}", NAME);
}

fn bye_coordinate_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    use bevy::math::Vec3;

    use crate::coordinates::{Column, Row};

    #[test]
    fn should_convert_row_from_vec3_positive_range() {
        // given
        let y0 = Vec3::new(0.0, 0.0, 0.0);
        let y50 = Vec3::new(0.0, 0.0, 50.0);
        let y150 = Vec3::new(0.0, 0.0, 150.0);
        let y250 = Vec3::new(0.0, 0.0, 250.0);

        // when / then
        assert_eq!(Row(1), Row::from(y0));
        assert_eq!(Row(1), Row::from(y50));
        assert_eq!(Row(2), Row::from(y150));
        assert_eq!(Row(3), Row::from(y250));
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
