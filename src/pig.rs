use bevy::prelude::*;
use crate::{Money, Player};
use rand::Rng;

pub struct PigPlugin;
impl Plugin for PigPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, spawn_pig_parent)
      .add_systems(Update, (spawn_pig, pig_random_movement, pig_lifetime).chain())
      .register_type::<Pig>();

  }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Pig {
  pub lifetime: Timer,
  pub movement_timer: Timer,
}

#[derive(Component)]
pub struct PigParent;

fn spawn_pig_parent(mut commands: Commands) {
  commands.spawn((
    SpatialBundle::default(),
    PigParent,
    Name::new("Pig Parent")
  ));
}

fn spawn_pig(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
    parent: Query<Entity, With<PigParent>>,
  ) {
      if !input.just_pressed(KeyCode::Space) {
        return;
      }
      
      let player_transform = player.single();
      let parent = parent.single();
  
      if money.0 >= 10.0 {
        money.0 -= 10.0;
        info!("Spent $10 dollars on a pig, remaining money is {}", money.0);
  
        let texture = asset_server.load("pig.png"); 
  
        commands.entity(parent).with_children(|commands| {
          commands.spawn((
            SpriteBundle {
              texture,
              transform: *player_transform,
              ..default()
            },
            Pig {
              lifetime: Timer::from_seconds(5.0, TimerMode::Once),
              movement_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            },
            Name::new("Pig"),
          ));
        });
      }
  }
  
fn pig_lifetime(
  mut commands: Commands,
  time: Res<Time>,
  mut pigs: Query<(Entity, &mut Pig)>,
  parent: Query<Entity, With<PigParent>>,
  mut money: ResMut<Money>,
) {
    let parent = parent.single();

    for (pig_entity, mut pig) in &mut pigs {
      pig.lifetime.tick(time.delta());

      if pig.lifetime.finished() {
        money.0 += 15.0;

        commands.entity(parent).remove_children(&[pig_entity]);
        commands.entity(pig_entity).despawn();

        info!("Pig sold for 15.0, current money is {}", money.0);
      }
    }
}

fn pig_random_movement(
  time: Res<Time>,
  mut pigs: Query<(&mut Transform, &mut Pig)>,
) {
    let mut rng = rand::thread_rng();
    for (mut pig_transform, mut pig) in &mut pigs {
      pig.movement_timer.tick(time.delta());

      if pig.movement_timer.finished() {
      pig_transform.translation.x += rng.gen_range(-5.0..5.0);
      pig_transform.translation.y += rng.gen_range(-5.0..5.0);

      pig.movement_timer.reset();
      }
    }
}

