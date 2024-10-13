use bevy::{
    app::Plugin,
    color::palettes::css::{MAROON, RED},
    window::PrimaryWindow,
};

use crate::{
    //exfil::Operator, first_person_controller::PlayerControlled, inventory::ItemSlots,
    fake_level::Crosshair,
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
                (
                    startup_cursor_crosshair,
                    start_loot_cache_ui,
                    start_backpack_ui,
                    start_loadout_ui,
                ),
            )
            .add_systems(
                OnEnter(RaidState::AccessBackpack),
                (
                    startup_cursor_crosshair,
                    start_backpack_ui,
                    start_loadout_ui,
                ),
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
                (
                    cleanup_cursor_crosshair,
                    bye_loot_cache_ui,
                    bye_backpack_ui,
                    bye_loadout_ui,
                ),
            )
            .add_systems(
                OnExit(RaidState::AccessBackpack),
                (cleanup_cursor_crosshair, bye_backpack_ui, bye_loadout_ui),
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

fn startup_cursor_crosshair(
    mut commands: Commands,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    crosshair: Query<Entity, With<Crosshair>>,
) {
    let mut primary_window = windows.single_mut();
    let crosshair_vis = crosshair.single();
    primary_window.cursor.visible = true;
    commands.entity(crosshair_vis).insert(Visibility::Hidden);
}

fn start_loot_cache_ui(mut commands: Commands) {
    debug!("start loot cache ui");

    // Layout
    // Top-level grid (app frame)
    let loot_cache_ui = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect {
                    top: Val::Percent(10.),
                    ..Default::default()
                },
                justify_self: JustifySelf::Center,
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
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(12.0)),
                        ..default()
                    },
                    //background_color: DARK_GREEN.into(),
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
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(Val::Px(12.0)),
                        border: UiRect {
                            left: Val::Px(100.0),
                            right: Val::Px(100.0),
                            ..default()
                        },
                        ..default()
                    },
                    //background_color: GREEN.into(),
                    ..default()
                })
                .insert(Name::new("Loot Cache Main"))
                .with_children(|builder| {
                    builder.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(100.),
                                height: Val::Px(50.),
                                border: UiRect::all(Val::Px(10.)),
                                margin: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            background_color: MAROON.into(),
                            border_color: RED.into(),
                            ..Default::default()
                        },
                        Outline {
                            width: Val::Px(6.),
                            offset: Val::Px(6.),
                            color: Color::WHITE,
                        },
                    ));
                    builder.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(50.),
                                height: Val::Px(50.),
                                border: UiRect::all(Val::Px(10.)),
                                margin: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            background_color: MAROON.into(),
                            border_color: RED.into(),
                            ..Default::default()
                        },
                        Outline {
                            width: Val::Px(6.),
                            offset: Val::Px(6.),
                            color: Color::WHITE,
                        },
                    ));
                    builder.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(50.),
                                height: Val::Px(50.),
                                border: UiRect::all(Val::Px(10.)),
                                margin: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            background_color: MAROON.into(),
                            border_color: RED.into(),
                            ..Default::default()
                        },
                        Outline {
                            width: Val::Px(6.),
                            offset: Val::Px(6.),
                            color: Color::WHITE,
                        },
                    ));
                    builder.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(50.),
                                height: Val::Px(50.),
                                border: UiRect::all(Val::Px(10.)),
                                margin: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            background_color: MAROON.into(),
                            border_color: RED.into(),
                            ..Default::default()
                        },
                        Outline {
                            width: Val::Px(6.),
                            offset: Val::Px(6.),
                            color: Color::WHITE,
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
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect {
                    top: Val::Percent(25.),
                    ..Default::default()
                },
                justify_self: JustifySelf::Center,
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
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(12.0)),
                        ..default()
                    },
                    //background_color: DARK_GREEN.into(),
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
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(Val::Px(12.0)),
                        border: UiRect {
                            left: Val::Px(100.0),
                            right: Val::Px(100.0),
                            ..default()
                        },
                        ..default()
                    },
                    //background_color: GREEN.into(),
                    ..default()
                })
                .insert(Name::new("Backpack Main"))
                .with_children(|builder| {
                    builder.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(50.),
                                height: Val::Px(50.),
                                border: UiRect::all(Val::Px(10.)),
                                margin: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            background_color: MAROON.into(),
                            border_color: RED.into(),
                            ..Default::default()
                        },
                        Outline {
                            width: Val::Px(6.),
                            offset: Val::Px(6.),
                            color: Color::WHITE,
                        },
                    ));
                    builder.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(50.),
                                height: Val::Px(50.),
                                border: UiRect::all(Val::Px(10.)),
                                margin: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            background_color: MAROON.into(),
                            border_color: RED.into(),
                            ..Default::default()
                        },
                        Outline {
                            width: Val::Px(6.),
                            offset: Val::Px(6.),
                            color: Color::WHITE,
                        },
                    ));
                    builder.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(50.),
                                height: Val::Px(50.),
                                border: UiRect::all(Val::Px(10.)),
                                margin: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            background_color: MAROON.into(),
                            border_color: RED.into(),
                            ..Default::default()
                        },
                        Outline {
                            width: Val::Px(6.),
                            offset: Val::Px(6.),
                            color: Color::WHITE,
                        },
                    ));
                    builder.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(50.),
                                height: Val::Px(50.),
                                border: UiRect::all(Val::Px(10.)),
                                margin: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            background_color: MAROON.into(),
                            border_color: RED.into(),
                            ..Default::default()
                        },
                        Outline {
                            width: Val::Px(6.),
                            offset: Val::Px(6.),
                            color: Color::WHITE,
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
                flex_direction: FlexDirection::Column,
                padding: UiRect {
                    top: Val::Percent(40.),
                    //left: Val::Percent(20.),
                    //right: Val::Percent(20.),
                    ..Default::default()
                },
                justify_self: JustifySelf::Center,
                ..default()
            },
            //background_color: BLUE.into(),
            ..default()
        })
        .insert(Name::new("Main Loadout Layout"))
        .with_children(|builder| {
            // Header
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(12.0)),
                        ..default()
                    },
                    //background_color: DARK_GREEN.into(),
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
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(Val::Px(12.0)),
                        border: UiRect {
                            left: Val::Px(50.0),
                            right: Val::Px(50.0),
                            ..default()
                        },
                        ..default()
                    },
                    //background_color: GREEN.into(),
                    ..default()
                })
                .insert(Name::new("Loadout Main"))
                .with_children(|builder| {
                    builder.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(100.),
                                height: Val::Px(50.),
                                border: UiRect::all(Val::Px(10.)),
                                margin: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            background_color: MAROON.into(),
                            border_color: RED.into(),
                            ..Default::default()
                        },
                        Outline {
                            width: Val::Px(6.),
                            offset: Val::Px(6.),
                            color: Color::WHITE,
                        },
                    ));
                    builder.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(50.),
                                height: Val::Px(50.),
                                border: UiRect::all(Val::Px(10.)),
                                margin: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            background_color: MAROON.into(),
                            border_color: RED.into(),
                            ..Default::default()
                        },
                        Outline {
                            width: Val::Px(6.),
                            offset: Val::Px(6.),
                            color: Color::WHITE,
                        },
                    ));
                    builder.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(50.),
                                height: Val::Px(50.),
                                border: UiRect::all(Val::Px(10.)),
                                margin: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            background_color: MAROON.into(),
                            border_color: RED.into(),
                            ..Default::default()
                        },
                        Outline {
                            width: Val::Px(6.),
                            offset: Val::Px(6.),
                            color: Color::WHITE,
                        },
                    ));
                    builder.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(50.),
                                height: Val::Px(50.),
                                border: UiRect::all(Val::Px(10.)),
                                margin: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            background_color: MAROON.into(),
                            border_color: RED.into(),
                            ..Default::default()
                        },
                        Outline {
                            width: Val::Px(6.),
                            offset: Val::Px(6.),
                            color: Color::WHITE,
                        },
                    ));
                    builder.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(50.),
                                height: Val::Px(50.),
                                border: UiRect::all(Val::Px(10.)),
                                margin: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            background_color: MAROON.into(),
                            border_color: RED.into(),
                            ..Default::default()
                        },
                        Outline {
                            width: Val::Px(6.),
                            offset: Val::Px(6.),
                            color: Color::WHITE,
                        },
                    ));
                    builder.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(50.),
                                height: Val::Px(50.),
                                border: UiRect::all(Val::Px(10.)),
                                margin: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            background_color: MAROON.into(),
                            border_color: RED.into(),
                            ..Default::default()
                        },
                        Outline {
                            width: Val::Px(6.),
                            offset: Val::Px(6.),
                            color: Color::WHITE,
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

fn cleanup_cursor_crosshair(
    mut commands: Commands,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    crosshair: Query<Entity, With<Crosshair>>,
) {
    let mut primary_window = windows.single_mut();
    let crosshair_vis = crosshair.single();
    primary_window.cursor.visible = false;
    commands.entity(crosshair_vis).insert(Visibility::Visible);
}

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
