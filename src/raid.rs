// TODO: idea: put all related resources/events/systems into their own plugins like Infil, Exfil,
// Raid and declare a Raid PluginGroup that is added to the main.rs
// TODO: basic timelimited raid

use bevy::{math::bounding::Aabb3d, prelude::*};

use crate::{
    exfil::Operator,
    AppState::{self, Raid},
};

// Components

#[derive(Component)]
pub struct FirstPersonCamera;

#[derive(Component, Debug)]
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
    commands.insert_resource(InfilCountdown(31));
    commands.insert_resource(LiftoffCountdown(34));
    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(Name::new("FirstPersonCamera"))
        .insert(Volume(Aabb3d {
            min: Vec3 {
                x: -0.5,
                y: 0.0,
                z: -0.5,
            },
            max: Vec3 {
                x: 0.5,
                y: 1.0,
                z: 0.5,
            },
        }))
        .insert(Operator)
        .insert(FirstPersonCamera);
}

fn update_raid(mut _next_state: ResMut<NextState<AppState>>) {
    debug!("updating raid called");
}

fn bye_raid(mut commands: Commands, query: Query<Entity, With<FirstPersonCamera>>) {
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
