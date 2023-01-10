use bevy::{ecs::query::QueryEntityError, prelude::*};

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load("mani-idle-run.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(8.0)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(1.0 / 12.0, TimerMode::Repeating)),
    ));
}

pub fn animate(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        if timer.tick(time.delta()).just_finished() {
            if let Some(texture_atlas) = texture_atlases.get(texture_atlas_handle) {
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            }
        }
    }
}
