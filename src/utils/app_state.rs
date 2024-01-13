use bevy::{
    prelude::*,
    app::AppExit,
};


#[derive(States, Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}


pub fn transition_to_game_state(
    input: Res<Input<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if input.just_pressed(KeyCode::G) {
        if app_state.0 != Some(AppState::Game) {
            app_state.set(AppState::Game);
            println!("Entered AppState::Game");
        }
    }
}

pub fn transition_to_main_menu_state(
    input: Res<Input<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if input.just_pressed(KeyCode::M) {
        if app_state.0 != Some(AppState::MainMenu) {
            app_state.set(AppState::MainMenu);
            println!("Entered AppState::MainMenu");
        }
    }
}


pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut exit_writer: ResMut<Events<AppExit>>
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit_writer.send(AppExit);
    }
}
