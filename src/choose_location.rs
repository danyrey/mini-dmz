use crate::AppState::DeployScreen;
use crate::DeployScreen::*;
use crate::{AppState, ButtonTargetState};
use bevy::prelude::*;

// Constants
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// Plugin
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

// Components

// Resources
#[derive(Resource)]
struct ChooseLocationMenuData {
    location_layout: Entity,
}

// TODO: make this visible in inspector
#[derive(Resource)]
pub struct ChosenLocation(String);

// Events

// Systems
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
                    let vondel_name = Name::new("Vondel");
                    spawn_location_button_bundle(
                        builder,
                        vondel_name.clone(),
                        vondel_name.as_str(),
                        ButtonTargetState(DeployScreen(ActiveMissions)),
                    );
                    let ashika_island_name = Name::new("Ashika Island");
                    spawn_location_button_bundle(
                        builder,
                        ashika_island_name.clone(),
                        ashika_island_name.as_str(),
                        ButtonTargetState(DeployScreen(ActiveMissions)),
                    );
                    let al_mazrah_name = Name::new("Al Mazrah");
                    spawn_location_button_bundle(
                        builder,
                        al_mazrah_name.clone(),
                        al_mazrah_name.as_str(),
                        ButtonTargetState(DeployScreen(ActiveMissions)),
                    );
                    let building_21_name = Name::new("Building 21");
                    spawn_location_button_bundle(
                        builder,
                        building_21_name.clone(),
                        building_21_name.as_str(),
                        ButtonTargetState(DeployScreen(ActiveMissions)),
                    );
                });
        })
        .id();

    // insert resource
    commands.insert_resource(ChooseLocationMenuData { location_layout });
}

fn update_choose_location_screen(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonTargetState,
            &Name,
        ),
        (Changed<Interaction>, (With<Button>, With<Name>)),
    >,
) {
    debug!("updating choose location screen");
    for (interaction, mut color, target_state, name) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                debug!("button pressed, target_state: {:?}", target_state);
                *color = PRESSED_BUTTON.into();
                commands.insert_resource(ChosenLocation(name.to_string().clone()));
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

fn bye_choose_location_screen(
    mut commands: Commands,
    menu_data: Res<ChooseLocationMenuData>,
    chosen_location: Res<ChosenLocation>,
) {
    debug!("exiting choose location screen");
    debug!("chosen location: {}", chosen_location.0);
    commands
        .entity(menu_data.location_layout)
        .despawn_recursive();
}

// helper functions
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
    button_name_component: Name,
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
        .insert(button_name_component.clone())
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
                .insert(button_name_component)
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
