use bevy::app::Plugin;
use bevy_inspector_egui::prelude::*;

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

#[derive(Component)]
pub struct LootName(pub String);

// TODO: fix some of the research language conflicts: is Gasmask a CircleDefense or Item???

// some of the wording is straight from the screenshots of loot and buystation terms
// they might not be used ubuiqitously by gamers but by the dev and might have their
// origin from WZ (CircleDefense hints at items to be used in the gas, ironically
// the personal exfil is listed as CircleDefense in the buy stations, lowkey hinting
// that the personal exfil is in fact meant to be used inside the gas!).
// also the loot type is more like a type/category and the actual item is an implementation
// of that type/category: LastStand = type, implementations = self revive, battlerage self revive
#[derive(Component)]
pub enum LootType {
    Item(ItemType),
    Weapon,
    Ammo,
    Lethal,
    Tactical,
    CombatDefense, // like armor plates
    FieldUpgrade,
    KillStreak,
    CircleDefense,       // gasmask
    RadiationProtection, // radiation meds
    LastStand,           // self revive
    Cash,
    Intel,
    Key,
}

pub enum ItemType {
    Equipment, // example: vests
    Item,
}

/// price/cash amount per item in cent amount
#[derive(Component)]
pub struct Price(pub u32);

#[derive(Component)]
pub struct Stackable {
    pub max_stack: u32,
    pub current_stack: u32,
}

#[derive(Component)]
pub enum Rarity {
    /// represented by transparent/grey background
    Regular,
    /// represented by golden background
    Rare,
}

/// represented by blue item background
#[derive(Component)]
pub struct Stashable;

/// uses can be subject to rng, not every use will progress usage.
/// this is up to the system to decide not the component.
/// the components is just a storage of the current usage state.
#[derive(Component)]
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
    fn percent(&self) -> u8 {
        let max: u32 = self.max.into();
        let current: u32 = self.current.into();
        (current * 100 / max).try_into().unwrap()
    }
}

// Resources

// Events

// Systems
fn start_loot_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}
fn update_loot_system() {
    debug!("updating {}", NAME);
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
    fn should_test_something() {
        // given
        let mut app = App::new();

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
