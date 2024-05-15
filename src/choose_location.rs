use crate::AppState::DeployScreen;
use crate::DeployScreen::*;
use crate::{AppState, ButtonTargetState};
use bevy::prelude::*;

// TODO: four simple buttons as standins for the selectable maps
// TODO: click on a button saves the selection somehow and advances to missions
//
pub struct ChooseLocationScreenPlugin;

impl Plugin for ChooseLocationScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(DeployScreen(ChooseLocation)),
            start_choose_location_screen,
        )
        .add_systems(
            Update,
            (update_choose_location_screen).run_if(in_state(DeployScreen(ChooseLocation))),
        )
        .add_systems(
            OnExit(DeployScreen(ChooseLocation)),
            bye_choose_location_screen,
        );
    }
}

// TODO: hardcoding the levels for now, maybe later make this more dynamic
#[derive(Resource)]
struct ChooseLocationMenuData {
    vondel_button_entity: Entity,
    ashika_island_button_entity: Entity,
    al_mazrah_objectives_button_entity: Entity,
    building_21_button_entity: Entity,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn start_choose_location_screen(mut commands: Commands) {
    println!("start testtest");
    todo!()
}

fn update_choose_location_screen(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonTargetState),
        (Changed<Interaction>, With<Button>),
    >,
) {
    println!("update testtest");
    todo!()
}

fn bye_choose_location_screen(mut commands: Commands, menu_data: Res<ChooseLocationMenuData>) {
    todo!()
}
