use bevy::prelude::*;
use bevy::color::Color;

use std::collections::HashMap;
use rand::{Rng};

mod ascii_renderer;
use ascii_renderer::{AsciiRendererPlugin, spawn_ascii_entity};

// Constants
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

const TILE_SIZE: f32 = 20.0;

const MAP_WIDTH: usize = 40;
const MAP_HEIGHT: usize = 30;

// Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: char,
    color: Color,
}

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Floor;

#[derive(Resource)]
struct Map {
    tiles: HashMap<(i32, i32), TileType>,
}

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

// Systems
fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);
}

fn spawn_map(
    mut commands: Commands,
    mut map: ResMut<Map>,
    asset_server: Res<AssetServer>
) {
    let mut rng = rand::rng();

    // Initialize all tiles as walls
    for x in 0..MAP_WIDTH as i32 {
        for y in 0..MAP_HEIGHT as i32 {
            map.tiles.insert((x, y), TileType::Wall);
        }
    }

    // Create a simple room in the center
    let room_width = 20;
    let room_height = 15;
    let start_x = (MAP_WIDTH as i32 - room_width) / 2;
    let start_y = (MAP_HEIGHT as i32 - room_height) / 2;

    for x in start_x..(start_x + room_width) {
        for y in start_y..(start_y + room_height) {
            map.tiles.insert((x, y), TileType::Floor);
        }
    }

    // Add some random walls inside the room
    for _ in 0..10 {
        let x = rng.random_range(start_x + 1..(start_x + room_width - 1));
        let y = rng.random_range(start_y + 1..(start_y + room_height - 1));
        map.tiles.insert((x, y), TileType::Wall);
    }

    // Render the map
    for (pos, tile_type) in map.tiles.iter() {
        let (x, y) = *pos;

        match tile_type {
            TileType::Wall => {
                spawn_ascii_entity(
                    &mut commands,
                    &asset_server,
                    Position { x, y },
                    Renderable {
                        glyph: '#',
                        color: Color::srgb(0.5, 0.5, 0.5), // Gray
                    },
                    0.0,
                );
            }
            TileType::Floor => {
                spawn_ascii_entity(
                    &mut commands,
                    &asset_server,
                    Position { x, y },
                    Renderable {
                        glyph: '.',
                        color: Color::srgb(0.0, 0.5, 0.0), // Dark green
                    },
                    0.0,
                );
            }
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    map: Res<Map>,
    asset_server: Res<AssetServer>
) {
    // Find a valid floor tile for the player
    let mut valid_positions = Vec::new();
    for (pos, tile_type) in map.tiles.iter() {
        if *tile_type == TileType::Floor {
            valid_positions.push(*pos);
        }
    }

    // Choose a random position
    let mut rng =   rand::rng();
    let player_pos = valid_positions[rng.random_range(0..valid_positions.len())];
    let (x, y) = player_pos;

    let player_id = spawn_ascii_entity(
        &mut commands,
        &asset_server,
        Position { x, y },
        Renderable {
            glyph: '@',
            color: Color::srgb(1.0, 1.0, 0.0), // Yellow
        },
        1.0,
    );

    commands.entity(player_id).insert(Player);
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    map: Res<Map>,
    mut query: Query<(&mut Position, &mut Transform), With<Player>>,
) {
    if let Ok((mut pos, mut transform)) = query.get_single_mut() {
        let mut dx = 0;
        let mut dy = 0;

        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            dy = 1;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            dy = -1;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            dx = -1;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            dx = 1;
        }

        let new_x = pos.x + dx;
        let new_y = pos.y + dy;

        // Check if the new position is valid (not a wall and within bounds)
        if let Some(tile_type) = map.tiles.get(&(new_x, new_y)) {
            if *tile_type == TileType::Floor {
                pos.x = new_x;
                pos.y = new_y;
                transform.translation.x = new_x as f32 * TILE_SIZE - (MAP_WIDTH as f32 * TILE_SIZE / 2.0) + (TILE_SIZE / 2.0);
                transform.translation.y = new_y as f32 * TILE_SIZE - (MAP_HEIGHT as f32 * TILE_SIZE / 2.0) + (TILE_SIZE / 2.0);
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "ASCII Roguelike".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AsciiRendererPlugin)
        .insert_resource(Map {
            tiles: HashMap::new(),
        })
        .add_systems(Startup, (setup, spawn_map, spawn_player).chain())
        .add_systems(Update, player_movement)
        .run();
}
