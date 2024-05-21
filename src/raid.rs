// TODO: implement initial countdown on infil
// TODO: basic timelimited raid
// TODO: very basic exfil mechanic

use bevy::prelude::*;

// Events

#[derive(Event)]
struct InfilCounter(u32);
