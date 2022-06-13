use bevy::prelude::*;

use crate::{GameTextures, WinSize, PLAYER_SIZE, SPRITE_SCALE};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system);
  }
}

fn player_spawn_system(
  mut commands: Commands,
  game_textures: Res<GameTextures>,
  win_size: Res<WinSize>,
) {
  let bottom = -win_size.h / 2.;
  // add rectangle
  commands.spawn_bundle(SpriteBundle {
    texture: game_textures.player.clone(),

    transform: Transform {
      translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. + 2., 10.),
      scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
      ..Default::default()
    },
    ..Default::default()
  });
}
