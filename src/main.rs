mod utils;
mod main_menu;
mod game_play;
mod game_over;

use bevy::prelude::*;

use crate::main_menu::MainMenuPlugin;
use crate::game_play::GamePlayPlugin;
use crate::game_over::GameOverPlugin;
use crate::utils::app_state::{
    AppState,
    transition_app_state,
    exit_game
};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlayPlugin)
        .add_plugins(GameOverPlugin)
        .add_systems(Update, transition_app_state)
        .add_systems(Update, exit_game)
        .run();
}
