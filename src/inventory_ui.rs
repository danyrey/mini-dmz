use bevy::{
    app::Plugin,
    color::palettes::css::{DARK_GREY, GREY, MAROON, RED},
    window::PrimaryWindow,
};
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

use crate::{
    fake_level::Crosshair,
    interaction::InventoryInteracted,
    inventory::{Inventory, ItemSlot, ItemSlots, StowLoot, StowedLoot, WeaponSlot, WeaponSlots},
    loot::{Loot, LootName, LootType, Price, Rarity},
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
        app
            // trying to setup systems so they dont run into any panic
            // if resources are not available
            .register_type::<EntityReference>()
            .add_event::<AccessLootCache>()
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
                )
                    .chain()
                    .run_if(in_state(AppState::Raid))
                    .run_if(resource_exists::<LootCacheEntities>),
            )
            .add_systems(
                OnEnter(RaidState::AccessBackpack),
                (
                    startup_cursor_crosshair,
                    start_backpack_ui,
                    start_loadout_ui,
                )
                    .run_if(in_state(AppState::Raid)),
            )
            .add_systems(
                Update,
                (update_loot_cache_ui, update_backpack_ui, update_loadout_ui)
                    .chain()
                    .run_if(in_state(AppState::Raid))
                    .run_if(in_state(RaidState::AccessLootCache))
                    .run_if(resource_exists::<LootCacheEntities>)
                    .run_if(resource_exists::<LoadoutUI>),
            )
            .add_systems(
                Update,
                (update_stowed_loot_cache_ui, update_stowed_loot_backpack_ui)
                    .chain()
                    .run_if(in_state(AppState::Raid))
                    .run_if(in_state(RaidState::AccessLootCache))
                    .run_if(resource_exists::<LootCacheEntities>)
                    .run_if(on_event::<StowedLoot>()),
            )
            .add_systems(
                Update,
                (update_backpack_ui, update_loadout_ui)
                    .chain()
                    .run_if(in_state(AppState::Raid))
                    .run_if(in_state(RaidState::AccessBackpack))
                    .run_if(resource_exists::<LoadoutUI>),
            )
            .add_systems(
                OnExit(RaidState::AccessLootCache),
                (
                    cleanup_cursor_crosshair,
                    bye_loot_cache_ui,
                    bye_backpack_ui,
                    bye_loadout_ui,
                )
                    .chain()
                    .run_if(in_state(AppState::Raid))
                    .run_if(resource_exists::<LootCacheEntities>)
                    .run_if(resource_exists::<LoadoutUI>),
            )
            .add_systems(
                OnExit(RaidState::AccessBackpack),
                (cleanup_cursor_crosshair, bye_backpack_ui, bye_loadout_ui)
                    .chain()
                    .run_if(in_state(AppState::Raid))
                    .run_if(resource_exists::<LoadoutUI>),
            );
    }
}

// Components
#[derive(Component, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
struct EntityReference(Entity);

#[derive(Clone)]
struct Item<'a> {
    slot: &'a ItemSlot,
    name: Option<&'a LootName>,
    price: Option<&'a Price>,
    rarity: Option<&'a Rarity>,
    entity: Entity,
}

#[derive(Clone)]
struct Weapon<'a> {
    slot: &'a WeaponSlot,
    name: Option<&'a LootName>,
    entity: Entity,
}

#[derive(Component)]
struct LootCacheItem;

#[derive(Component)]
struct LootCacheWeapon;

#[derive(Component)]
struct BackpackItem;

#[derive(Component)]
struct BackpackWeapon;

#[derive(Component)]
struct LootCacheUI;

#[derive(Component)]
struct BackpackUI;

// Resources
#[derive(Resource)]
struct LootCacheEntities {
    loot_cache: Entity,
    backpack: Entity,
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

// Misc
enum InventoryUI {
    LootCache,
    Backpack,
    #[allow(dead_code)]
    Loadout,
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

fn render_loot_cache_ui(
    mut commands: Commands,
    loot_cache_name: String,
    loot_cache_items: Vec<Item>,
    loot_cache_item_slots: usize,
    loot_cache_weapons: Vec<Weapon>,
    loot_cache_weapon_slots: usize,
) {
    // Layout
    // Top-level grid (app frame)
    commands
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
        .insert(LootCacheUI)
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
                            if ((s.slot).0 as usize).eq(&weapon_slot_no) {
                                debug!("slot: {:?}", (s.slot).0);
                                create_weapon_slot_ui(builder, s.clone(), InventoryUI::LootCache);
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
                            if ((s.slot).0 as usize).eq(&item_slot_no) {
                                debug!("slot: {:?}", (s.slot).0);
                                create_item_slot_ui(builder, s.clone(), InventoryUI::LootCache);
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
        });
}

#[allow(clippy::type_complexity)]
fn start_loot_cache_ui(
    mut commands: Commands,
    loot_entities: Res<LootCacheEntities>,
    inventories_with_items: Query<(&ItemSlots, &Name), With<Inventory>>,
    inventory_items: Query<
        (
            &Parent,
            &ItemSlot,
            Option<&LootName>,
            Option<&Price>,
            Option<&Rarity>,
            Entity,
        ),
        With<Loot>,
    >,
    inventories_with_weapons: Query<&WeaponSlots, With<Inventory>>,
    inventory_weapons: Query<(&Parent, &WeaponSlot, Option<&LootName>, Entity), With<Loot>>,
    ui: Query<Entity, With<LootCacheUI>>,
) {
    debug!("start loot cache ui");

    if let Ok(ui) = ui.get_single() {
        commands.entity(ui).despawn_recursive();
    }

    let loot_cache = loot_entities.loot_cache;

    // Loot Cache
    let mut loot_cache_items: Vec<Item> = inventory_items
        .iter()
        .filter(|ii| loot_cache == ii.0.get())
        .map(|ii| Item {
            slot: ii.1,
            name: ii.2,
            price: ii.3,
            rarity: ii.4,
            entity: ii.5,
        })
        .collect();
    loot_cache_items.sort_by(|a, b| (a.slot).0.cmp(&(b.slot).0));

    let loot_cache_item_slots: usize = inventories_with_items
        .get(loot_cache)
        .map_or(0, |r| (r.0).0.into());

    let loot_cache_name: String = inventories_with_items
        .get(loot_cache)
        .map_or("".to_string(), |r| r.1.into());

    let mut loot_cache_weapons: Vec<Weapon> = inventory_weapons
        .iter()
        .filter(|ii| loot_cache == ii.0.get())
        .map(|ii| Weapon {
            slot: ii.1,
            name: ii.2,
            entity: ii.3,
        })
        .collect();
    loot_cache_weapons.sort_by(|a, b| (a.slot).0.cmp(&(b.slot).0));

    let loot_cache_weapon_slots: usize = inventories_with_weapons
        .get(loot_cache)
        .map_or(0, |r| r.0.into());

    render_loot_cache_ui(
        commands,
        loot_cache_name.clone(),
        loot_cache_items.clone(),
        loot_cache_item_slots,
        loot_cache_weapons.clone(),
        loot_cache_weapon_slots,
    );
}

fn render_backpack_ui(
    mut commands: Commands,
    backpack_name: String,
    backpack_items: Vec<Item>,
    backpack_item_slots: usize,
    backpack_weapons: Vec<Weapon>,
    backpack_weapon_slots: usize,
) {
    // Layout
    // Top-level grid (app frame)
    commands
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
        .insert(BackpackUI)
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
                            if ((s.slot).0 as usize).eq(&weapon_slot_no) {
                                debug!("slot: {:?}", (s.slot).0);
                                create_weapon_slot_ui(builder, s.clone(), InventoryUI::Backpack);
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
                            if ((s.slot).0 as usize).eq(&item_slot_no) {
                                debug!("slot: {:?}", (s.slot).0);
                                create_item_slot_ui(builder, s.clone(), InventoryUI::Backpack);
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
        });
}

#[allow(clippy::type_complexity)]
fn start_backpack_ui(
    mut commands: Commands,
    // MAYBE replace with a query for player controlled inventory for backpack
    loot_entities: Res<LootCacheEntities>,
    inventories_with_items: Query<(&ItemSlots, &Name), With<Inventory>>,
    inventory_items: Query<
        (
            &Parent,
            &ItemSlot,
            Option<&LootName>,
            Option<&Price>,
            Option<&Rarity>,
            Entity,
        ),
        With<Loot>,
    >,
    inventories_with_weapons: Query<&WeaponSlots, With<Inventory>>,
    inventory_weapons: Query<(&Parent, &WeaponSlot, Option<&LootName>, Entity), With<Loot>>,
    ui: Query<Entity, With<BackpackUI>>,
) {
    debug!("start backpack ui");

    if let Ok(ui) = ui.get_single() {
        commands.entity(ui).despawn_recursive();
    }

    let backpack = loot_entities.backpack;

    // Backpack
    let mut backpack_items: Vec<Item> = inventory_items
        .iter()
        .filter(|ii| backpack == ii.0.get())
        .map(|ii| Item {
            slot: ii.1,
            name: ii.2,
            price: ii.3,
            rarity: ii.4,
            entity: ii.5,
        })
        .collect();
    backpack_items.sort_by(|a, b| (a.slot).0.cmp(&(b.slot).0));

    let backpack_item_slots: usize = inventories_with_items
        .get(backpack)
        .map_or(0, |r| (r.0).0.into());

    let backpack_name: String = inventories_with_items
        .get(backpack)
        .map_or("".to_string(), |r| r.1.into());

    let mut backpack_weapons: Vec<Weapon> = inventory_weapons
        .iter()
        .filter(|ii| backpack == ii.0.get())
        .map(|ii| Weapon {
            slot: ii.1,
            name: ii.2,
            entity: ii.3,
        })
        .collect();
    backpack_weapons.sort_by(|a, b| (a.slot).0.cmp(&(b.slot).0));

    let backpack_weapon_slots: usize = inventories_with_weapons
        .get(backpack)
        .map_or(0, |r| r.0.into());

    render_backpack_ui(
        commands,
        backpack_name.clone(),
        backpack_items.clone(),
        backpack_item_slots,
        backpack_weapons.clone(),
        backpack_weapon_slots,
    );
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
#[allow(clippy::type_complexity)]
fn update_loot_cache_ui(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&LootCacheItem>,
            Option<&LootCacheWeapon>,
            &EntityReference,
        ),
        (
            Changed<Interaction>,
            Or<(With<LootCacheItem>, With<LootCacheWeapon>)>,
        ),
    >,
    loot_cache_entities: Res<LootCacheEntities>,
    mut stow_loot: EventWriter<StowLoot>,
) {
    debug!("updating loot cache ui");
    for (interaction, mut color, item, weapon, loot_entity) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                debug!("loot cache ui button pressed");
                *color = PRESSED_BUTTON.into();

                if item.is_some() {
                    stow_loot.send(StowLoot {
                        stowing_entity: loot_cache_entities.backpack,
                        loot: loot_entity.0,
                        loot_type: LootType::Item(crate::loot::ItemType::Item),
                    });
                }

                if weapon.is_some() {
                    stow_loot.send(StowLoot {
                        stowing_entity: loot_cache_entities.backpack,
                        loot: loot_entity.0,
                        loot_type: LootType::Weapon,
                    });
                }
            }
            Interaction::Hovered => {
                debug!("loot cache ui button hovered");
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                debug!("loot cache ui button normal");
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
fn update_stowed_loot_cache_ui(
    mut stowed_loot: EventReader<StowedLoot>,
    loot_cache_entities: Res<LootCacheEntities>,
    inventories_with_items: Query<(&ItemSlots, &Name), With<Inventory>>,
    inventory_items: Query<
        (
            &Parent,
            &ItemSlot,
            Option<&LootName>,
            Option<&Price>,
            Option<&Rarity>,
            Entity,
        ),
        With<Loot>,
    >,
    inventories_with_weapons: Query<&WeaponSlots, With<Inventory>>,
    inventory_weapons: Query<(&Parent, &WeaponSlot, Option<&LootName>, Entity), With<Loot>>,
    mut commands: Commands,
    ui: Query<Entity, With<LootCacheUI>>,
) {
    debug!("update stowed loot loot cache ui");
    for _ in stowed_loot.read() {
        if let Ok(ui) = ui.get_single() {
            commands.entity(ui).despawn_recursive();
        }

        // event contents dont matter, we get loot cache from next line
        let loot_cache = loot_cache_entities.loot_cache;

        // Loot Cache
        let mut loot_cache_items: Vec<Item> = inventory_items
            .iter()
            .filter(|ii| loot_cache == ii.0.get())
            .map(|ii| Item {
                slot: ii.1,
                name: ii.2,
                price: ii.3,
                rarity: ii.4,
                entity: ii.5,
            })
            .collect();
        loot_cache_items.sort_by(|a, b| (a.slot).0.cmp(&(b.slot).0));

        let loot_cache_item_slots: usize = inventories_with_items
            .get(loot_cache)
            .map_or(0, |r| (r.0).0.into());

        let loot_cache_name: String = inventories_with_items
            .get(loot_cache)
            .map_or("".to_string(), |r| r.1.into());

        let mut loot_cache_weapons: Vec<Weapon> = inventory_weapons
            .iter()
            .filter(|ii| loot_cache == ii.0.get())
            .map(|ii| Weapon {
                slot: ii.1,
                name: ii.2,
                entity: ii.3,
            })
            .collect();
        loot_cache_weapons.sort_by(|a, b| (a.slot).0.cmp(&(b.slot).0));

        let loot_cache_weapon_slots: usize = inventories_with_weapons
            .get(loot_cache)
            .map_or(0, |r| r.0.into());

        render_loot_cache_ui(
            commands.reborrow(),
            loot_cache_name,
            loot_cache_items,
            loot_cache_item_slots,
            loot_cache_weapons,
            loot_cache_weapon_slots,
        );
    }
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
fn update_stowed_loot_backpack_ui(
    mut stowed_loot: EventReader<StowedLoot>,
    loot_cache_entities: Res<LootCacheEntities>,
    inventories_with_items: Query<(&ItemSlots, &Name), With<Inventory>>,
    inventory_items: Query<
        (
            &Parent,
            &ItemSlot,
            Option<&LootName>,
            Option<&Price>,
            Option<&Rarity>,
            Entity,
        ),
        With<Loot>,
    >,
    inventories_with_weapons: Query<&WeaponSlots, With<Inventory>>,
    inventory_weapons: Query<(&Parent, &WeaponSlot, Option<&LootName>, Entity), With<Loot>>,
    mut commands: Commands,
    ui: Query<Entity, With<BackpackUI>>,
) {
    debug!("update stowed loot backpack ui");
    for _ in stowed_loot.read() {
        if let Ok(ui) = ui.get_single() {
            commands.entity(ui).despawn_recursive();
        }

        // event contents dont matter, we get backpack from next line
        let backpack = loot_cache_entities.backpack;

        // Backpack
        let mut backpack_items: Vec<Item> = inventory_items
            .iter()
            .filter(|ii| backpack == ii.0.get())
            .map(|ii| Item {
                slot: ii.1,
                name: ii.2,
                price: ii.3,
                rarity: ii.4,
                entity: ii.5,
            })
            .collect();
        backpack_items.sort_by(|a, b| (a.slot).0.cmp(&(b.slot).0));

        let backpack_item_slots: usize = inventories_with_items
            .get(backpack)
            .map_or(0, |r| (r.0).0.into());

        let backpack_name: String = inventories_with_items
            .get(backpack)
            .map_or("".to_string(), |r| r.1.into());

        let mut backpack_weapons: Vec<Weapon> = inventory_weapons
            .iter()
            .filter(|ii| backpack == ii.0.get())
            .map(|ii| Weapon {
                slot: ii.1,
                name: ii.2,
                entity: ii.3,
            })
            .collect();
        backpack_weapons.sort_by(|a, b| (a.slot).0.cmp(&(b.slot).0));

        let backpack_weapon_slots: usize = inventories_with_weapons
            .get(backpack)
            .map_or(0, |r| r.0.into());

        render_backpack_ui(
            commands.reborrow(),
            backpack_name.clone(),
            backpack_items.clone(),
            backpack_item_slots,
            backpack_weapons.clone(),
            backpack_weapon_slots,
        );
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

fn bye_loot_cache_ui(mut commands: Commands, loot_cache_ui: Query<Entity, With<LootCacheUI>>) {
    debug!("cleanup loot cache ui");
    let ui = loot_cache_ui.single();
    commands.entity(ui).despawn_recursive();
    commands.remove_resource::<LootCacheEntities>();
}

fn bye_backpack_ui(mut commands: Commands, backpack_ui: Query<Entity, With<BackpackUI>>) {
    debug!("cleanup backpack ui");
    let ui = backpack_ui.single();
    commands.entity(ui).despawn_recursive();
}

fn bye_loadout_ui(mut commands: Commands, loadout_ui: Res<LoadoutUI>) {
    debug!("cleanup loadout ui");
    commands.entity(loadout_ui.loadout_ui).despawn_recursive();
    commands.remove_resource::<LoadoutUI>();
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

fn create_weapon_slot_ui(builder: &mut ChildBuilder, weapon: Weapon, ui: InventoryUI) {
    // TODO: there must be a better way, this fugly
    let label: String = weapon.name.map(|x| x.0.clone()).unwrap_or("".to_string());
    let mut ui_weapon = builder.spawn((
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

    ui_weapon.with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            label,
            TextStyle {
                font_size: 8.0,
                color: Color::srgb(0.9, 0.9, 0.9),
                ..default()
            },
        ));
    });

    ui_weapon.insert(EntityReference(weapon.entity));

    match ui {
        InventoryUI::LootCache => ui_weapon.insert(LootCacheWeapon),
        InventoryUI::Backpack => ui_weapon.insert(BackpackWeapon),
        InventoryUI::Loadout => todo!(),
    };
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

fn create_item_slot_ui(builder: &mut ChildBuilder, item: Item, ui: InventoryUI) {
    // TODO: there must be a better way, this fugly
    let label: String = item.name.map(|x| x.0.clone()).unwrap_or("".to_string());
    let mut ui_item = builder.spawn((
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
            background_color: {
                match item.rarity {
                    Some(r) => match r {
                        Rarity::Regular => NORMAL_BUTTON.into(),
                        Rarity::Rare => Srgba {
                            red: 0.5,
                            green: 0.5,
                            blue: 0.0,
                            alpha: 1.0,
                        }
                        .into(),
                    },
                    None => NORMAL_BUTTON.into(),
                }
            },
            ..default()
        },
        Outline {
            width: Val::Px(6.),
            offset: Val::Px(6.),
            color: Color::WHITE,
        },
    ));

    ui_item.with_children(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    // Make the height of the node fill its parent
                    height: Val::Percent(100.0),
                    // Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
                    // As the height is set explicitly, this means the width will adjust to match the height
                    aspect_ratio: Some(1.0),
                    // Use grid layout for this node
                    display: Display::Grid,
                    // Set the grid to have 3 columns all with sizes minmax(0, 1fr)
                    // This creates 3 exactly evenly sized columns
                    grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                    // Set the grid to have 3 rows all with sizes minmax(0, 1fr)
                    // This creates 3 exactly evenly sized rows
                    grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
                    //border: UiRect::all(Val::Px(1.)),
                    ..default()
                },
                //border_color: Color::WHITE.into(),
                ..default()
            })
            .with_children(|parent| {
                // TOP
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            align_content: AlignContent::FlexStart,
                            justify_content: JustifyContent::FlexStart,
                            //border: UiRect::all(Val::Px(1.)),
                            ..default()
                        },
                        //border_color: Color::BLACK.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(
                                //String::from("1"),
                                String::from(""),
                                TextStyle {
                                    font_size: 8.0,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            )
                            .with_text_justify(JustifyText::Left),
                        );
                    });

                parent
                    .spawn(NodeBundle {
                        style: Style {
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::FlexStart,
                            //border: UiRect::all(Val::Px(1.)),
                            ..default()
                        },
                        //border_color: Color::BLACK.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(
                                label,
                                //String::from("2"),
                                TextStyle {
                                    font_size: 8.0,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            )
                            .with_text_justify(JustifyText::Center),
                        );
                    });

                parent
                    .spawn(NodeBundle {
                        style: Style {
                            align_content: AlignContent::FlexEnd,
                            justify_content: JustifyContent::FlexStart,
                            //border: UiRect::all(Val::Px(1.)),
                            ..default()
                        },
                        //border_color: Color::BLACK.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(
                                //String::from("3"),
                                String::from(""),
                                TextStyle {
                                    font_size: 8.0,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            )
                            .with_text_justify(JustifyText::Right),
                        );
                    });

                // MIDDLE
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            align_content: AlignContent::FlexStart,
                            justify_content: JustifyContent::Center,
                            //border: UiRect::all(Val::Px(1.)),
                            ..default()
                        },
                        //border_color: Color::BLACK.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(
                                //String::from("4"),
                                String::from(""),
                                TextStyle {
                                    font_size: 8.0,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            )
                            .with_text_justify(JustifyText::Left),
                        );
                    });

                parent
                    .spawn(NodeBundle {
                        style: Style {
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::Center,
                            //border: UiRect::all(Val::Px(1.)),
                            ..default()
                        },
                        //border_color: Color::BLACK.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(
                                //String::from("5"),
                                String::from(""),
                                TextStyle {
                                    font_size: 8.0,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            )
                            .with_text_justify(JustifyText::Center),
                        );
                    });

                parent
                    .spawn(NodeBundle {
                        style: Style {
                            align_content: AlignContent::FlexEnd,
                            justify_content: JustifyContent::Center,
                            //border: UiRect::all(Val::Px(1.)),
                            ..default()
                        },
                        //border_color: Color::BLACK.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(
                                //String::from("6"),
                                String::from(""),
                                TextStyle {
                                    font_size: 8.0,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            )
                            .with_text_justify(JustifyText::Right),
                        );
                    });

                // BOTTOM
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            align_content: AlignContent::FlexStart,
                            justify_content: JustifyContent::FlexEnd,
                            //border: UiRect::all(Val::Px(1.)),
                            ..default()
                        },
                        //border_color: Color::BLACK.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(
                                //String::from("7"),
                                String::from(""),
                                TextStyle {
                                    font_size: 8.0,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            )
                            .with_text_justify(JustifyText::Left),
                        );
                    });

                parent
                    .spawn(NodeBundle {
                        style: Style {
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::FlexEnd,
                            //border: UiRect::all(Val::Px(1.)),
                            ..default()
                        },
                        //border_color: Color::BLACK.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(
                                //String::from("8"),
                                String::from(""),
                                TextStyle {
                                    font_size: 8.0,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            )
                            .with_text_justify(JustifyText::Center),
                        );
                    });

                parent
                    .spawn(NodeBundle {
                        style: Style {
                            align_content: AlignContent::FlexEnd,
                            justify_content: JustifyContent::FlexEnd,
                            //border: UiRect::all(Val::Px(1.)),
                            ..default()
                        },
                        //border_color: Color::BLACK.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        if let Some(price) = item.price {
                            debug!("price: {:?}", price.0);
                            parent.spawn(
                                TextBundle::from_section(
                                    format!("${}", price.0),
                                    //String::from("9"),
                                    TextStyle {
                                        font_size: 8.0,
                                        color: Color::srgb(0.9, 0.9, 0.9),
                                        ..default()
                                    },
                                )
                                .with_text_justify(JustifyText::Right),
                            );
                        }
                    });
            });
    });

    ui_item.insert(EntityReference(item.entity));

    match ui {
        InventoryUI::LootCache => ui_item.insert(LootCacheItem),
        InventoryUI::Backpack => ui_item.insert(BackpackItem),
        InventoryUI::Loadout => todo!(),
    };
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
