use bevy::app::Plugin;
use bevy_inspector_egui::prelude::*;

use crate::exfil::Operator;
use crate::AppState;
use crate::AppState::Raid;
use bevy::prelude::*;

// Constants
const NAME: &str = "loot";

// Plugin
pub struct LootPlugin;

impl Plugin for LootPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Durability>()
            .add_event::<DroppedLoot>()
            .add_systems(OnEnter(Raid), start_loot_system)
            .add_systems(
                Update,
                (update_loot_system).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_loot_system);
    }
}

// Components

#[derive(Component)]
pub struct Loot;

/// marker template for tagging loot entities that are in proximity to any operator
#[derive(Component)]
#[allow(dead_code)]
pub struct Proximity;

#[derive(Component, Clone)]
pub struct LootName(pub String);

// TODO: fix some of the research language conflicts: is Gasmask a CircleDefense or Item???

// some of the wording is straight from the screenshots of loot and buystation terms
// they might not be used ubuiqitously by gamers but by the dev and might have their
// origin from WZ (CircleDefense hints at items to be used in the gas, ironically
// the personal exfil is listed as CircleDefense in the buy stations, lowkey hinting
// that the personal exfil is in fact meant to be used inside the gas!).
// also the loot type is more like a type/category and the actual item is an implementation
// of that type/category: LastStand = type, implementations = self revive, battlerage self revive
#[derive(Component, Clone, Debug, PartialEq)]
pub enum LootType {
    Item(ItemType),
    Weapon,
    #[allow(dead_code)]
    Ammo,
    #[allow(dead_code)]
    Lethal,
    #[allow(dead_code)]
    Tactical,
    CombatDefense, // like armor plates
    #[allow(dead_code)]
    FieldUpgrade,
    #[allow(dead_code)]
    KillStreak,
    #[allow(dead_code)]
    CircleDefense, // gasmask
    #[allow(dead_code)]
    RadiationProtection, // radiation meds
    #[allow(dead_code)]
    LastStand, // self revive
    #[allow(dead_code)]
    Cash,
    #[allow(dead_code)]
    Intel,
    #[allow(dead_code)]
    Key,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    #[allow(dead_code)]
    Equipment, // example: vests
    Item,
}

/// price/cash amount per item in cent amount
#[derive(Component)]
#[allow(dead_code)]
pub struct Price(pub u32);

#[derive(Component)]
pub struct Stackable {
    #[allow(dead_code)]
    pub max_stack: u32,
    #[allow(dead_code)]
    pub current_stack: u32,
}

#[derive(Component)]
pub enum Rarity {
    /// represented by transparent/grey background
    #[allow(dead_code)]
    Regular,
    /// represented by golden background
    Rare,
}

/// represented by blue item background
#[derive(Component)]
#[allow(dead_code)]
pub struct Stashable;

/// uses can be subject to rng, not every use will progress usage.
/// this is up to the system to decide not the component.
/// the components is just a storage of the current usage state.
#[derive(Component)]
#[allow(dead_code)]
pub enum Uses {
    /// fresh and pristine
    Pristine,
    /// used once or multiple times
    Used,
    /// most likely the last usage
    Worn,
}

/// items like gasmaks have only a certain amount like 100% that depletes over usage
/// shows a percentage on UI
//#[derive(Component)]
#[derive(Component, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Durability {
    pub max: u8,
    pub current: u8,
}

impl Default for Durability {
    fn default() -> Self {
        Durability {
            max: 100,
            current: 100,
        }
    }
}

impl Durability {
    #[allow(dead_code)] // not dead code i use it in unit tests!
    fn percent(&self) -> u8 {
        let max: u32 = self.max.into();
        let current: u32 = self.current.into();
        (current * 100 / max).try_into().unwrap()
    }
}

// TODO: maybe listener for Inventor components to attach LootCacheState automatically?
/// loot cache state machine enum
#[derive(Component, Default, Debug, PartialEq)]
#[allow(dead_code)] // not dead code i use it in unit tests!
pub enum LootCacheState {
    Locked,
    #[default]
    Closed,
    Open,
    Empty,
}

#[allow(dead_code)] // not dead code i use it in unit tests!
impl LootCacheState {
    fn next(self) -> LootCacheState {
        match self {
            LootCacheState::Locked => Self::Closed,
            LootCacheState::Closed => Self::Open,
            LootCacheState::Open => Self::Empty,
            LootCacheState::Empty => Self::Empty,
        }
    }
}

// Resources

// Events
#[derive(Event, Debug, PartialEq)]
pub struct DroppedLoot {
    pub dropping_entity: Entity,
    pub dropped_position: Vec3,
    pub loot: Entity,
}

#[derive(Event)]
#[allow(dead_code)]
pub struct LootPickupAvailable {
    pub operator_entity: Entity,
    pub loot_entity: Entity,
}

#[derive(Event)]
#[allow(dead_code)]
pub struct LootPickupUnavailable {
    pub operator_entity: Entity,
    pub loot_entity: Entity,
}

// Systems
fn start_loot_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn update_loot_system() {
    debug!("updating {}", NAME);
}

// TODO: how to keep track in we enter or leave proximity?
fn _loot_proximity_detection(
    _loot_query: Query<(Entity, &GlobalTransform, &Loot), With<Loot>>,
    _operator_query: Query<(Entity, &GlobalTransform), With<Operator>>,
    _loot_available: EventWriter<LootPickupAvailable>,
    _loot_unavailable: EventWriter<LootPickupUnavailable>,
) {
    // proximity distance hardcoded for now
    let _min_distance = 2.0;
    // TODO: nest loop over all operators and loot items and mark loot items in proximity with marker component
    // TODO: send out events depending and addage or removal of said marker templates
}

fn bye_loot_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions

// tests
#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_calculate_durabilities() {
        // given
        let full = Durability {
            max: 100,
            current: 100,
        };
        let half = Durability {
            max: 100,
            current: 50,
        };
        let quarter = Durability {
            max: 100,
            current: 25,
        };
        let zero = Durability {
            max: 100,
            current: 0,
        };
        // when
        let full_durability = full.percent();
        let half_durability = half.percent();
        let quarter_durability = quarter.percent();
        let zero_durability = zero.percent();

        // then
        assert_eq!(100, full_durability);
        assert_eq!(50, half_durability);
        assert_eq!(25, quarter_durability);
        assert_eq!(0, zero_durability);
    }

    #[test]
    fn should_give_next_loot_cache_state() {
        // given
        let locked = LootCacheState::Locked;
        let closed = LootCacheState::Closed;
        let open = LootCacheState::Open;
        let empty = LootCacheState::Empty;

        // when / then
        let after_locked = locked.next();
        let after_closed = closed.next();
        let after_open = open.next();
        let after_empty = empty.next();

        // then
        assert_eq!(LootCacheState::Closed, after_locked);
        assert_eq!(LootCacheState::Open, after_closed);
        assert_eq!(LootCacheState::Empty, after_open);
        assert_eq!(LootCacheState::Empty, after_empty);
    }
}
