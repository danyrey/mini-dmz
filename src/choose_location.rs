use crate::AppState::DeployScreen;
use crate::DeployScreen::*;
use crate::{AppState, ButtonTargetState};
use bevy::prelude::*;

// TODO: click on a button saves the selection somehow and advances to missions

pub struct ChooseLocationScreenPlugin;

impl Plugin for ChooseLocationScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(DeployScreen(ChooseLocation)),
            start_choose_location_screen,
        )
        .add_systems(
            Update,
            (update_choose_location_screen).run_if(in_state(DeployScreen(ChooseLocation))),
        )
        .add_systems(
            OnExit(DeployScreen(ChooseLocation)),
            bye_choose_location_screen,
        );
    }
}

#[derive(Resource)]
struct ChooseLocationMenuData {
    vondel_button_entity: Entity,
    ashika_island_button_entity: Entity,
    al_mazrah_button_entity: Entity,
    building_21_button_entity: Entity,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn start_choose_location_screen(mut commands: Commands) {
    debug!("starting choose location screen");
    let vondel_button_entity = commands
        .spawn(NodeBundle {
            style: Style {
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
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Vondel",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                })
                .insert(ButtonTargetState(DeployScreen(ActiveMissions)));
        })
        .id();

    let ashika_island_button_entity = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(60.),
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
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Ashika Island",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                })
                .insert(ButtonTargetState(DeployScreen(ActiveMissions)));
        })
        .id();

    let al_mazrah_button_entity = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(90.),
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
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Al Mazrah",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                })
                .insert(ButtonTargetState(DeployScreen(ActiveMissions)));
        })
        .id();

    let building_21_button_entity = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(120.),
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
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Building 21",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                })
                .insert(ButtonTargetState(DeployScreen(ActiveMissions)));
        })
        .id();

    commands.insert_resource(ChooseLocationMenuData {
        vondel_button_entity,
        ashika_island_button_entity,
        al_mazrah_button_entity,
        building_21_button_entity,
    });

    commands
        .entity(vondel_button_entity)
        .insert(Name::new("Vondel Button"));
    commands
        .entity(ashika_island_button_entity)
        .insert(Name::new("Ashika Button"));
    commands
        .entity(al_mazrah_button_entity)
        .insert(Name::new("Al Mazrah Button"));
    commands
        .entity(building_21_button_entity)
        .insert(Name::new("B21 Button"));
}

fn update_choose_location_screen(
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

fn bye_choose_location_screen(mut commands: Commands, menu_data: Res<ChooseLocationMenuData>) {
    debug!("exiting choose location screen");
    commands
        .entity(menu_data.vondel_button_entity)
        .despawn_recursive();
    commands
        .entity(menu_data.ashika_island_button_entity)
        .despawn_recursive();
    commands
        .entity(menu_data.al_mazrah_button_entity)
        .despawn_recursive();
    commands
        .entity(menu_data.building_21_button_entity)
        .despawn_recursive();
}
