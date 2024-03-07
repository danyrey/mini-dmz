use crate::AppState;
use crate::AppState::MissionObjectives;
use crate::MissionObjectives::*;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct MissionObjectivesScreenPlugin;

impl Plugin for MissionObjectivesScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MissionObjectives(Start)),
            start_mission_objectives_screen,
        )
        .add_systems(
            Update,
            (update_mission_objectives_screen).run_if(in_state(MissionObjectives(Start))),
        )
        .add_systems(
            OnExit(MissionObjectives(Start)),
            bye_mission_objective_screen,
        );
        app.add_plugins(WorldInspectorPlugin::new());
    }
}

// TODO : enable proper inspector output, currently it shows:
// "ButtonTargetState is not registered in the TypeRegistry"
#[derive(Component, Debug)]
struct ButtonTargetState(AppState);

#[derive(Resource)]
struct MissionObjectiveMenuData {
    missions_button_entity: Entity,
    upgrades_button_entity: Entity,
    location_objectives_button_entity: Entity,
    notes_button_entity: Entity,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn start_mission_objectives_screen(mut commands: Commands) {
    debug!("starting mission objectives screen");
    let missions_button_entity = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                width: Val::Percent(30.),
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
                        "Missions",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                })
                .insert(ButtonTargetState(MissionObjectives(Missions)));
        })
        .id();

    let upgrades_button_entity = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                width: Val::Percent(50.),
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
                        width: Val::Px(220.),
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
                        "Upgrades",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                })
                .insert(ButtonTargetState(MissionObjectives(Upgrades)));
        })
        .id();

    let location_objectives_button_entity = commands
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
                        width: Val::Px(220.),
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
                        "Location Objectives",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                })
                .insert(ButtonTargetState(MissionObjectives(LocationObjectives)));
        })
        .id();

    let notes_button_entity = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                width: Val::Percent(110.),
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
                        width: Val::Px(220.),
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
                        "Notes",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                })
                .insert(ButtonTargetState(MissionObjectives(Notes)));
        })
        .id();

    commands.insert_resource(MissionObjectiveMenuData {
        missions_button_entity,
        upgrades_button_entity,
        location_objectives_button_entity,
        notes_button_entity,
    });

    commands
        .entity(missions_button_entity)
        .insert(Name::new("Mission Button"));

    commands
        .entity(upgrades_button_entity)
        .insert(Name::new("Upgrades Button"));

    commands
        .entity(location_objectives_button_entity)
        .insert(Name::new("Location Objectives Button"));

    commands
        .entity(notes_button_entity)
        .insert(Name::new("Notes Button"));
}

fn update_mission_objectives_screen(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonTargetState),
        (Changed<Interaction>, With<Button>),
    >,
) {
    debug!("updating mission objectives screen");
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

fn bye_mission_objective_screen(mut commands: Commands, menu_data: Res<MissionObjectiveMenuData>) {
    debug!("bye mission objectives screen!");
    commands
        .entity(menu_data.missions_button_entity)
        .despawn_recursive();
    commands
        .entity(menu_data.upgrades_button_entity)
        .despawn_recursive();
    commands
        .entity(menu_data.location_objectives_button_entity)
        .despawn_recursive();
    commands
        .entity(menu_data.notes_button_entity)
        .despawn_recursive();
}
