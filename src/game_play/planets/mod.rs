use bevy::{
    prelude::*,
    window::PrimaryWindow
};

use crate::utils::app_state::AppState;


pub struct PlanetsPlugin;

impl Plugin for PlanetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), spawn_planets)
            .add_systems(OnExit(AppState::Game), despawn_planets);
    }
}

#[derive(Component, Default)]
struct Planet;

fn spawn_planets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    println!("Spawning planets");
    if let Ok(window) = window_query.get_single() {
        let planet_transform = Transform::from_xyz(window.width() / 4.0, window.height() / 4.0, 0.0)
            .with_scale(Vec3::splat(0.05));
        commands.spawn((
            SpriteBundle {
                transform: planet_transform,
                texture: asset_server.load("sprites/planets/planet-earth-like-1.png"),
                ..default()
            },
            Planet
        ));
    }
}

fn despawn_planets(
    mut commands: Commands,
    star_query: Query<Entity, With<Planet>>,
) {
    println!("Despawning planets");
    for planet in star_query.iter() {
        commands.entity(planet).despawn();
    }
}