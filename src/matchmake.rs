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
struct PlayersFoundUpdate(u32); // total amount of players found

#[derive(Event)]
struct LobbyFilled;

#[derive(Event)]
struct LevelLoaded;

#[derive(Event)]
struct Launching(u32);

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

// Components

#[derive(Component, Debug)]
struct MessageTextMarker;

// Resources

#[derive(Resource)]
struct MatchmakeInProgressMenuData {
    matchmake_messagebox_entity: Entity,
}

#[derive(Resource, Default)]
struct EventCounter(u32);

impl Plugin for MatchmakeInProgressScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(DeployScreen(MatchMakeInProgress)),
            start_matchmake_in_progress_screen,
        )
        .add_systems(
            Update,
            (
                update_matchmake_in_progress_screen,
                matchmaking_started_listener,
                matchmaking_update_listener,
                match_found_listener,
                players_found_listener,
                lobby_filled_listener,
                level_loaded_listener,
                launching_listener,
            )
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
        .add_event::<PlayersFoundUpdate>()
        .add_event::<LobbyFilled>()
        .add_event::<Launching>()
        .add_event::<LevelLoaded>()
        // TODO: research how to have multiple fixed time
        // schedules and not just one
        .insert_resource(EventCounter(0))
        .insert_resource(Time::<Fixed>::from_seconds(1.0));
    }
}

fn start_matchmake_in_progress_screen(
    mut commands: Commands,
    mut matchmake_started_event: EventWriter<MatchmakingStarted>,
    mut event_counter: ResMut<EventCounter>,
) {
    debug!("starting matchmake in progress screen");
    event_counter.0 = 0;
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
            builder
                .spawn(TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 30.0,
                        ..default()
                    },
                ))
                .insert(MessageTextMarker);
        })
        .id();

    commands.insert_resource(MatchmakeInProgressMenuData {
        matchmake_messagebox_entity,
    });

    commands
        .entity(matchmake_messagebox_entity)
        .insert(Name::new("Message Box"));

    matchmake_started_event.send(MatchmakingStarted);
}

fn update_matchmake_in_progress_screen() {
    //debug!("updating matchmake in progress screen");
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

fn update_fake_matchmake_server(
    mut event_counter: ResMut<EventCounter>,
    time_fixed: Res<Time<Fixed>>,
    mut update: EventWriter<MatchmakingUpdate>, // new ping we are currently searching for
    mut match_found: EventWriter<MatchFound>,
    mut players_found: EventWriter<PlayersFoundUpdate>, // total amount of players found
    mut filled: EventWriter<LobbyFilled>,
    mut loaded: EventWriter<LevelLoaded>,
    mut launching: EventWriter<Launching>,
) {
    debug!(
        "fake matchmake server update. counter: {:?}, fixed time: {:?}",
        event_counter.0,
        time_fixed.elapsed()
    );
    if event_counter.0 <= 1 {
        update.send(MatchmakingUpdate(20));
    } else if event_counter.0 == 2 {
        update.send(MatchmakingUpdate(32));
    } else if event_counter.0 == 3 {
        update.send(MatchmakingUpdate(52));
    } else if event_counter.0 == 4 {
        match_found.send(MatchFound);
    } else if event_counter.0 == 5 {
        players_found.send(PlayersFoundUpdate(10));
    } else if event_counter.0 == 6 {
        players_found.send(PlayersFoundUpdate(25));
    } else if event_counter.0 == 7 {
        players_found.send(PlayersFoundUpdate(29));
    } else if event_counter.0 == 8 {
        players_found.send(PlayersFoundUpdate(30));
    } else if event_counter.0 == 9 {
        filled.send(LobbyFilled);
    } else if event_counter.0 == 10 {
        loaded.send(LevelLoaded);
    } else if event_counter.0 == 11 {
        launching.send(Launching(8));
    } else if event_counter.0 == 12 {
        launching.send(Launching(4));
    } else {
        launching.send(Launching(0));
    }
    event_counter.0 += 1;
}

fn matchmaking_started_listener(
    mut event: EventReader<MatchmakingStarted>,
    mut query: Query<&mut Text, (With<Text>, With<MessageTextMarker>)>,
) {
    for _ev in event.read() {
        for mut text in &mut query {
            text.sections[0].value = format!("CONNECTING");
            debug!("matchmaking started. connecting.");
        }
    }
}

fn matchmaking_update_listener(
    mut event: EventReader<MatchmakingUpdate>,
    mut query: Query<&mut Text, (With<Text>, With<MessageTextMarker>)>,
) {
    for ev in event.read() {
        let e = ev.0;
        for mut text in &mut query {
            text.sections[0].value = format!("SEARCHING FOR A MATCH <{:?}MS PING", e);
            debug!("matchmaking updated. searching for ping <{:?}ms", e);
        }
    }
}

fn match_found_listener(
    mut event: EventReader<MatchFound>,
    mut query: Query<&mut Text, (With<Text>, With<MessageTextMarker>)>,
) {
    for _ev in event.read() {
        for mut text in &mut query {
            text.sections[0].value = format!("CONNECTING");
            debug!("match found. connecting");
        }
    }
}

fn players_found_listener(
    mut event: EventReader<PlayersFoundUpdate>,
    mut query: Query<&mut Text, (With<Text>, With<MessageTextMarker>)>,
) {
    // TODO: hardcoded for now, make this dynamic later
    let max_num_players = 30;
    for ev in event.read() {
        let num = ev.0;
        let remaining_players = max_num_players - num;
        for mut text in &mut query {
            let player_string = if remaining_players == 1 {
                String::from("PLAYER")
            } else {
                String::from("PLAYERS")
            };
            text.sections[0].value = format!(
                "LOOKING FOR {:?} MORE {:}",
                remaining_players, player_string
            );
            debug!("found players. ({:?})", num);
        }
    }
}

fn lobby_filled_listener(
    mut event: EventReader<LobbyFilled>,
    mut query: Query<&mut Text, (With<Text>, With<MessageTextMarker>)>,
) {
    for _ev in event.read() {
        for mut text in &mut query {
            text.sections[0].value = format!("WAITING. LOADING LEVEL");
            debug!("lobby filled. waiting. loading level");
        }
    }
}

fn level_loaded_listener(
    mut event: EventReader<LevelLoaded>,
    mut query: Query<&mut Text, (With<Text>, With<MessageTextMarker>)>,
) {
    for _ev in event.read() {
        for mut text in &mut query {
            text.sections[0].value = format!("LAUNCHING");
            debug!("level loaded. start launching.");
        }
    }
}

fn launching_listener(
    mut next_state: ResMut<NextState<AppState>>,
    mut event: EventReader<Launching>,
    mut query: Query<&mut Text, (With<Text>, With<MessageTextMarker>)>,
) {
    for ev in event.read() {
        for mut text in &mut query {
            let countdown = ev.0;
            text.sections[0].value = format!("LAUNCHING {:?}", countdown);
            debug!("launch countdown.");
            if countdown == 0 {
                next_state.set(AppState::LoadingScreen);
            }
        }
    }
}
