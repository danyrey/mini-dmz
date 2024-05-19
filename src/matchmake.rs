// TODO: 'matchmake' will start the matchmake. all operators in the party need to be on ready
// state. the party leader and matchmaker is ready by default and i think he can forcibly matchmake
// if not all members are ready, not sure.
// TODO: more research for corner cases. make screenshots for when not the whole team is not ready.
// TODO: messages during matchmaking need to be displayed until we can advance to loading screen

use crate::AppState::DeployScreen;
use crate::DeployScreen::*;
use crate::{AppState, ButtonTargetState};
use bevy::prelude::*;

// Events

#[derive(Event)]
struct MatchmakingStarted;

#[derive(Event)]
struct MatchmakingUpdate(u32); // new ping we are currently searching for

#[derive(Event)]
struct MatchFound;

#[derive(Event)]
struct FoundPlayersUpdate(u32); // total amount of players found

#[derive(Event)]
struct LobbyFilled;

#[derive(Event)]
struct LevelLoaded;

// Matchmake screen

pub struct MatchmakeScreenPlugin;

impl Plugin for MatchmakeScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(DeployScreen(MatchMake)), start_matchmake_screen)
            .add_systems(
                Update,
                (update_matchmake_screen).run_if(in_state(DeployScreen(MatchMake))),
            )
            .add_systems(OnExit(DeployScreen(MatchMake)), bye_matchmake_screen);
    }
}

#[derive(Resource)]
struct MatchmakeMenuData {
    matchmake_button_entity: Entity,
    back_button_entity: Entity,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn start_matchmake_screen(mut commands: Commands) {
    debug!("starting matchmake screen");
    let matchmake_button_entity = commands
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
                        "MATCHMAKE",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                })
                .insert(ButtonTargetState(DeployScreen(MatchMakeInProgress)));
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
                .insert(ButtonTargetState(DeployScreen(ActiveDutyConfirmation)));
        })
        .id();

    commands.insert_resource(MatchmakeMenuData {
        matchmake_button_entity,
        back_button_entity,
    });

    commands
        .entity(matchmake_button_entity)
        .insert(Name::new("Confirm Button"));
    commands
        .entity(back_button_entity)
        .insert(Name::new("Back Button"));
}

fn update_matchmake_screen(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonTargetState),
        (Changed<Interaction>, With<Button>),
    >,
) {
    debug!("updating matchmake screen");
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

fn bye_matchmake_screen(mut commands: Commands, menu_data: Res<MatchmakeMenuData>) {
    debug!("exiting matchmake screen");
    commands
        .entity(menu_data.matchmake_button_entity)
        .despawn_recursive();
    commands
        .entity(menu_data.back_button_entity)
        .despawn_recursive();
}

// Matchmake in progress screen
// this is the screen you seen when you clicked the matchmake button
// and will show the notifications from the matchmake server

pub struct MatchmakeInProgressScreenPlugin;

#[derive(Resource)]
struct MatchmakeInProgressMenuData {
    matchmake_messagebox_entity: Entity,
}

impl Plugin for MatchmakeInProgressScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(DeployScreen(MatchMakeInProgress)),
            start_matchmake_in_progress_screen,
        )
        .add_systems(
            Update,
            (update_matchmake_in_progress_screen, matchmaking_started)
                .run_if(in_state(DeployScreen(MatchMakeInProgress))),
        )
        .add_systems(
            FixedUpdate,
            (update_fake_matchmake_server).run_if(in_state(DeployScreen(MatchMakeInProgress))),
        )
        .add_systems(
            OnExit(DeployScreen(MatchMakeInProgress)),
            bye_matchmake_in_progress_screen,
        )
        .add_event::<MatchmakingStarted>()
        .add_event::<MatchmakingUpdate>()
        .add_event::<MatchFound>()
        .add_event::<FoundPlayersUpdate>()
        .add_event::<LobbyFilled>()
        .add_event::<LevelLoaded>()
        // TODO: research how to have multiple fixed time
        // schedules and not just one
        .insert_resource(Time::<Fixed>::from_seconds(1.0));
    }
}

fn start_matchmake_in_progress_screen(
    mut commands: Commands,
    mut matchmake_started_event: EventWriter<MatchmakingStarted>,
) {
    debug!("starting matchmake in progress screen");
    let matchmake_messagebox_entity = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Start,
                flex_grow: 1.,
                margin: UiRect::axes(Val::Px(15.), Val::Px(5.)),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "This is\ntext with\nline breaks\nin the top left.",
                TextStyle {
                    font_size: 30.0,
                    ..default()
                },
            ));
        })
        .id();

    commands.insert_resource(MatchmakeInProgressMenuData {
        matchmake_messagebox_entity,
    });

    commands.entity(matchmake_messagebox_entity);
    matchmake_started_event.send(MatchmakingStarted);
}

fn update_matchmake_in_progress_screen(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<&mut Text, With<Text>>,
) {
    debug!("updating matchmake in progress screen");
    // TODO: add update to progress via message box
    // for now just jump to loading screen instead
    for mut _text in &mut interaction_query {
        debug!("updating text in message box");
    }
    //next_state.set(AppState::LoadingScreen); // TODO: move this to the LevelLoaded event reader
}

fn bye_matchmake_in_progress_screen(
    mut commands: Commands,
    menu_data: Res<MatchmakeInProgressMenuData>,
) {
    debug!("exiting matchmake in progress screen");
    commands
        .entity(menu_data.matchmake_messagebox_entity)
        .despawn_recursive();
    commands.remove_resource::<MatchmakeInProgressMenuData>();
}

// TODO: add a fake system that periodically sends out the events
// that would usually come from the network communication with a
// matchmaking server. this system runs on a timer
fn update_fake_matchmake_server(
    time_fixed: Res<Time<Fixed>>,
    mut started: EventWriter<MatchmakingStarted>,
    mut update: EventWriter<MatchmakingUpdate>, // new ping we are currently searching for
    mut match_found: EventWriter<MatchFound>,
    mut players_found: EventWriter<FoundPlayersUpdate>, // total amount of players found
    mut filled: EventWriter<LobbyFilled>,
    mut loaded: EventWriter<LevelLoaded>,
) {
    debug!(
        "fake matchmake server update. fixed time: {:?}",
        time_fixed.elapsed()
    );
    // TODO: create a queue/stack of some sort and every seconds send out the next event
}

// TODO: add other event readers
fn matchmaking_started(mut event: EventReader<MatchmakingStarted>) {
    for ev in event.read() {
        // TODO: change textbox contents
        debug!("matchmaking started");
    }
}
