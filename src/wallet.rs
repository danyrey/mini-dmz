use bevy::app::Plugin;

use crate::AppState::Raid;
use crate::{loot::Price, AppState};
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

use bevy::prelude::*;

// Constants
const NAME: &str = "wallet";

// Plugin
pub struct WalletPlugin;

impl Plugin for WalletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_wallet_system)
            .register_type::<Wallet>()
            .add_systems(
                Update,
                (update_wallet_system).run_if(in_state(AppState::Raid)),
            )
            .add_systems(
                Update,
                (stow_money_listener, drop_money_listener).run_if(in_state(AppState::Raid)),
            )
            .add_systems(OnExit(AppState::Raid), bye_wallet_system);
    }
}

// Components

#[derive(Component)]
pub struct Money;

#[derive(Component, Debug, PartialEq, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Wallet {
    pub money: u32,
    pub limit: u32,
}

impl Default for Wallet {
    fn default() -> Self {
        Wallet {
            money: 0,
            limit: 250_000,
        }
    }
}
// Resources

// Events

/// command for stowing money
#[derive(Event, Debug, PartialEq)]
pub struct StowMoney {
    pub stowing_entity: Entity,
    pub money: Entity,
}

/// event for stowed money
#[derive(Event, Debug, PartialEq)]
pub struct StowedMoney {
    pub stowing_entity: Entity,
    pub money: Entity,
}

#[derive(Event, Debug, PartialEq)]
pub struct DropMoney {
    pub dropping_entity: Entity,
    pub amount: u32,
}

#[derive(Event, Debug, PartialEq)]
pub struct DroppedMoney {
    pub dropping_entity: Entity,
    pub dropped_position: Vec3,
    pub money: Entity,
}

// Systems
fn start_wallet_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn drop_money_listener(
    mut commands: Commands,
    mut command: EventReader<DropMoney>,
    mut wallet_entities: Query<(Entity, &mut Wallet, &GlobalTransform)>,
    mut event: EventWriter<DroppedMoney>,
) {
    debug!("dropping money {}", NAME);
    for c in command.read() {
        if let Ok((operator, mut wallet, global_transform)) =
            wallet_entities.get_mut(c.dropping_entity)
        {
            // TODO: transfer money amount from wallet to new money entity
            // TODO: send event DroppedMoney
            let money = commands.spawn(Money).id();
            wallet.money -= c.amount;
            commands.entity(money).insert(Price(c.amount));
            commands
                .entity(money)
                .insert(global_transform.compute_transform());
            commands.entity(money).insert(*global_transform);
            event.send(DroppedMoney {
                dropping_entity: operator,
                dropped_position: global_transform.translation(),
                money,
            });
        }
    }
}

fn stow_money_listener(
    mut command: EventReader<StowMoney>,
    // TODO: query
    mut event: EventWriter<StowedMoney>,
) {
    debug!("stowing money {}", NAME);
}

fn update_wallet_system() {
    debug!("updating {}", NAME);
}

fn bye_wallet_system(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    use crate::{exfil::Operator, loot::Price};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_drop_money() {
        // given
        let mut app = App::new();

        // when
        app.add_event::<DropMoney>();
        app.add_event::<DroppedMoney>();
        app.add_systems(Update, drop_money_listener);

        let mut operator_with_wallet = app.world_mut().spawn(Operator);
        let wallet = Wallet {
            money: 250_000,
            ..default()
        };
        let operator_position = Vec3::new(1.0, 2.0, 3.0);
        let operator_transform = Transform::from_translation(operator_position);

        operator_with_wallet.insert(wallet);
        operator_with_wallet.insert(operator_transform);
        operator_with_wallet.insert(GlobalTransform::from(operator_transform));
        let operator_id = operator_with_wallet.id();

        // run an update on the app once for the initial comand
        app.world_mut()
            .resource_mut::<Events<DropMoney>>()
            .send(DropMoney {
                dropping_entity: operator_id,
                amount: 100,
            });
        app.update();

        // then
        //assert!(app.world.get::<Health>(entity).is_some());
        assert_eq!(
            249_900,
            app.world().get::<Wallet>(operator_id).unwrap().money
        );
        let mut query = app
            .world_mut()
            .query::<(Entity, &Money, &Price, &GlobalTransform)>();
        let money = query.single(app.world());
        assert_eq!(100, (money.2).0);
        assert_eq!(operator_position, (money.3).translation());

        // run an update once more for checking the DroppedMoney event
        //app.update();

        let dropped_money_events = app.world().resource::<Events<DroppedMoney>>();
        let mut dropped_money_reader = dropped_money_events.get_reader();
        let actual_dropped_money = dropped_money_reader
            .read(dropped_money_events)
            .next()
            .unwrap();
        let expected_dropped_money = DroppedMoney {
            dropping_entity: operator_id,
            dropped_position: operator_position,
            money: money.0,
        };
        assert_eq!(&expected_dropped_money, actual_dropped_money);

        // then
        //assert!(app.world.get::<Health>(entity).is_some());
        //assert_eq!(app.world.get::<Health>(entity).unwrap().0, 90);
    }

    //#[test]
    //fn should_drop_money() {
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
    //}

    //#[test]
    fn should_stow_money() {
        // given
        let mut app = App::new();

        // when
        app.add_event::<StowMoney>();
        app.add_event::<StowedMoney>();
        //app.add_systems(Update, stow_money_listener);

        // run an update on the app
        app.update();
        todo!();

        // then
        //assert!(app.world.get::<Health>(entity).is_some());
        //assert_eq!(app.world.get::<Health>(entity).unwrap().0, 90);
    }
}
