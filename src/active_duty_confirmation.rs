use crate::AppState::DeployScreen;
use crate::DeployScreen::*;
use crate::{AppState, ButtonTargetState};
use bevy::prelude::*;

pub struct ActiveDutyConfirmationScreenPlugin;

impl Plugin for ActiveDutyConfirmationScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(DeployScreen(ActiveDutyConfirmation)),
            start_active_duty_confirmation_screen,
        )
        .add_systems(
            Update,
            (update_active_duty_confirmation_screen)
                .run_if(in_state(DeployScreen(ActiveDutyConfirmation))),
        )
        .add_systems(
            OnExit(DeployScreen(ActiveDutyConfirmation)),
            bye_active_duty_confirmation_screen,
        );
    }
}

#[derive(Resource)]
struct ActiveDutyConfirmationMenuData {
    confirm_button_entity: Entity,
    edit_button_entity: Entity,
    back_button_entity: Entity,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn start_active_duty_confirmation_screen(mut commands: Commands) {
    debug!("starting active duty confirmation screen");
    let confirm_button_entity = commands
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
                        "CONFIRM",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                })
                .insert(ButtonTargetState(DeployScreen(MatchMake)));
        })
        .id();

    let edit_button_entity = commands
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
                        "EDIT",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                })
                .insert(ButtonTargetState(DeployScreen(EditLoadout)));
        })
        .id();

    let back_button_entity = commands
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
                        "BACK",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                })
                // TODO: change it back to missions once missions is implemented
                .insert(ButtonTargetState(DeployScreen(ChooseLocation)));
        })
        .id();

    commands.insert_resource(ActiveDutyConfirmationMenuData {
        confirm_button_entity,
        edit_button_entity,
        back_button_entity,
    });

    commands
        .entity(confirm_button_entity)
        .insert(Name::new("Confirm Button"));
    commands
        .entity(edit_button_entity)
        .insert(Name::new("Edit Button"));
    commands
        .entity(back_button_entity)
        .insert(Name::new("Back Button"));
}

fn update_active_duty_confirmation_screen(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonTargetState),
        (Changed<Interaction>, With<Button>),
    >,
) {
    debug!("updating choose location screen");
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

fn bye_active_duty_confirmation_screen(
    mut commands: Commands,
    menu_data: Res<ActiveDutyConfirmationMenuData>,
) {
    debug!("exiting active duty confirmation screen");
    commands
        .entity(menu_data.confirm_button_entity)
        .despawn_recursive();
    commands
        .entity(menu_data.edit_button_entity)
        .despawn_recursive();
    commands
        .entity(menu_data.back_button_entity)
        .despawn_recursive();
}
