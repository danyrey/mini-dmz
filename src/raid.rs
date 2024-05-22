// TODO: basic timelimited raid
// TODO: very basic exfil mechanic
// just use a button for calling exfil for now

use bevy::prelude::*;

use crate::AppState::{self, Raid};

// Events

#[derive(Event, Debug)]
struct InfilCounter(u32);

#[derive(Event)]
struct InfilComplete;

// TODO: Potential events for Exfil procedure
// ExfilCalled // trigger the flare and sound fx
// ExfilEnteredAO // trigger spawning of helicopter
// ExfilSpawned // trigger radio in of pilot
// ExfilApproached
// ExfilDescented
// ExfilLandingHovered
// ExfilTouchedDown
// ExfilFullyBoarded
// ExfilTakeOffHovered
// ExfilClimbed
// ExfilCruised
// ExfilExfilled

// Resources

#[derive(Default, Resource)]
struct InfilCountdown {
    counter: u32,
}

#[derive(Default, Resource)]
struct LiftoffCountdown(u32);

// TODO: counters for all other phases of exfil

// Plugin

pub struct RaidPlugin;

impl Plugin for RaidPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InfilCounter>()
            .add_event::<InfilComplete>()
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

fn start_raid(mut commands: Commands) {
    debug!("starting raid called");
    commands.insert_resource(InfilCountdown { counter: 31 });
    commands.insert_resource(LiftoffCountdown(34));
}

fn update_raid(mut _next_state: ResMut<NextState<AppState>>) {
    debug!("updating raid called");
}

fn bye_raid(mut _commands: Commands) {
    debug!("exiting raid called");
}

fn infil_countdown(
    mut infil_countdown: ResMut<InfilCountdown>,
    mut event: EventWriter<InfilCounter>, // new ping we are currently searching for
) {
    debug!("infil countdown system called");
    if infil_countdown.counter > 0 {
        infil_countdown.counter -= 1;
        event.send(InfilCounter(infil_countdown.counter));
        debug!("infil countdown event send : {:?}", infil_countdown.counter);
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
        todo!("'activate' controls so player can move now. match is in progress now")
    }
}
