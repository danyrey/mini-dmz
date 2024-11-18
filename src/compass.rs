use bevy::app::Plugin;

use crate::exfil::Operator;
use crate::first_person_controller::PlayerControlled;
use crate::raid::RaidState;
use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "compass";

// Plugin
pub struct CompassPlugin;

impl Plugin for CompassPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_compass_system)
            .add_systems(
                Update,
                (update_compass_system, update_compass_ui)
                    .chain()
                    .run_if(in_state(AppState::Raid))
                    .run_if(in_state(RaidState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_compass_system);
    }
}

// Components

#[derive(Component)]
struct CompassUI;

#[derive(Component)]
struct CompassLabel;

#[derive(Component, Default)]
pub struct Compass {
    pub heading: i32,
    pub direction: Direction,
}

#[derive(Debug, Default)]
pub enum Direction {
    #[default]
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl From<i32> for Direction {
    fn from(value: i32) -> Self {
        let angle = value % 360;
        match angle {
            0..=22 => Self::N,
            23..=67 => Self::NE,
            68..=112 => Self::E,
            113..=157 => Self::SE,
            158..=202 => Self::S,
            203..=247 => Self::SW,
            248..=292 => Self::W,
            293..=337 => Self::NW,
            338..=359 => Self::N,
            _ => Self::N,
        }
    }
}

// Resources
#[derive(Resource)]
struct CompassEntities {
    #[allow(dead_code)]
    heading_direction: Entity,
}

// Events

// Systems
fn start_compass_system(mut commands: Commands) {
    debug!("starting {}", NAME);
    let heading_direction = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                flex_direction: FlexDirection::Column,
                justify_self: JustifySelf::Center,
                ..default()
            },
            //background_color: BLUE.into(),
            ..default()
        })
        .insert(CompassUI)
        .insert(Name::new("Compass Layout"))
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle::from_section(
                        String::from("TEST"),
                        TextStyle {
                            font_size: 10.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    )
                    .with_text_justify(JustifyText::Center),
                )
                .insert(CompassLabel);
        })
        .id();

    commands.insert_resource(CompassEntities { heading_direction });
}

#[allow(clippy::type_complexity)]
fn update_compass_system(
    mut operator_query: Query<
        (&GlobalTransform, &mut Compass),
        (With<Operator>, With<PlayerControlled>),
    >,
) {
    for (o, mut c) in operator_query.iter_mut() {
        debug!("updating {}", NAME);
        let angle = -o
            .to_scale_rotation_translation()
            .1
            .to_euler(EulerRot::YXZ)
            .0
            .to_degrees() as i32;
        let heading = if angle < 0 { 360 + angle } else { angle };
        c.heading = heading;
        c.direction = Direction::from(heading);
        debug!("compass angle {}", c.heading);
        debug!("compass direction {:?}", c.direction);
    }
}

fn update_compass_ui(
    operator_query: Query<&Compass, (With<Operator>, With<PlayerControlled>)>,
    mut ui_query: Query<&mut Text, With<CompassLabel>>,
) {
    let compass = operator_query.single();
    let mut label = ui_query.single_mut();
    label.sections[0].value = format!("{:?}{}", compass.direction, compass.heading);
}

fn bye_compass_system(mut commands: Commands, compass_ui: Query<Entity, With<CompassUI>>) {
    debug!("stopping {}", NAME);
    let ui = compass_ui.single();
    commands.entity(ui).despawn_recursive();
    commands.remove_resource::<CompassEntities>();
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

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
