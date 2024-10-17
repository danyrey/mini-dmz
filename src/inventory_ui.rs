use bevy::{
    app::Plugin,
    color::palettes::css::{DARK_GREY, GREY, MAROON, RED},
    window::PrimaryWindow,
};

use crate::{
    fake_level::Crosshair,
    interaction::InventoryInteracted,
    inventory::{Inventory, ItemSlot, ItemSlots, WeaponSlot, WeaponSlots},
    loot::{Loot, LootName},
    raid::RaidState,
    AppState,
};
use bevy::prelude::*;

// Constants
const NORMAL_BUTTON: Color = Color::srgb(MAROON.red, MAROON.green, MAROON.blue);
const HOVERED_BUTTON: Color = Color::srgb(RED.red, RED.green, RED.blue);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

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
                (
                    start_loot_cache_interaction,
                    toggle_loot_cache_ui,
                    toggle_backpack_ui,
                )
                    .run_if(in_state(AppState::Raid)),
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
#[derive(Component)]
pub struct LootCacheItem;

#[derive(Component)]
pub struct LootCacheWeapon;

#[derive(Component)]
pub struct BackpackItem;

#[derive(Component)]
pub struct BackpackWeapon;

// Resources
#[derive(Resource)]
struct LootCacheUI {
    loot_cache_ui: Entity,
}

#[derive(Resource)]
struct LootCacheEntities {
    loot_cache: Entity,
    backpack: Entity,
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

fn start_loot_cache_interaction(
    mut loot_cache_interaction: EventReader<InventoryInteracted>,
    mut commands: Commands,
    raid_state: Res<State<RaidState>>,
    mut next_raid_state: ResMut<NextState<RaidState>>,
) {
    for interacted in loot_cache_interaction.read() {
        debug!("i received the event, TODO: time to popluate the ui");
        debug!("loot cache: {:?}", interacted.interaction_inventory);
        debug!(
            "operator inventory(backpack): {:?}",
            interacted.operator_inventory
        );

        commands.insert_resource(LootCacheEntities {
            loot_cache: interacted.interaction_inventory,
            backpack: interacted.operator_inventory,
        });

        if raid_state.get() == &RaidState::Raid {
            next_raid_state.set(RaidState::AccessLootCache)
        }
    }
}

// TODO: query/fetch items for populating the ui
fn start_loot_cache_ui(
    mut commands: Commands,
    loot_entities: Res<LootCacheEntities>,
    inventories_with_items: Query<(&ItemSlots, &Name), With<Inventory>>,
    inventory_items: Query<(&Parent, &ItemSlot, Option<&LootName>), With<Loot>>,
    inventories_with_weapons: Query<&WeaponSlots, With<Inventory>>,
    inventory_weapons: Query<(&Parent, &WeaponSlot, Option<&LootName>), With<Loot>>,
) {
    debug!("start loot cache ui");

    let loot_cache = loot_entities.loot_cache;

    // Loot Cache
    let mut loot_cache_items: Vec<(&ItemSlot, Option<&LootName>)> = inventory_items
        .iter()
        .filter(|ii| loot_cache == ii.0.get())
        .map(|ii| (ii.1, ii.2))
        .collect();
    loot_cache_items.sort_by(|a, b| (a.0).0.cmp(&(b.0).0));

    let loot_cache_item_slots: usize = inventories_with_items
        .get(loot_cache)
        .map_or(0, |r| (r.0).0.into());

    let loot_cache_name: String = inventories_with_items
        .get(loot_cache)
        .map_or("".to_string(), |r| r.1.into());

    let mut loot_cache_weapons: Vec<(&WeaponSlot, Option<&LootName>)> = inventory_weapons
        .iter()
        .filter(|ii| loot_cache == ii.0.get())
        .map(|ii| (ii.1, ii.2))
        .collect();
    loot_cache_weapons.sort_by(|a, b| (a.0).0.cmp(&(b.0).0));

    let loot_cache_weapon_slots: usize = inventories_with_weapons
        .get(loot_cache)
        .map_or(0, |r| r.0.into());

    // Loadout
    // TODO: loadout

    debug!("item slots: {:?}", loot_cache_item_slots);
    debug!("weapon slots: {:?}", loot_cache_weapon_slots);
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
                        //"Loot Cache Header",
                        loot_cache_name,
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
                    let mut it_slot = loot_cache_weapons.iter();
                    let mut slot = it_slot.next();

                    for weapon_slot_no in 0..loot_cache_weapon_slots {
                        debug!("weapon slot: {:?}", weapon_slot_no);
                        if let Some(s) = slot {
                            if ((s.0).0 as usize).eq(&weapon_slot_no) {
                                debug!("slot: {:?}", (s.0).0);
                                create_weapon_slot_ui(builder, s.1, true, false);
                                slot = it_slot.next();
                            } else {
                                debug!("slot: nothing");
                                create_empty_weapon_slot_ui(builder);
                            }
                        } else {
                            debug!("slot: nothing");
                            create_empty_weapon_slot_ui(builder);
                        }
                    }

                    let mut it_slot = loot_cache_items.iter();
                    let mut slot = it_slot.next();

                    for item_slot_no in 0..loot_cache_item_slots {
                        debug!("item slot: {:?}", item_slot_no);
                        if let Some(s) = slot {
                            if ((s.0).0 as usize).eq(&item_slot_no) {
                                debug!("slot: {:?}", (s.0).0);
                                create_item_slot_ui(builder, s.1, true, false);
                                slot = it_slot.next();
                            } else {
                                debug!("slot: nothing");
                                create_empty_item_slot_ui(builder);
                            }
                        } else {
                            debug!("slot: nothing");
                            create_empty_item_slot_ui(builder);
                        }
                    }
                });
        })
        .id();

    // insert resource
    commands.insert_resource(LootCacheUI { loot_cache_ui });
}

fn start_backpack_ui(
    mut commands: Commands,
    loot_entities: Res<LootCacheEntities>,
    inventories_with_items: Query<(&ItemSlots, &Name), With<Inventory>>,
    inventory_items: Query<(&Parent, &ItemSlot, Option<&LootName>), With<Loot>>,
    inventories_with_weapons: Query<&WeaponSlots, With<Inventory>>,
    inventory_weapons: Query<(&Parent, &WeaponSlot, Option<&LootName>), With<Loot>>,
) {
    debug!("start backpack ui");
    let backpack = loot_entities.backpack;

    // Backpack
    let mut backpack_items: Vec<(&ItemSlot, Option<&LootName>)> = inventory_items
        .iter()
        .filter(|ii| backpack == ii.0.get())
        .map(|ii| (ii.1, ii.2))
        .collect();
    backpack_items.sort_by(|a, b| (a.0).0.cmp(&(b.0).0));

    let backpack_item_slots: usize = inventories_with_items
        .get(backpack)
        .map_or(0, |r| (r.0).0.into());

    let backpack_name: String = inventories_with_items
        .get(backpack)
        .map_or("".to_string(), |r| r.1.into());

    let mut backpack_weapons: Vec<(&WeaponSlot, Option<&LootName>)> = inventory_weapons
        .iter()
        .filter(|ii| backpack == ii.0.get())
        .map(|ii| (ii.1, ii.2))
        .collect();
    backpack_weapons.sort_by(|a, b| (a.0).0.cmp(&(b.0).0));

    let backpack_weapon_slots: usize = inventories_with_weapons
        .get(backpack)
        .map_or(0, |r| r.0.into());

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
                        backpack_name,
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
                    let mut it_slot = backpack_weapons.iter();
                    let mut slot = it_slot.next();

                    for weapon_slot_no in 0..backpack_weapon_slots {
                        debug!("weapon slot: {:?}", weapon_slot_no);
                        if let Some(s) = slot {
                            if ((s.0).0 as usize).eq(&weapon_slot_no) {
                                debug!("slot: {:?}", (s.0).0);
                                create_weapon_slot_ui(builder, s.1, false, true);
                                slot = it_slot.next();
                            } else {
                                debug!("slot: nothing");
                                create_empty_weapon_slot_ui(builder);
                            }
                        } else {
                            debug!("slot: nothing");
                            create_empty_weapon_slot_ui(builder);
                        }
                    }

                    let mut it_slot = backpack_items.iter();
                    let mut slot = it_slot.next();

                    for item_slot_no in 0..backpack_item_slots {
                        debug!("item slot: {:?}", item_slot_no);
                        if let Some(s) = slot {
                            if ((s.0).0 as usize).eq(&item_slot_no) {
                                debug!("slot: {:?}", (s.0).0);
                                create_item_slot_ui(builder, s.1, false, true);
                                slot = it_slot.next();
                            } else {
                                debug!("slot: nothing");
                                create_empty_item_slot_ui(builder);
                            }
                        } else {
                            debug!("slot: nothing");
                            create_empty_item_slot_ui(builder);
                        }
                    }
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

fn update_loot_cache_ui(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            Or<(With<LootCacheItem>, With<LootCacheWeapon>)>,
        ),
    >,
) {
    debug!("updating loot cache ui");
    // TODO: triggered for backpack too, maybe adjust query
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                debug!("button pressed");
                *color = PRESSED_BUTTON.into();
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
    commands.remove_resource::<LootCacheEntities>();
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
fn create_empty_weapon_slot_ui(builder: &mut ChildBuilder) {
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
            background_color: DARK_GREY.into(),
            border_color: GREY.into(),
            ..Default::default()
        },
        Outline {
            width: Val::Px(6.),
            offset: Val::Px(6.),
            color: Color::WHITE,
        },
    ));
}

fn create_weapon_slot_ui(
    builder: &mut ChildBuilder,
    name: Option<&LootName>,
    loot_cache: bool,
    backpack: bool,
) {
    // TODO: there must be a better way, this fugly
    let label: String = name.map(|x| x.0.clone()).unwrap_or("".to_string());
    let mut weapon = builder.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(100.),
                height: Val::Px(50.),
                border: UiRect::all(Val::Px(10.)),
                margin: UiRect::all(Val::Px(20.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            border_color: RED.into(),
            background_color: NORMAL_BUTTON.into(),
            ..default()
        },
        Outline {
            width: Val::Px(6.),
            offset: Val::Px(6.),
            color: Color::WHITE,
        },
    ));

    weapon.with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            label,
            TextStyle {
                font_size: 8.0,
                color: Color::srgb(0.9, 0.9, 0.9),
                ..default()
            },
        ));
    });

    if loot_cache {
        weapon.insert(LootCacheWeapon);
    }

    if backpack {
        weapon.insert(BackpackWeapon);
    }
}

fn create_empty_item_slot_ui(builder: &mut ChildBuilder) {
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
            background_color: DARK_GREY.into(),
            border_color: GREY.into(),
            ..Default::default()
        },
        Outline {
            width: Val::Px(6.),
            offset: Val::Px(6.),
            color: Color::WHITE,
        },
    ));
}

fn create_item_slot_ui(
    builder: &mut ChildBuilder,
    name: Option<&LootName>,
    loot_cache: bool,
    backpack: bool,
) {
    // TODO: there must be a better way, this fugly
    let label: String = name.map(|x| x.0.clone()).unwrap_or("".to_string());
    let mut item = builder.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(50.),
                height: Val::Px(50.),
                border: UiRect::all(Val::Px(10.)),
                margin: UiRect::all(Val::Px(20.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            border_color: RED.into(),
            background_color: NORMAL_BUTTON.into(),
            ..default()
        },
        Outline {
            width: Val::Px(6.),
            offset: Val::Px(6.),
            color: Color::WHITE,
        },
    ));

    item.with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            label,
            TextStyle {
                font_size: 8.0,
                color: Color::srgb(0.9, 0.9, 0.9),
                ..default()
            },
        ));
    });

    if loot_cache {
        item.insert(LootCacheItem);
    }

    if backpack {
        item.insert(BackpackItem);
    }
}

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
