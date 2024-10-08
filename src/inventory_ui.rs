use bevy::app::Plugin;

use crate::{
    //exfil::Operator, first_person_controller::PlayerControlled, inventory::ItemSlots,
    raid::RaidState,
    AppState,
};
use bevy::prelude::*;

// Constants

// Plugin

/// Plugin to manage ui for loot cache, backpack and loadout in game
///
/// accessing loot cache shows loot cache, backpack and loadout ui
/// accessing backpack alone shows backpack and loadout ui
///
/// all UIs can only be shown during a raid
pub struct InventoryUIPlugin;

impl Plugin for InventoryUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AccessLootCache>()
            .add_systems(
                Update,
                (toggle_loot_cache_ui, toggle_backpack_ui).run_if(in_state(AppState::Raid)),
            )
            .add_systems(
                OnEnter(RaidState::AccessLootCache),
                (start_loot_cache_ui, start_backpack_ui, start_loadout_ui),
            )
            .add_systems(
                OnEnter(RaidState::AccessBackpack),
                (start_backpack_ui, start_loadout_ui),
            )
            .add_systems(
                Update,
                (update_loot_cache_ui, update_backpack_ui, update_loadout_ui)
                    .run_if(in_state(RaidState::AccessLootCache)),
            )
            .add_systems(
                Update,
                (update_backpack_ui, update_loadout_ui).run_if(in_state(RaidState::AccessBackpack)),
            )
            .add_systems(
                OnExit(RaidState::AccessLootCache),
                (bye_loot_cache_ui, bye_backpack_ui, bye_loadout_ui),
            )
            .add_systems(
                OnExit(RaidState::AccessBackpack),
                (bye_backpack_ui, bye_loadout_ui),
            );
    }
}

// Components

// Resources
#[derive(Resource)]
struct LootCacheUI {
    loot_cache_ui: Entity,
}

#[allow(dead_code)]
#[derive(Resource)]
struct BackpackUI {
    backpack_ui: Entity,
}

#[allow(dead_code)]
#[derive(Resource)]
struct LoadoutUI {
    loadout_ui: Entity,
}

// Events

#[derive(Event, Debug, PartialEq)]
struct AccessLootCache {
    pub operator: Entity,
    pub inventory: Entity,
}

#[allow(dead_code)]
#[derive(Event, Debug, PartialEq)]
struct AccessBackpack {
    pub operator: Entity,
}

// Systems

// TODO: add system similar to stowing, but only for inventories instead of loot and transition to
// RaidStates::Inventory for a given Inventory
// let this plugin decide when the state transition happens to decouple it from inventory plugin

// TOGGLE SYSTEMS

// TODO: replace keypress with actual logic (proximity, raycast, occlusion, ...)
/// toggles sub state for AccessLootCache
fn toggle_loot_cache_ui(
    //mut _commands: Commands,
    //mut _raid_state: ResMut<NextState<RaidState>>,
    //_operator_query: Query<&Transform, (With<Operator>, With<PlayerControlled>)>,
    //_inventory_query: Query<Entity, With<ItemSlots>>,
    //mut _access_inventory: EventWriter<AccessLootCache>,
    key_input: Res<ButtonInput<KeyCode>>,
    raid_state: Res<State<RaidState>>,
    mut next_raid_state: ResMut<NextState<RaidState>>,
) {
    debug!("toggle ui for accessing loot cache");
    if key_input.just_pressed(KeyCode::F5) {
        match raid_state.get() {
            RaidState::Raid => next_raid_state.set(RaidState::AccessLootCache),
            RaidState::AccessLootCache => next_raid_state.set(RaidState::Raid),
            _ => (),
        }
    }
}

// TODO: replace keypress with actual logic (proximity, raycast, occlusion, ...)
/// toggles sub state for AccessBackpack
fn toggle_backpack_ui(
    key_input: Res<ButtonInput<KeyCode>>,
    raid_state: Res<State<RaidState>>,
    mut next_raid_state: ResMut<NextState<RaidState>>,
) {
    debug!("toggle ui for accessing backpack");
    if key_input.just_pressed(KeyCode::F6) {
        match raid_state.get() {
            RaidState::Raid => next_raid_state.set(RaidState::AccessBackpack),
            RaidState::AccessBackpack => next_raid_state.set(RaidState::Raid),
            _ => (),
        }
    }
}

// STARTING SYSTEMS

fn start_loot_cache_ui(mut commands: Commands) {
    debug!("start loot cache ui");

    // Layout
    // Top-level grid (app frame)
    let loot_cache_ui = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect {
                    top: Val::Percent(10.),
                    ..Default::default()
                },
                grid_template_columns: vec![GridTrack::auto()],
                grid_template_rows: vec![
                    GridTrack::auto(),
                    GridTrack::flex(1.0),
                    GridTrack::px(20.),
                ],
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Main Loot Cache Layout"))
        .with_children(|builder| {
            // Header
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(12.0)),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new("Loot Cache Header"))
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "Loot Cache Header",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
            // Main
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(12.0)),
                        grid_template_columns: RepeatedGridTrack::flex(1, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new("Loot Cache Main"))
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "Loot Cache Main",
                        TextStyle {
                            font_size: 10.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        })
        .id();

    // insert resource
    commands.insert_resource(LootCacheUI { loot_cache_ui });
}

fn start_backpack_ui(mut commands: Commands) {
    debug!("start backpack ui");

    // Layout
    // Top-level grid (app frame)
    let backpack_ui = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect {
                    top: Val::Percent(20.),
                    ..Default::default()
                },
                grid_template_columns: vec![GridTrack::auto()],
                grid_template_rows: vec![
                    GridTrack::auto(),
                    GridTrack::flex(1.0),
                    GridTrack::px(20.),
                ],
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Main Backpack Layout"))
        .with_children(|builder| {
            // Header
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(12.0)),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new("Backpack Header"))
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "Backpack Header",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
            // Main
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(12.0)),
                        grid_template_columns: RepeatedGridTrack::flex(1, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new("Backpack Main"))
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "Backpack Main",
                        TextStyle {
                            font_size: 10.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        })
        .id();

    // insert resource
    commands.insert_resource(BackpackUI { backpack_ui });
}

fn start_loadout_ui(mut commands: Commands) {
    debug!("start loadout ui");

    // Layout
    // Top-level grid (app frame)
    let loadout_ui = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect {
                    top: Val::Percent(30.),
                    ..Default::default()
                },
                grid_template_columns: vec![GridTrack::auto()],
                grid_template_rows: vec![
                    GridTrack::auto(),
                    GridTrack::flex(1.0),
                    GridTrack::px(20.),
                ],
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Main Loadout Layout"))
        .with_children(|builder| {
            // Header
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(12.0)),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new("Loadout Header"))
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "Loadout Header",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
            // Main
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(12.0)),
                        grid_template_columns: RepeatedGridTrack::flex(1, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new("Loadout Main"))
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "Loadout Main",
                        TextStyle {
                            font_size: 10.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        })
        .id();

    // insert resource
    commands.insert_resource(LoadoutUI { loadout_ui });
}

// UPDATE SYSTEMS

fn update_loot_cache_ui() {
    debug!("updating loot cache ui");
}

fn update_backpack_ui() {
    debug!("updating backpack ui");
}

fn update_loadout_ui() {
    debug!("updating loadout ui");
}

// SHUTDOWN SYSTEMS

fn bye_loot_cache_ui(mut commands: Commands, loot_cache_ui: Res<LootCacheUI>) {
    debug!("cleanup loot cache ui");
    commands
        .entity(loot_cache_ui.loot_cache_ui)
        .despawn_recursive();
}

fn bye_backpack_ui(mut commands: Commands, backpack_ui: Res<BackpackUI>) {
    debug!("cleanup backpack ui");
    commands.entity(backpack_ui.backpack_ui).despawn_recursive();
}

fn bye_loadout_ui(mut commands: Commands, loadout_ui: Res<LoadoutUI>) {
    debug!("cleanup loadout ui");
    commands.entity(loadout_ui.loadout_ui).despawn_recursive();
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    #[test]
    fn should_test_something() {
        // given
        //let mut _app = App::new();

        // when
        //app.add_event::<HealthDamageReceived>();
        //app.add_systems(Update, damage_received_listener);
        //let entity = app.borrow_mut().world.spawn(Health(100)).id();
        //app.borrow_mut().world.resource_mut::<Events<HealthDamageReceived>>().send(HealthDamageReceived { entity, damage: 10 });
        //app.update();

        // then
        //assert!(app.world.get::<Health>(entity).is_some());
        //assert_eq!(app.world.get::<Health>(entity).unwrap().0, 90);
    }
}
