// Raid and declare a Raid PluginGroup that is added to the main.rs

use bevy::{
    math::bounding::Aabb3d,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::{
    choose_location::ChosenLocation,
    AppState::{self, Raid},
};

// Sub States
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::Raid)]
pub enum RaidState {
    #[default]
    Raid,
    AccessLootCache,
    AccessBackpack,
}

// Components

#[derive(Component)]
pub struct FreeLookCamera;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Debug)]
#[allow(dead_code)]
pub struct Volume(pub Aabb3d);

// Events

#[derive(Event, Debug)]
struct InfilCounter(u32);

#[derive(Event)]
struct InfilComplete;

// Resources

// TODO: add simple ui "exfil button" for triggering the exfil procedure

#[derive(Default, Resource)]
struct InfilCountdown(u32);

#[derive(Default, Resource)]
#[allow(dead_code)]
struct LiftoffCountdown(u32);

// TODO: counters for all other phases of exfil

// Plugin

pub struct RaidPlugin;

impl Plugin for RaidPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InfilCounter>()
            .add_event::<InfilComplete>()
            .add_sub_state::<RaidState>()
            .add_systems(OnEnter(Raid), start_raid)
            .add_systems(
                Update,
                (
                    update_raid,
                    infil_countdown_listener,
                    infil_countdown_complete_listener,
                )
                    .run_if(in_state(Raid)),
            )
            .add_systems(FixedUpdate, (infil_countdown).run_if(in_state(Raid)))
            .add_systems(OnExit(Raid), bye_raid);
    }
}

// Systems

fn start_raid(mut commands: Commands, mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();

    // if you want to use the cursor, but not let it leave the window,
    // use `Confined` mode:
    primary_window.cursor.grab_mode = CursorGrabMode::Confined;

    // for a game that doesn't use the cursor (like a shooter):
    // use `Locked` mode to keep the cursor in one place
    //primary_window.cursor.grab_mode = CursorGrabMode::Locked;

    // also hide the cursor
    //primary_window.cursor.visible = false;

    debug!("starting raid called");
    commands.insert_resource(InfilCountdown(31));
    commands.insert_resource(LiftoffCountdown(34));
    // chosen location cleanup
    commands.remove_resource::<ChosenLocation>();
}

fn update_raid(mut _next_state: ResMut<NextState<AppState>>) {
    debug!("updating raid called");
}

fn bye_raid(
    mut commands: Commands,
    query: Query<Entity, With<FreeLookCamera>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_windows.single_mut();

    primary_window.cursor.grab_mode = CursorGrabMode::None;
    primary_window.cursor.visible = true;

    debug!("exiting raid called");
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn infil_countdown(
    mut infil_countdown: ResMut<InfilCountdown>,
    mut event: EventWriter<InfilCounter>, // new ping we are currently searching for
) {
    debug!("infil countdown system called");
    if infil_countdown.0 > 0 {
        infil_countdown.0 -= 1;
        event.send(InfilCounter(infil_countdown.0));
        debug!("infil countdown event send : {:?}", infil_countdown.0);
    }
}
fn infil_countdown_listener(
    mut events: EventReader<InfilCounter>,
    mut complete: EventWriter<InfilComplete>,
) {
    for event in events.read() {
        // TODO: print countdown on screen, output just to log for now
        debug!("counter received: {:?}", event);
        if event.0 == 0 {
            complete.send(InfilComplete);
            debug!("sending complete event");
        }
    }
}

fn infil_countdown_complete_listener(mut events: EventReader<InfilComplete>) {
    for _ in events.read() {
        debug!("'activate' controls so player can move now. match is in progress now")
    }
}
