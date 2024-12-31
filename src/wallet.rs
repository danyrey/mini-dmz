use bevy::app::Plugin;

use crate::exfil::Operator;
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
            .add_event::<StowMoney>()
            .add_event::<StowedMoney>()
            .add_event::<DropMoney>()
            .add_event::<DroppedMoney>()
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
    pub money_entity: Entity,
}

/// event for stowed money
#[derive(Event, Debug, PartialEq)]
pub struct StowedMoney {
    pub stowing_entity: Entity,
    pub amount: u32,
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

/// reacting to the
/// ```
/// DropMoney
/// ```
/// 'comand'/event and dropping an entity on the ground the same position of the operator. The
/// event:
/// ```
/// DroppedMoney
/// ```
/// is dispatched for system interested in dropped money.
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
    mut commands: Commands,
    mut command: EventReader<StowMoney>,
    money_entities: Query<(Entity, &Price), With<Money>>,
    mut operator_entities: Query<(Entity, &mut Wallet), With<Operator>>,
    mut event: EventWriter<StowedMoney>,
) {
    debug!("stowing money {}", NAME);
    for c in command.read() {
        debug!("m: {}, o: {}", c.money_entity, c.stowing_entity);
        let money_query = money_entities.get(c.money_entity);
        let operator_query = operator_entities.get_mut(c.stowing_entity);
        // TODO: y u no op and monero???
        if let Ok(mut operator) = operator_query {
            debug!("we got operator, no dinero yet");
            if let Ok(money) = money_query {
                debug!("we got operator, we got dinero");
                let amount = (money.1).0;
                operator.1.money += amount;
                commands.entity(money.0).despawn_recursive();
                event.send(StowedMoney {
                    stowing_entity: operator.0,
                    amount,
                });
            }
        }
    }
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
        // wallet was reduced
        assert_eq!(
            249_900,
            app.world().get::<Wallet>(operator_id).unwrap().money
        );
        // new ground money exists with correct amount and position of operator
        let mut query = app
            .world_mut()
            .query::<(Entity, &Money, &Price, &GlobalTransform)>();
        let money = query.single(app.world());
        assert_eq!(100, (money.2).0);
        assert_eq!(operator_position, (money.3).translation());

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
    }

    #[test]
    fn should_stow_money() {
        // given
        let mut app = App::new();

        app.add_event::<StowMoney>();
        app.add_event::<StowedMoney>();
        app.add_systems(Update, stow_money_listener);

        // when
        let mut ground_money = app.world_mut().spawn(Money);

        let money_position = Vec3::new(1.0, 2.0, 3.0);
        let money_transform = Transform::from_translation(money_position);
        let money_amount: u32 = 100;

        ground_money.insert(money_transform);
        ground_money.insert(GlobalTransform::from(money_transform));
        ground_money.insert(Price(money_amount));
        let money_id = ground_money.id();

        let mut operator = app.world_mut().spawn(Operator);
        let wallet = Wallet {
            money: 0,
            ..default()
        };
        operator.insert(wallet);
        let operator_id = operator.id();

        app.world_mut()
            .resource_mut::<Events<StowMoney>>()
            .send(StowMoney {
                stowing_entity: operator_id,
                money_entity: money_id,
            });

        // run an update on the app
        app.update();

        // then
        assert_eq!(
            100,
            app.world().get::<Wallet>(operator_id).unwrap().money,
            "amount in wallet should be 100"
        );
        let mut query = app.world_mut().query::<&Money>();
        let money_query = query.get_single(app.world());
        assert!(
            money_query.is_err(),
            "Money entity should have been removed"
        );

        // check for event StowedMoney

        let stowed_money_events = app.world().resource::<Events<StowedMoney>>();
        let mut dropped_money_reader = stowed_money_events.get_reader();
        let actual_dropped_money = dropped_money_reader.read(stowed_money_events).next();
        let expected_dropped_money = StowedMoney {
            stowing_entity: operator_id,
            amount: money_amount,
        };
        assert!(
            actual_dropped_money.is_some(),
            "event StowedMoney is present"
        );
        assert_eq!(
            &expected_dropped_money,
            actual_dropped_money.unwrap(),
            "StowedMoney contains correct operator entity and amount"
        );
    }
}
