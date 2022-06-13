use bevy::prelude::*;
use player::PlayerPlugin;
mod components;
mod player;
const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);

const SPRITE_SCALE: f32 = 0.5;

// region
pub struct WinSize {
  pub w: f32,
  pub h: f32,
}

pub struct GameTextures {
  player: Handle<Image>,
}

// endregion

fn main() {
  App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource(WindowDescriptor {
      title: "Rust Invaders!".to_string(),
      width: 598.0,
      height: 676.0,
      ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(PlayerPlugin)
    .add_startup_system(setup_system)
    .run();
}

fn setup_system(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut windws: ResMut<Windows>,
) {
  // camera
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());

  // capture window size
  let window = windws.get_primary_mut().unwrap();
  let (win_w, win_h) = (window.width(), window.height());

  //  window position
  window.set_position(IVec2::new(500, 100));

  // window size
  let win_size = WinSize { w: win_w, h: win_h };
  commands.insert_resource(win_size);

  // add GameTextures resource
  let game_textures = GameTextures {
    player: asset_server.load(PLAYER_SPRITE),
  };
  commands.insert_resource(game_textures);
}
