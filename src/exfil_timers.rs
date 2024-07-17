use bevy::prelude::*;

use bevy::{
    app::{App, Plugin},
    ecs::schedule::{OnEnter, OnExit},
};

use crate::AppState::Raid;

const RAID_COUNTDOWN_MINUTES: f32 = 0.5; // 25 minutes in real dmz
const GAS_SPREADING_COUNTDOWN_MINUTES: f32 = 0.5; // 10 minutes in real dmz

#[derive(Resource)]
pub struct RaidCountdown(Timer);

impl RaidCountdown {
    pub fn new() -> Self {
        Self(Timer::from_seconds(
            RAID_COUNTDOWN_MINUTES * 60.0,
            TimerMode::Once,
        ))
    }
}

// We need to implement Default so that our timer can be initialized as
// a resource when we call `init_resource`
impl Default for RaidCountdown {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Resource)]
pub struct GasSpreadingCountdown(Timer);

impl GasSpreadingCountdown {
    pub fn new() -> Self {
        Self(Timer::from_seconds(
            GAS_SPREADING_COUNTDOWN_MINUTES * 60.0,
            TimerMode::Once,
        ))
    }
}

// We need to implement Default so that our timer can be initialized as
// a resource when we call `init_resource`
impl Default for GasSpreadingCountdown {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ExfilTimersPlugin;

impl Plugin for ExfilTimersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_exfil_timers)
            .add_systems(FixedUpdate, (update_raid_countdown).run_if(in_state(Raid)))
            .add_systems(
                FixedUpdate,
                (update_gas_spreading_countdown).run_if(in_state(Raid)),
            )
            .add_systems(OnExit(Raid), bye_exfil_timers);
    }
}

fn start_exfil_timers(mut commands: Commands) {
    debug!("start exfil timer called");
    commands.init_resource::<RaidCountdown>();
}

fn update_raid_countdown(
    time: Res<Time>,
    match_time: Option<ResMut<RaidCountdown>>,
    mut commands: Commands,
) {
    debug!("updating exfil timer called(RaidCountdown)");
    if let Some(mut raid_countdown) = match_time {
        raid_countdown.0.tick(time.delta());
        debug!("RaidCountdown: {:?}", raid_countdown.0.remaining());
        if raid_countdown.0.just_finished() {
            debug!("RaidCountdown finished: {:?}", raid_countdown.0.remaining());
            commands.init_resource::<GasSpreadingCountdown>();
            commands.remove_resource::<RaidCountdown>();
        }
    }
}

fn update_gas_spreading_countdown(
    time: Res<Time>,
    match_time: Option<ResMut<GasSpreadingCountdown>>,
) {
    debug!("updating exfil timer called(GasSpreadingCountdown)");
    if let Some(mut gas_spreading) = match_time {
        gas_spreading.0.tick(time.delta());
        debug!("GasSpreadingCountdown: {:?}", gas_spreading.0.remaining());
        if gas_spreading.0.just_finished() {
            debug!(
                "GasSpreadingCountdown finished: {:?}",
                gas_spreading.0.remaining()
            );
        }
    }
}

fn bye_exfil_timers(mut commands: Commands) {
    debug!("exit exfil timer called");
    commands.remove_resource::<RaidCountdown>();
    commands.remove_resource::<GasSpreadingCountdown>();
}
