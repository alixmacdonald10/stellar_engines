mod star;
mod planets;

use bevy::prelude::*;

use star::StarPlugin;
use planets::PlanetsPlugin;

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(StarPlugin)
            .add_plugins(PlanetsPlugin);
    }
}
