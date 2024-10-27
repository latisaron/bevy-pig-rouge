use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use pig::PigPlugin;
use ui::GameUI;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy::input::common_conditions::input_toggle_active;


mod pig;
mod ui;

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
  pub speed: f32,
}

#[derive(Resource)]
pub struct Money(pub f32);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let mut camera = Camera2dBundle::default();

  camera.projection.scaling_mode = ScalingMode::AutoMin {
    min_width: 256.0,
    min_height: 144.0,
  };

  commands.spawn(camera);
  
  let texture = asset_server.load("character.png");

  commands.spawn((
    SpriteBundle {
      texture,
      ..default()
    },
    Player { speed: 100.0 },
    Name::new("Player"),
  ));
}

fn character_movement(
  mut characters: Query<(&mut Transform, &Player)>,
  input: Res<Input<KeyCode>>,
  time: Res<Time>,
) {
  for (mut transform, player) in &mut characters {
    let movement_amount = player.speed * time.delta_seconds();
    if input.pressed(KeyCode::W) {
      transform.translation.y += movement_amount;
    } else if input.pressed(KeyCode::S) {
      transform.translation.y -= movement_amount;
    } else if input.pressed(KeyCode::D) {
      transform.translation.x += movement_amount;
    } else if input.pressed(KeyCode::A) {
      transform.translation.x -= movement_amount;
    }
  }
}



fn main() {
    App::new()
      .add_plugins(DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: "Aron Rougelike".into(),
            resolution: (640.0, 480.0).into(),
            resizable: false,
            ..default()
          }),
          ..default()
        })
        .build(),
      )
      .insert_resource(Money(100.0))
      .add_plugins((PigPlugin, GameUI))
      .add_plugins(
        WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape))
      )
      .add_systems(Startup, setup)
      .add_systems(Update, character_movement)
      .run();
}
