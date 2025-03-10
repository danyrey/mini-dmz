use crate::AppState::DeployScreen;
use crate::DeployScreen::*;
use crate::{AppState, ButtonTargetState};
use bevy::prelude::*;

pub struct ActiveMissionsScreenPlugin;

impl Plugin for ActiveMissionsScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(DeployScreen(ActiveMissions)),
            start_active_missions_screen,
        )
        .add_systems(
            Update,
            (update_active_missions_screen).run_if(in_state(DeployScreen(ActiveMissions))),
        )
        .add_systems(
            OnExit(DeployScreen(ActiveMissions)),
            bye_active_missions_screen,
        );
    }
}

#[derive(Resource)]
struct ActiveMissionsMenuData {
    active_missions_layout: Entity,
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn start_active_missions_screen(mut commands: Commands) {
    debug!("starting active missions screen");

    // Layout
    // Top-level grid (app frame)
    let active_missions_layout = commands
        .spawn(Node {
            display: Display::Grid,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            grid_template_columns: vec![GridTrack::auto()],
            grid_template_rows: vec![GridTrack::auto(), GridTrack::flex(1.0), GridTrack::px(20.)],
            ..default()
        })
        .insert(Name::new("Main Layout"))
        .with_children(|builder| {
            // Header
            builder
                .spawn(Node {
                    display: Display::Grid,
                    justify_items: JustifyItems::Center,
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                })
                .insert(Name::new("Header"))
                .with_children(|builder| {
                    spawn_nested_text_bundle(builder, 40.0, "ACTIVE MISSIONS");
                    spawn_nested_text_bundle(builder, 10.0, "0/3 MISSIONS SELECTED");
                });
            // Main
            builder
                .spawn(Node {
                    display: Display::Grid,
                    justify_items: JustifyItems::Center,
                    padding: UiRect::all(Val::Px(12.0)),
                    grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                    ..default()
                })
                .insert(Name::new("Main"))
                .with_children(|builder| {
                    let confirm_name = Name::new("CONFIRM");
                    spawn_location_button_bundle(
                        builder,
                        confirm_name.clone(),
                        confirm_name.as_str(),
                        ButtonTargetState(DeployScreen(ActiveDutyConfirmation)),
                    );
                    let edit_mission_name = Name::new("EDIT MISSIONS");
                    spawn_location_button_bundle(
                        builder,
                        edit_mission_name.clone(),
                        edit_mission_name.as_str(),
                        ButtonTargetState(DeployScreen(EditMissions)),
                    );
                    let cancel_name = Name::new("CANCEL");
                    spawn_location_button_bundle(
                        builder,
                        cancel_name.clone(),
                        cancel_name.as_str(),
                        ButtonTargetState(DeployScreen(ChooseLocation)),
                    );
                });
        })
        .id();

    commands.insert_resource(ActiveMissionsMenuData {
        active_missions_layout,
    });
}

#[allow(clippy::type_complexity)]
fn update_active_missions_screen(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonTargetState),
        (Changed<Interaction>, With<Button>),
    >,
) {
    debug!("updating active missions screen");
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

fn bye_active_missions_screen(mut commands: Commands, menu_data: Res<ActiveMissionsMenuData>) {
    debug!("exiting active missions screen");
    commands
        .entity(menu_data.active_missions_layout)
        .despawn_recursive();
}

// helper functions
fn spawn_nested_text_bundle(builder: &mut ChildBuilder, font_size: f32, text: &str) {
    builder
        .spawn(Text::new(text))
        .insert(TextFont {
            font_size,
            ..default()
        })
        .insert(TextColor(Color::srgb(0.9, 0.9, 0.9)));
}

fn spawn_location_button_bundle(
    builder: &mut ChildBuilder,
    button_name_component: Name,
    button_text: &str,
    button_target_state: ButtonTargetState,
) {
    builder
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .insert(button_name_component.clone())
        .with_children(|parent| {
            parent
                .spawn(Button)
                .insert(Node {
                    width: Val::Px(150.),
                    height: Val::Px(110.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                // TODO: not sure this is how it works, see migration guide
                .insert(ImageNode {
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(button_name_component)
                .with_children(|parent| {
                    parent
                        .spawn(Text::new(button_text))
                        .insert(TextFont {
                            font_size: 40.0,
                            ..default()
                        })
                        .insert(TextColor(Color::srgb(0.9, 0.9, 0.9)));
                })
                .insert(button_target_state);
        });
}
