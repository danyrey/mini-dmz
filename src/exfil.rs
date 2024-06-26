use bevy::prelude::*;

use crate::{
    AppState::{self, Raid, StartScreen},
    ButtonTargetState,
};

// Events

// trigger for showing the prompt
#[derive(Event)]
pub struct ExfilAreaEntered {
    pub operator_entity: Entity,
    pub exfil_area: ExfilArea,
}

// trigger for hiding the prompt again
#[derive(Event)]
pub struct ExfilAreaExited {
    pub operator_entity: Entity,
    pub exfil_area: ExfilArea,
}

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

#[derive(Component, Clone, Debug, Default)]
pub struct ExfilArea(pub String);

#[derive(Component)]
struct ExfilButton;

// potential issues: overlapping/multiple exfils might be an issue or not
#[derive(Component, Clone, Debug, Default)]
pub struct InsideExfilArea(ExfilArea);

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
            visibility: Visibility::Hidden,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.),
                        height: Val::Px(110.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "EXFIL",
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
        .insert(Name::new("Exfil Button"))
        .insert(ExfilButton);
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

fn exfil_area_entered(
    mut commands: Commands,
    mut entered: EventReader<ExfilAreaEntered>,
    query: Query<Entity, (With<ExfilButton>, With<Visibility>)>,
) {
    for event in entered.read() {
        debug!("entered exfil zone");
        for entity in query.iter() {
            debug!(
                "operator({:?}) entered exfil zone({:?})",
                event.operator_entity, event.exfil_area
            );
            commands.entity(entity).insert(Visibility::Visible);
        }
    }
}

fn exfil_area_exited(
    mut commands: Commands,
    mut exited: EventReader<ExfilAreaExited>,
    query: Query<Entity, (With<ExfilButton>, With<Visibility>)>,
) {
    for event in exited.read() {
        debug!("exited exfil zone");
        for entity in query.iter() {
            debug!(
                "operator({:?}) exited exfil zone({:?})",
                event.operator_entity, event.exfil_area
            );
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
}

fn exfil_area_collision_detection(
    mut commands: Commands,
    exfil_query: Query<(Entity, &GlobalTransform, &ExfilArea), With<ExfilArea>>,
    mut operator_query: Query<
        (Entity, &GlobalTransform, Option<&mut InsideExfilArea>),
        With<Operator>,
    >,
    mut entered: EventWriter<ExfilAreaEntered>,
    mut exited: EventWriter<ExfilAreaExited>,
) {
    // TODO: make it a resource maybe
    let min_distance = 5.0;

    for (operator_entity, operator_transform, operator_exfil_area) in operator_query.iter_mut() {
        let mut any_exfil_area: Option<ExfilArea> = None;

        for (_exfil_entity, exfil_transform, exfil_area) in exfil_query.iter() {
            let distance = exfil_transform
                .translation()
                .distance(operator_transform.translation());

            if distance < min_distance {
                any_exfil_area = Some(exfil_area.clone());
            }
        }

        if let Some(area) = any_exfil_area {
            if let Some(mut component_inside) = operator_exfil_area {
                if !area.0.eq(&(component_inside.0).0) {
                    component_inside.0 = area.clone();
                    entered.send(ExfilAreaEntered {
                        operator_entity,
                        exfil_area: area.clone(),
                    });
                }
            } else {
                commands
                    .entity(operator_entity)
                    .insert(InsideExfilArea(area.clone()));
                entered.send(ExfilAreaEntered {
                    operator_entity,
                    exfil_area: area.clone(),
                });
            }
        } else {
            if let Some(component) = operator_exfil_area {
                commands.entity(operator_entity).remove::<InsideExfilArea>();
                exited.send(ExfilAreaExited {
                    operator_entity,
                    exfil_area: component.0.clone(),
                });
            }
        }
    }
}
