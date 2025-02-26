use bevy::prelude::*;

use crate::{Position, Renderable, TILE_SIZE, MAP_WIDTH, MAP_HEIGHT};

pub struct AsciiRendererPlugin;

impl Plugin for AsciiRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_ascii_display);
    }
}

// This system updates the text display for entities with both Position and Renderable components
fn update_ascii_display(
    mut query: Query<(&Position, &Renderable, &mut Text, &mut TextColor, &mut Transform)>,
) {
    for (position, renderable, mut text, mut text_color, mut transform) in query.iter_mut() {
        // Update the text with the current glyph
        text.0 = renderable.glyph.to_string();
        text_color.0 = renderable.color;

        // Update position
        transform.translation.x = position.x as f32 * TILE_SIZE - (MAP_WIDTH as f32 * TILE_SIZE / 2.0) + (TILE_SIZE / 2.0);
        transform.translation.y = position.y as f32 * TILE_SIZE - (MAP_HEIGHT as f32 * TILE_SIZE / 2.0) + (TILE_SIZE / 2.0);
    }
}

// Helper function to spawn an ASCII entity
pub fn spawn_ascii_entity(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Position,
    renderable: Renderable,
    z_index: f32,
) -> Entity {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    let text_style = TextFont {
        font: font.clone(),
        font_size: 25.0,
        ..default()
    };

    commands.spawn((
        Text2d::new(renderable.glyph.to_string()),
        text_style,
        TextColor(renderable.color),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_xyz(
            position.x as f32 * TILE_SIZE - (MAP_WIDTH as f32 * TILE_SIZE / 2.0) + (TILE_SIZE / 2.0),
            position.y as f32 * TILE_SIZE - (MAP_HEIGHT as f32 * TILE_SIZE / 2.0) + (TILE_SIZE / 2.0),
            z_index,
        ),
        position,
        renderable,
    )).id()
}
