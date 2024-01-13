use bevy::{
    prelude::*,
    window::PrimaryWindow
};

use crate::utils::app_state::AppState;


pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), spawn_star)
            .add_systems(OnExit(AppState::Game), despawn_star);
    }
}

#[derive(Component, Default)]
struct Star;

fn spawn_star(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    println!("Spawning star");
    if let Ok(window) = window_query.get_single() {
        let star_transform = Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0)
            .with_scale(Vec3::splat(0.25));
        commands.spawn((
            SpriteBundle {
                transform: star_transform,
                texture: asset_server.load("sprites/planets/planet-red-1.png"),
                ..default()
            },
            Star
        ));
    }
}

fn despawn_star(
    mut commands: Commands,
    star_query: Query<Entity, With<Star>>,
) {
    println!("Despawning star");
    for star in star_query.iter() {
        commands.entity(star).despawn();
    }
}