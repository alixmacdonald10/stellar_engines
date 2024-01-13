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

pub fn set_game_state(
    mut app_state: ResMut<NextState<AppState>>,
) {
    app_state.set(AppState::MainMenu);
    println!("Defaulting to  AppState::MainMenu");
}


pub fn transition_app_state(
    input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    match app_state.get() {
        AppState::MainMenu => {
            if input.just_pressed(KeyCode::G) {
                next_app_state.set(AppState::Game);
                println!("Entered AppState::Game");
            }
        },
        AppState::Game => {
            if input.just_pressed(KeyCode::M) {
                next_app_state.set(AppState::MainMenu);
                println!("Entered AppState::MainMenu");
            }
        },
        _ => (),
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
