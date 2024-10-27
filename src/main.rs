use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use rand::Rng;


#[derive(Component)]
pub struct Player {
  pub speed: f32,
}

#[derive(Component)]
pub struct Pig {
  pub lifetime: Timer,
  pub movement_timer: Timer,
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

fn spawn_pig(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  input: Res<Input<KeyCode>>,
  mut money: ResMut<Money>,
  player: Query<&Transform, With<Player>>,
) {
    if !input.pressed(KeyCode::Space) {
      return;
    }
    
    let player_transform = player.single();

    if money.0 >= 10.0 {
      money.0 -= 10.0;
      info!("Spent $10 dollars on a pig, remaining money is {}", money.0);

      let texture = asset_server.load("pig.png");

      commands.spawn((
          SpriteBundle {
            texture,
            transform: *player_transform,
            ..default()
          },
          Pig {
            lifetime: Timer::from_seconds(2.0, TimerMode::Once),
            lifetime: Timer.from_seconds(1.0, TimerMode::Repeating),
          },
      ));
    }
}

fn pig_lifetime(
  mut commands: Commands,
  time: Res<Time>,
  mut pigs: Query<(Entity, &mut Pig)>,
  mut money: ResMut<Money>,
) {
  for (pig_entity, mut pig) in &mut pigs {
    pig.lifetime.tick(time.delta());

    if pig.lifetime.finished() {
      money.0 += 15.0;

      commands.entity(pig_entity).despawn();

      info!("Pig sold for 15.0, current money is {}", money.0);
    }
  }
}

fn pig_random_movement(
  mut pigs: Query<&mut Transform, &mut Pig>>,
) {
  let mut rng = rand::thread_rng();
  for mut pig_transform, pig in &mut pigs {
    pig.movement_timer.tick(time.delta());

    if pig.movement_timer.finished {
      pig_transform.translation.x += rng.gen_range(-5.0..5.0);
      pig_transform.translation.y += rng.gen_range(-5.0..5.0);

      pig.movement_timer.reset();
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
      .add_systems(Startup, setup)
      .add_systems(Update, (character_movement, spawn_pig, (pig_random_movement,pig_lifetime).chain()))
      .run();
}
