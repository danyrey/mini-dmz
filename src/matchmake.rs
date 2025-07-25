// TODO: 'matchmake' will start the matchmake. all operators in the party need to be on ready
// state. the party leader and matchmaker is ready by default and i think he can forcibly matchmake
// if not all members are ready, not sure.
// TODO: more research for corner cases. make screenshots for when not the whole team is not ready.
// TODO: messages during matchmaking need to be displayed until we can advance to loading screen

use std::time::Duration;

use crate::AppState::DeployScreen;
use crate::DeployScreen::*;
use crate::{AppState, ButtonTargetState};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

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
    location_layout: Entity,
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn start_matchmake_screen(mut commands: Commands) {
    debug!("starting matchmake screen");

    // Layout
    // Top-level grid (app frame)
    let location_layout = commands
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
                    spawn_nested_text_bundle(builder, 40.0, "TEAM READY");
                    spawn_nested_text_bundle(builder, 10.0, "");
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
                    let confirm_name = Name::new("MATCHMAKE");
                    button_bundle(
                        builder,
                        confirm_name.clone(),
                        confirm_name.as_str(),
                        ButtonTargetState(DeployScreen(MatchMakeInProgress)),
                    );
                    let edit_name = Name::new("BACK");
                    button_bundle(
                        builder,
                        edit_name.clone(),
                        edit_name.as_str(),
                        ButtonTargetState(DeployScreen(ActiveDutyConfirmation)),
                    );
                });
        })
        .id();

    // insert resource
    commands.insert_resource(MatchmakeMenuData { location_layout });
}

#[allow(clippy::type_complexity)]
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
        .entity(menu_data.location_layout)
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
            (update_fake_matchmake_server)
                .run_if(in_state(DeployScreen(MatchMakeInProgress)))
                .run_if(on_timer(Duration::from_secs(1))),
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
        .insert_resource(EventCounter(0));
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
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Start,
            flex_grow: 1.,
            margin: UiRect::axes(Val::Px(15.), Val::Px(5.)),
            ..default()
        })
        .with_children(|builder| {
            builder
                .spawn(Text::new(""))
                .insert(TextFont {
                    font_size: 30.0,
                    ..default()
                })
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

// TODO: refactor this. ignored by clippy for now
#[allow(clippy::too_many_arguments)]
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

#[allow(clippy::type_complexity)]
fn matchmaking_started_listener(
    mut event: EventReader<MatchmakingStarted>,
    query: Query<Entity, (With<Text>, With<MessageTextMarker>)>,
    mut writer: TextUiWriter,
) {
    for _ev in event.read() {
        for entity in query.iter() {
            *writer.text(entity, 0) = "CONNECTING".to_string();
            debug!("matchmaking started. connecting.");
        }
    }
}

#[allow(clippy::type_complexity)]
fn matchmaking_update_listener(
    mut event: EventReader<MatchmakingUpdate>,
    query: Query<Entity, (With<Text>, With<MessageTextMarker>)>,
    mut writer: TextUiWriter,
) {
    for ev in event.read() {
        let e = ev.0;
        for entity in query.iter() {
            *writer.text(entity, 0) = format!("SEARCHING FOR A MATCH <{e:?}MS PING");
            debug!("matchmaking updated. searching for ping <{:?}ms", e);
        }
    }
}

#[allow(clippy::type_complexity)]
fn match_found_listener(
    mut event: EventReader<MatchFound>,
    query: Query<Entity, (With<Text>, With<MessageTextMarker>)>,
    mut writer: TextUiWriter,
) {
    for _ev in event.read() {
        for entity in query.iter() {
            *writer.text(entity, 0) = "CONNECTING".to_string();
            debug!("match found. connecting");
        }
    }
}

fn players_found_listener(
    mut event: EventReader<PlayersFoundUpdate>,
    query: Query<Entity, (With<Text>, With<MessageTextMarker>)>,
    mut writer: TextUiWriter,
) {
    // TODO: hardcoded for now, make this dynamic later
    let max_num_players = 30;
    for ev in event.read() {
        let num = ev.0;
        let remaining_players = max_num_players - num;
        for entity in query.iter() {
            let player_string = if remaining_players == 1 {
                String::from("PLAYER")
            } else {
                String::from("PLAYERS")
            };
            *writer.text(entity, 0) =
                format!("LOOKING FOR {remaining_players:?} MORE {player_string}");
            debug!("found players. ({:?})", num);
        }
    }
}

fn lobby_filled_listener(
    mut event: EventReader<LobbyFilled>,
    query: Query<Entity, (With<Text>, With<MessageTextMarker>)>,
    mut writer: TextUiWriter,
) {
    for _ev in event.read() {
        for entity in query.iter() {
            *writer.text(entity, 0) = "WAITING. LOADING LEVEL".to_string();
            debug!("lobby filled. waiting. loading level");
        }
    }
}

fn level_loaded_listener(
    mut event: EventReader<LevelLoaded>,
    query: Query<Entity, (With<Text>, With<MessageTextMarker>)>,
    mut writer: TextUiWriter,
) {
    for _ev in event.read() {
        for entity in query.iter() {
            *writer.text(entity, 0) = "LAUNCHING".to_string();
            debug!("level loaded. start launching.");
        }
    }
}

fn launching_listener(
    mut next_state: ResMut<NextState<AppState>>,
    mut event: EventReader<Launching>,
    query: Query<Entity, (With<Text>, With<MessageTextMarker>)>,
    mut writer: TextUiWriter,
) {
    for ev in event.read() {
        for entity in query.iter() {
            let countdown = ev.0;
            *writer.text(entity, 0) = format!("LAUNCHING {countdown:?}");
            debug!("launch countdown.");
            if countdown == 0 {
                next_state.set(AppState::LoadingScreen);
            }
        }
    }
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

fn button_bundle(
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
                    // TODO: redo
                    //background_color: NORMAL_BUTTON.into(),
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
