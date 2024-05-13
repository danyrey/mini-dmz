use crate::{
    AppState::{self, *},
    ButtonTargetState,
    DeployScreen::*,
    MissionObjectives::{self, Start},
};
use bevy::prelude::*;

// --- Start Screen START

pub struct StartScreenPlugin;

impl Plugin for StartScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(StartScreen), start_start_screen)
            .add_systems(Update, (update_start_screen).run_if(in_state(StartScreen)))
            .add_systems(OnExit(StartScreen), bye_start_screen);
    }
}

#[derive(Resource)]
struct MenuData {
    deploy_button_entity: Entity,
    mission_objective_button_entity: Entity,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_INACTIVE_BUTTON: Color = Color::rgb(1.0, 0.0, 0.0);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn start_start_screen(mut commands: Commands) {
    debug!("starting start screen");
    let deploy_button_entity = commands
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
                        "Deploy",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
            //.insert(ButtonTargetState(DeployScreen(ChooseLocation)));
        })
        .id();

    let mission_objective_button_entity = commands
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
                        "Mission Objectives",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                })
                .insert(ButtonTargetState(MissionObjectives(Start)));
        })
        .id();

    commands.insert_resource(MenuData {
        deploy_button_entity,
        mission_objective_button_entity,
    });
}

fn update_start_screen(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonTargetState),
        (Changed<Interaction>, With<Button>),
    >,
) {
    debug!("updating start screen");
    for (interaction, mut color, target_state) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                debug!("button pressed");
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

fn bye_start_screen(mut commands: Commands, menu_data: Res<MenuData>) {
    debug!("bye start screen!");
    commands
        .entity(menu_data.deploy_button_entity)
        .despawn_recursive();
    commands
        .entity(menu_data.mission_objective_button_entity)
        .despawn_recursive();
}

// --- Start Screen STOP
