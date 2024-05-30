// TODO: very basic exfil mechanic
// just use a button for calling exfil for now

use bevy::prelude::*;

use crate::{
    AppState::{self, Raid, StartScreen},
    ButtonTargetState,
};

// Events

#[derive(Event)]
pub struct ExfilAreaEntered; // trigger for showing the prompt

#[derive(Event)]
pub struct ExfilAreaExited; // trigger for hiding the prompt again

// TODO: Potential events for Exfil procedure
// ExfilCalled // trigger the flare and sound fx and hide prompt while exfil is in progress
// ExfilEnteredAO // trigger spawning of helicopter
// ExfilSpawned // trigger radio in of pilot
// ExfilApproached
// ExfilDescented
// ExfilLandingHovered
// ExfilTouchedDown
// ExfilFullyBoarded
// ExfilTakeOffHovered
// ExfilClimbed
// ExfilCruised
// ExfilExfilled
// ExfilCooldownComplete // after x amount of time smoke and prompt show up again

// Resources

// TODO: add simple ui "exfil button" for triggering the exfil procedure
#[derive(Resource)]
struct ExfilUIData {
    exfil_button_entity: Entity,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// TODO: counters for all other phases of exfil

// Plugin

pub struct ExfilPlugin;

impl Plugin for ExfilPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_exfil)
            .add_systems(
                Update,
                ((
                    update_exfil,
                    exfil_area_collision_detection,
                    exfil_area_event_handling,
                    exfil_area_entered,
                    exfil_area_exited,
                ))
                    .run_if(in_state(Raid)),
            )
            .add_systems(OnExit(Raid), bye_exfil)
            .add_event::<ExfilAreaEntered>()
            .add_event::<ExfilAreaExited>();
    }
}

// Components

#[derive(Component)]
pub struct ExfilArea;

#[derive(Component, Debug, Default)]
pub struct InsideExfilArea(bool);

#[derive(Component)]
pub struct Operator;

// Systems

fn start_exfil(mut commands: Commands) {
    debug!("starting exfil called");
    let exfil_button_entity = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                width: Val::Percent(80.),
                height: Val::Percent(120.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.),
                        height: Val::Px(110.),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "FIXME: EXFIL",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                })
                .insert(ButtonTargetState(StartScreen));
        })
        .id();

    commands.insert_resource(ExfilUIData {
        exfil_button_entity,
    });

    commands
        .entity(exfil_button_entity)
        .insert(Name::new("Exfil Button"));
}

fn update_exfil(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonTargetState),
        (Changed<Interaction>, With<Button>),
    >,
) {
    debug!("updating exfil called");
    for (interaction, mut color, target_state) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                debug!("button pressed, target_state: {:?}", target_state);
                *color = PRESSED_BUTTON.into();
                next_state.set(target_state.0.clone());
            }
            Interaction::Hovered => {
                debug!("button hovered");
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                debug!("button normal");
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn bye_exfil(mut commands: Commands, menu_data: Res<ExfilUIData>) {
    debug!("exiting exfil called");
    commands
        .entity(menu_data.exfil_button_entity)
        .despawn_recursive();
}

fn exfil_area_event_handling(
    query: Query<&InsideExfilArea, Changed<InsideExfilArea>>,
    mut entered: EventWriter<ExfilAreaEntered>,
    mut exited: EventWriter<ExfilAreaExited>,
) {
    for inside in query.iter() {
        if inside.0 {
            entered.send(ExfilAreaEntered);
        } else {
            exited.send(ExfilAreaExited);
        }
    }
}

fn exfil_area_entered(mut entered: EventReader<ExfilAreaEntered>) {
    for _event in entered.read() {
        debug!("entered exfil zone");
    }
}

fn exfil_area_exited(mut exited: EventReader<ExfilAreaExited>) {
    for _event in exited.read() {
        debug!("exited exfil zone");
    }
}

fn exfil_area_collision_detection(
    mut commands: Commands,
    exfil_query: Query<&GlobalTransform, With<ExfilArea>>,
    mut operator_query: Query<
        (Entity, &GlobalTransform, Option<&mut InsideExfilArea>),
        With<Operator>,
    >,
) {
    // TODO: make it a resource maybe
    let min_distance = 5.0;

    for exfil_transform in exfil_query.iter() {
        for (entity, operator_transform, inside) in operator_query.iter_mut() {
            let distance = exfil_transform
                .translation()
                .distance(operator_transform.translation());

            let mut i = InsideExfilArea(default());

            i.0 = distance < min_distance;

            if let Some(mut ins) = inside {
                // needed otherwise it is detected as a change even for the same value
                if ins.0 != i.0 {
                    ins.0 = i.0;
                }
            } else {
                commands.entity(entity).insert(i);
            }
        }
    }
}
