// TODO: implement initial countdown on infil
// TODO: basic timelimited raid
// TODO: very basic exfil mechanic

use bevy::prelude::*;

use crate::AppState;

// Events

#[derive(Event)]
struct InfilCounter(u32);

pub struct RaidPlugin;

impl Plugin for RaidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Match(crate::Raid::Infil)), start_raid)
            .add_systems(
                Update,
                (update_raid).run_if(in_state(AppState::Match(crate::Raid::Infil))),
            )
            .add_systems(OnExit(AppState::Match(crate::Raid::Infil)), bye_raid);
    }
}

fn start_raid(mut commands: Commands) {
    todo!("raid setup system")
}

fn update_raid(mut next_state: ResMut<NextState<AppState>>) {
    todo!("raid update system")
}

fn bye_raid(mut commands: Commands) {
    todo!("raid exit system")
}
