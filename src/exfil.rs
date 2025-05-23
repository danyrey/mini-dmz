use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::prelude::*;

use crate::{
    first_person_controller::PlayerControlled,
    AppState::{Raid, StartScreen},
    ButtonTargetState,
};

// Events

#[derive(Event)]
pub struct ExfilCreated {
    #[allow(dead_code)] // not in use yet, can be used for starting particle system
    pub exfil_entity: Entity,
}

trait OriginState {
    const ORIGIN_STATE: ExfilState;
}

#[derive(Event)]
pub struct ExfilCalled {
    pub exfil_entity: Entity,
    pub calling_entity: Entity,
}

impl OriginState for ExfilCalled {
    const ORIGIN_STATE: ExfilState = ExfilState::Available;
}

#[allow(dead_code)]
#[derive(Event)]
pub struct ExfilEnteredAO;

#[allow(dead_code)]
#[derive(Event)]
pub struct ExfilSpawned;

#[allow(dead_code)]
#[derive(Event)]
pub struct ExfilApproached;

#[allow(dead_code)]
#[derive(Event)]
pub struct ExfilDescended;

#[allow(dead_code)]
#[derive(Event)]
pub struct ExfilLandingHover;

#[allow(dead_code)]
#[derive(Event)]
pub struct ExfilTouchedDown;

#[allow(dead_code)]
#[derive(Event)]
pub struct ExfilBoardingHold;

#[allow(dead_code)]
#[derive(Event)]
pub struct ExfilTookOff;

#[allow(dead_code)]
#[derive(Event)]
pub struct ExfilClimbed;

#[allow(dead_code)]
#[derive(Event)]
pub struct ExfilCruised;

#[derive(Event)]
pub struct ExfilExitedAO {
    pub operator_entity: Entity,
}

#[allow(dead_code)]
#[derive(Event)]
pub struct ExfilCooledDown;

#[derive(Event)]
pub struct Exfilled {
    #[allow(dead_code)] // not in use yet, can be used for starting particle system
    pub exfil_entity: Entity,
    // TODO: put the operator in here somehow
}

/// trigger for showing the prompt
#[derive(Event)]
pub struct ExfilAreaEntered {
    pub operator_entity: Entity,
    pub exfil_area: Entity,
}

// trigger for hiding the prompt again
#[derive(Event)]
pub struct ExfilAreaExited {
    pub operator_entity: Entity,
    pub exfil_area: ExfilArea,
}

// Resources

#[derive(Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Exfils {
    map: HashMap<Entity, Exfil>,
}

/// enum for the exfil procedure state machine
#[derive(Default, Debug, PartialEq, Clone, Copy, Reflect)]
pub enum ExfilState {
    #[default]
    Available,
    Called,
    EnteredAO,
    Spawned,
    Approached,
    Descended,
    LandingHover,
    TouchedDown,
    BoardingHold,
    TookOff,
    Climbed,
    Cruised,
    ExitedAO,
    Cooldown,
}

trait ExfilStateMachine {
    fn next(&mut self) -> ExfilState;
}

#[derive(Default, Reflect)]
struct Exfil {
    current_state: ExfilState,
    current_timer: Option<Timer>,
    exfil_operator: Option<Entity>,
}

impl ExfilStateMachine for Exfil {
    fn next(&mut self) -> ExfilState {
        let default_timer = Timer::new(Duration::from_secs(1), TimerMode::Once);
        debug!("switching exfil state from {:?}", self.current_state);
        self.current_state = match self.current_state {
            ExfilState::Available => ExfilState::Called,
            ExfilState::Called => ExfilState::EnteredAO,
            ExfilState::EnteredAO => ExfilState::Spawned,
            ExfilState::Spawned => ExfilState::Approached,
            ExfilState::Approached => ExfilState::Descended,
            ExfilState::Descended => ExfilState::LandingHover,
            ExfilState::LandingHover => ExfilState::TouchedDown,
            ExfilState::TouchedDown => ExfilState::BoardingHold,
            ExfilState::BoardingHold => ExfilState::TookOff,
            ExfilState::TookOff => ExfilState::Climbed,
            ExfilState::Climbed => ExfilState::Cruised,
            ExfilState::Cruised => ExfilState::ExitedAO,
            ExfilState::ExitedAO => ExfilState::Cooldown,
            ExfilState::Cooldown => ExfilState::Available,
        };
        self.current_timer = match self.current_state {
            ExfilState::Available => None,
            ExfilState::Called => Some(default_timer),
            ExfilState::EnteredAO => Some(default_timer),
            ExfilState::Spawned => Some(default_timer),
            ExfilState::Approached => Some(default_timer),
            ExfilState::Descended => Some(default_timer),
            ExfilState::LandingHover => Some(default_timer),
            ExfilState::TouchedDown => Some(default_timer),
            ExfilState::BoardingHold => Some(default_timer),
            ExfilState::TookOff => Some(default_timer),
            ExfilState::Climbed => Some(default_timer),
            ExfilState::Cruised => Some(default_timer),
            ExfilState::ExitedAO => Some(default_timer),
            ExfilState::Cooldown => Some(default_timer),
        };
        debug!("switching exfil state to {:?}", self.current_state);
        self.current_state
    }
}

// Resources

#[derive(Resource)]
struct ExfilUIData {
    exfil_button_entity: Entity,
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// TODO: counters for all other phases of exfil

// Plugin

pub struct ExfilPlugin;

impl Plugin for ExfilPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Raid), start_exfil)
            .add_systems(
                Update,
                (
                    update_exfil,
                    progress_exfils,
                    exfil_created,
                    exfil_area_collision_detection,
                    exfil_area_entered,
                    exfil_area_exited,
                    exfil_called,
                )
                    .run_if(in_state(Raid)),
            )
            .add_systems(OnExit(Raid), bye_exfil)
            .add_event::<ExfilAreaEntered>()
            .add_event::<ExfilAreaExited>()
            .init_resource::<Exfils>()
            .register_type::<Exfils>()
            .register_type::<CurrentExfil>()
            .add_event::<ExfilCreated>()
            .add_event::<ExfilCalled>()
            .add_event::<Exfilled>()
            .add_event::<ExfilExitedAO>();
    }
}

// Components

#[derive(Component, Clone, Debug, Default)]
pub struct ExfilArea(pub String);

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct CurrentExfil(pub Entity);

#[derive(Component)]
struct ExfilButton;

// potential issues: overlapping/multiple exfils might be an issue or not
#[derive(Component, Clone, Debug, Default)]
pub struct InsideExfilArea(ExfilArea);

#[derive(Component)]
pub struct Operator;

// Systems

fn start_exfil(mut commands: Commands) {
    debug!("starting exfil called");
    let exfil_button_entity = commands
        .spawn(Node {
            // center button
            width: Val::Percent(80.),
            height: Val::Percent(120.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .insert(Visibility::Hidden)
        .with_children(|parent| {
            parent
                .spawn(Button)
                .insert(Node {
                    width: Val::Px(150.),
                    height: Val::Px(110.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .insert(ImageNode {
                    color: NORMAL_BUTTON,
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(Text::new("EXFIL"))
                        .insert(TextFont {
                            font_size: 40.0,
                            ..default()
                        })
                        .insert(TextColor(Color::srgb(0.9, 0.9, 0.9)));
                })
                .insert(ButtonTargetState(StartScreen));
        })
        .id();

    commands.insert_resource(ExfilUIData {
        exfil_button_entity,
    });

    commands.insert_resource(Exfils {
        map: HashMap::new(),
    });

    commands
        .entity(exfil_button_entity)
        .insert(Name::new("Exfil Button"))
        .insert(ExfilButton);
}

fn exfil_created(
    query: Query<Entity, Added<ExfilArea>>,
    mut exfil_spawn: EventWriter<ExfilCreated>,
    mut exfil_map: ResMut<Exfils>,
) {
    for entity in query.iter() {
        debug!("exfil was spawned");
        exfil_map.map.insert(entity, Exfil::default());
        exfil_spawn.send(ExfilCreated {
            exfil_entity: entity,
        });
    }
}

/// system that progresses all timers for all current exfils
fn progress_exfils(
    mut exfils: ResMut<Exfils>,
    time: Res<Time>,
    mut exit_ao: EventWriter<ExfilExitedAO>,
) {
    for (entity, exfil) in exfils.map.iter_mut() {
        if let Some(timer) = &mut exfil.current_timer {
            timer.tick(time.delta());
            if timer.just_finished() {
                debug!("Exfil({:?}) timer just finished!", entity);
                // TODO: emit event for state progression
                if exfil.current_state != ExfilState::Available {
                    exfil.next(); // shortcut / hack until fully implemented
                }
                if exfil.current_state == ExfilState::ExitedAO {
                    if let Some(operator_entity) = exfil.exfil_operator {
                        exit_ao.send(ExfilExitedAO { operator_entity });
                    }
                }
            } else if !timer.finished() {
                debug!("Exfil({:?}) timer is at {:?}!", entity, timer);
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_exfil(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Parent),
        Changed<Interaction>,
    >,
    exfil_button_query: Query<&CurrentExfil, With<ExfilButton>>,
    mut called: EventWriter<ExfilCalled>,
    player_query: Query<Entity, With<PlayerControlled>>,
) {
    // TODO: we need to put the calling/clicking operator to the ExfilCalled event
    debug!("exfil update called");
    let caller = player_query.single();
    for (interaction, mut color, parent) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();

                if let Ok(exfil) = exfil_button_query.get(**parent) {
                    called.send(ExfilCalled {
                        exfil_entity: exfil.0,
                        calling_entity: caller,
                    });
                }
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

/// exfil called event listener
fn exfil_called(mut exfil_called: EventReader<ExfilCalled>, mut exfils: ResMut<Exfils>) {
    for event in exfil_called.read() {
        debug!("exfil called event received");
        let exfil = exfils.map.get_mut(&event.exfil_entity);
        if let Some(e) = exfil {
            if e.current_state == ExfilCalled::ORIGIN_STATE {
                e.next();
                e.exfil_operator = Some(event.calling_entity);
            } else {
                debug!("wrong origin state, event ignored");
            }
        }
    }
}

fn bye_exfil(mut commands: Commands, menu_data: Res<ExfilUIData>) {
    debug!("exiting exfil called");
    commands
        .entity(menu_data.exfil_button_entity)
        .despawn_recursive();
}

fn exfil_area_entered(
    mut commands: Commands,
    mut entered: EventReader<ExfilAreaEntered>,
    query: Query<Entity, (With<ExfilButton>, With<Visibility>)>,
) {
    for event in entered.read() {
        debug!("entered exfil zone");
        for entity in query.iter() {
            debug!(
                "operator({:?}) entered exfil zone({:?})",
                event.operator_entity, event.exfil_area
            );
            commands
                .entity(entity)
                .insert(CurrentExfil(event.exfil_area));
            commands.entity(entity).insert(Visibility::Visible);
        }
    }
}

fn exfil_area_exited(
    mut commands: Commands,
    mut exited: EventReader<ExfilAreaExited>,
    query: Query<Entity, (With<ExfilButton>, With<Visibility>)>,
) {
    for event in exited.read() {
        debug!("exited exfil zone");
        for entity in query.iter() {
            debug!(
                "operator({:?}) exited exfil zone({:?})",
                event.operator_entity, event.exfil_area
            );
            commands.entity(entity).remove::<CurrentExfil>();
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
}

fn exfil_area_collision_detection(
    mut commands: Commands,
    exfil_query: Query<(Entity, &GlobalTransform, &ExfilArea), With<ExfilArea>>,
    mut operator_query: Query<
        (Entity, &GlobalTransform, Option<&mut InsideExfilArea>),
        With<Operator>,
    >,
    mut entered: EventWriter<ExfilAreaEntered>,
    mut exited: EventWriter<ExfilAreaExited>,
) {
    // TODO: make it a resource maybe
    let min_distance = 5.0;

    for (operator_entity, operator_transform, operator_exfil_area) in operator_query.iter_mut() {
        let mut any_exfil_area: Option<(Entity, ExfilArea)> = None;

        for (exfil_entity, exfil_transform, exfil_area) in exfil_query.iter() {
            let distance = exfil_transform
                .translation()
                .distance(operator_transform.translation());

            if distance < min_distance {
                any_exfil_area = Some((exfil_entity, exfil_area.clone()));
            }
        }

        if let Some(area) = any_exfil_area {
            if let Some(mut component_inside) = operator_exfil_area {
                if !(area.1).0.eq(&(component_inside.0).0) {
                    component_inside.0 = area.1.clone();
                    entered.send(ExfilAreaEntered {
                        operator_entity,
                        exfil_area: area.0,
                    });
                }
            } else {
                commands
                    .entity(operator_entity)
                    .insert(InsideExfilArea(area.1.clone()));
                entered.send(ExfilAreaEntered {
                    operator_entity,
                    exfil_area: area.0,
                });
            }
        } else if let Some(component) = operator_exfil_area {
            commands.entity(operator_entity).remove::<InsideExfilArea>();
            exited.send(ExfilAreaExited {
                operator_entity,
                exfil_area: component.0.clone(),
            });
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_exfil() {
        // given
        let mut exfil = Exfil::default();

        // when & then
        assert_eq!(ExfilState::Available, exfil.current_state);
        assert_eq!(ExfilState::Called, exfil.next());
        assert_eq!(ExfilState::Called, exfil.current_state);
        assert_eq!(ExfilState::EnteredAO, exfil.next());
        assert_eq!(ExfilState::EnteredAO, exfil.current_state);
        assert_eq!(ExfilState::Spawned, exfil.next());
        assert_eq!(ExfilState::Spawned, exfil.current_state);
        assert_eq!(ExfilState::Approached, exfil.next());
        assert_eq!(ExfilState::Approached, exfil.current_state);
        assert_eq!(ExfilState::Descended, exfil.next());
        assert_eq!(ExfilState::Descended, exfil.current_state);
        assert_eq!(ExfilState::LandingHover, exfil.next());
        assert_eq!(ExfilState::LandingHover, exfil.current_state);
        assert_eq!(ExfilState::TouchedDown, exfil.next());
        assert_eq!(ExfilState::TouchedDown, exfil.current_state);
        assert_eq!(ExfilState::BoardingHold, exfil.next());
        assert_eq!(ExfilState::BoardingHold, exfil.current_state);
        assert_eq!(ExfilState::TookOff, exfil.next());
        assert_eq!(ExfilState::TookOff, exfil.current_state);
        assert_eq!(ExfilState::Climbed, exfil.next());
        assert_eq!(ExfilState::Climbed, exfil.current_state);
        assert_eq!(ExfilState::Cruised, exfil.next());
        assert_eq!(ExfilState::Cruised, exfil.current_state);
        assert_eq!(ExfilState::ExitedAO, exfil.next());
        assert_eq!(ExfilState::ExitedAO, exfil.current_state);
        assert_eq!(ExfilState::Cooldown, exfil.next());
        assert_eq!(ExfilState::Cooldown, exfil.current_state);
        assert_eq!(ExfilState::Available, exfil.next());
        assert_eq!(ExfilState::Available, exfil.current_state);

        // then
        //assert!(app.world.get::<Health>(entity).is_some());
        //assert_eq!(app.world.get::<Health>(entity).unwrap().0, 90);
    }
}
