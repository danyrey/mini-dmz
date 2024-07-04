// TODO: setup timers and timer chains up for state Raid

use bevy::prelude::*;

use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::{OnEnter, OnExit},
};

use crate::AppState::Raid;

pub struct ExfilTimersPlugin;

impl Plugin for ExfilTimersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_exfil_timers)
            .add_systems(Update, (update_exfil_timers).run_if(in_state(Raid)))
            .add_systems(OnExit(Raid), bye_exfil_timers);
    }
}

fn start_exfil_timers() {
    debug!("start exfil timer called");
}

fn update_exfil_timers() {
    debug!("updating exfil timer called");
}

fn bye_exfil_timers() {
    debug!("exit exfil timer called");
}
