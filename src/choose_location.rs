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
    location_layout: Entity,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn start_choose_location_screen(mut commands: Commands) {
    debug!("starting choose location screen");

    // Layout
    // Top-level grid (app frame)
    let location_layout = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                grid_template_columns: vec![GridTrack::auto()],
                grid_template_rows: vec![
                    GridTrack::auto(),
                    GridTrack::flex(1.0),
                    GridTrack::px(20.),
                ],
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Main Layout"))
        .with_children(|builder| {
            // Header
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(12.0)),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new("Header"))
                .with_children(|builder| {
                    spawn_nested_text_bundle(builder, 40.0, "CHOOSE LOCATION");
                    spawn_nested_text_bundle(
                        builder,
                        10.0,
                        "Select your deployment location into the DMZ",
                    );
                });
            // Main
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(12.0)),
                        grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new("Main"))
                .with_children(|builder| {
                    spawn_location_button_bundle(
                        builder,
                        "Vondel",
                        ButtonTargetState(DeployScreen(ActiveMissions)),
                    );
                    spawn_location_button_bundle(
                        builder,
                        "Ashika Island",
                        ButtonTargetState(DeployScreen(ActiveMissions)),
                    );
                    spawn_location_button_bundle(
                        builder,
                        "Al Mazrah",
                        ButtonTargetState(DeployScreen(ActiveMissions)),
                    );
                    spawn_location_button_bundle(
                        builder,
                        "Building 21",
                        ButtonTargetState(DeployScreen(ActiveMissions)),
                    );
                });
            // Footer : TODO: if needed
        })
        .id();

    // insert resource
    commands.insert_resource(ChooseLocationMenuData { location_layout });
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
        .entity(menu_data.location_layout)
        .despawn_recursive();
}

fn spawn_nested_text_bundle(builder: &mut ChildBuilder, font_size: f32, text: &str) {
    builder.spawn(TextBundle::from_section(
        text,
        TextStyle {
            font_size,
            color: Color::rgb(0.9, 0.9, 0.9),
            ..default()
        },
    ));
}

fn spawn_location_button_bundle(
    builder: &mut ChildBuilder,
    button_text: &str,
    button_target_state: ButtonTargetState,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .insert(Name::new("TODO")) // TODO: figure out how to pass in button_text
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
                        button_text,
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                })
                .insert(button_target_state);
        });
}
