use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SIZE: f32 = 64.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const ENEMIES_COUNT: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SPAWN_INTERVAL: f32 = 5.0;
pub const STAR_COUNT: u32 = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_SPAWN_INTERVAL: f32 = 1.0;


#[derive(Component)]
pub struct Player {

}

#[derive(Component)]
pub struct Enemy {
    direction: Vec2
}


#[derive(Component)]
pub struct Star {}

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32
}


#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_INTERVAL, TimerMode::Repeating)
        }
    }
}


#[derive(Resource)]
pub struct SpawnEmemyTimer {
    pub timer: Timer
}

impl Default for SpawnEmemyTimer {
    fn default() -> Self {
        SpawnEmemyTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_INTERVAL, TimerMode::Repeating)
        }
    }
}


#[derive(Event)]
pub struct GameOver {
    pub score: u32
}


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .init_resource::<Score>()
    .init_resource::<StarSpawnTimer>()
    .init_resource::<SpawnEmemyTimer>()
    .add_event::<GameOver>()
    .add_systems(Startup, (spawn_camera, spawn_players, spawn_enemies, spawn_stars).chain())
    .add_systems(Update, player_movement)
    .add_systems(Update, constrain_player)
    .add_systems(Update, enemy_movement)
    .add_systems(Update, update_enemy_movement)
    .add_systems(Update, enemy_player_collision)
    .add_systems(Update, collect_star)
    .add_systems(Update, update_score)
    .add_systems(Update, update_spawn_star_timer)
    .add_systems(Update, spawn_star_over_time)
    .add_systems(Update, exit_game)
    .add_systems(Update, update_enemy_spawn_timer)
    .add_systems(Update, spawn_enemy_over_time)
    .add_systems(Update, handle_game_over)
    .run();   
}


fn spawn_players(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },

        Player{}
    ));
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
        ..default()
    });
}





fn player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    key_input: Res<ButtonInput<KeyCode>>
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if key_input.pressed(KeyCode::ArrowUp) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if key_input.pressed(KeyCode::ArrowDown) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if key_input.pressed(KeyCode::ArrowLeft) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if key_input.pressed(KeyCode::ArrowRight) {
            direction += Vec3::new(1.0,0.0,0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        player.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
        
}

fn constrain_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    window: Query<&Window, With<PrimaryWindow>>
) {
    let window = window.get_single().unwrap();

    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let player_half = PLAYER_SIZE/2.0;

        let x_min = 0.0+player_half;
        let x_max = window.width() - player_half;
        let y_min = 0.0+player_half;
        let y_max = window.height() - player_half;

        let mut trans = player_transform.translation;

        if trans.x > x_max {
            trans.x = x_max;
        } else if trans.x < x_min {
            trans.x = x_min;
        }

        if trans.y > y_max {
            trans.y = y_max;
        } else if trans.y < y_min {
            trans.y = y_min;
        }

        player_transform.translation = trans;
    }
}


fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..ENEMIES_COUNT {
        let x_fact: f32 = random();
        let y_fact: f32 = random();

        let x = ENEMY_SIZE/2.0 + (window.width() - ENEMY_SIZE)*x_fact;
        let y = ENEMY_SIZE/2.0 + (window.height() - ENEMY_SIZE)*y_fact;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: assets_server.load("sprites/ball_red_large.png"),
                ..default()
            },

            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>())
            }
        ));
    }
}

fn enemy_movement(
    mut enemies_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
) {
    for (mut transform, enemy) in enemies_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

fn update_enemy_movement(
    mut enemies_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // audio: Res<AudioSource>,
    // asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    let enemy_half = ENEMY_SIZE/2.0;



    for (transform, mut enemy) in enemies_query.iter_mut() {
        let translation = transform.translation;
        let mut enemy_direction = enemy.direction;
        
        if translation.x > (window.width()-enemy_half) || translation.x < enemy_half {
            enemy_direction.x *= -1.0;  
            //play_enemy_impact_audio(audio, asset_server)
        }

        if translation.y > (window.height()-enemy_half) || translation.y < enemy_half {
            enemy_direction.y *= -1.0;    
            //play_enemy_impact_audio(audio, asset_server)
        }

        enemy.direction = enemy_direction;
    }
}

// fn play_enemy_impact_audio(
//     audio: Res<SpatialAudioSink>,
//     asset_server: Res<AssetServer>
// ) {
//     let impact_sound = match random::<f32>() > 0.5 {
//         true => asset_server.load("audio/pluck_001.ogg"),
//         false => asset_server.load("audio/pluck_002.ogg")
//     };

    
// }



fn enemy_player_collision(
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemies_query: Query<&Transform, With<Enemy>>,
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    score: Res<Score>
) {
    if let Ok((entity, player_transform)) = player_query.get_single_mut() {
        
        for enemy_transform in enemies_query.iter() {
            let distance = player_transform.translation.distance(enemy_transform.translation);

            if distance <= PLAYER_SIZE/2.0 + ENEMY_SIZE/2.0 {
                commands.entity(entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}



fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
    let mut rng = thread_rng();

    for _ in 0..STAR_COUNT {
        let x_fact = rng.gen::<f32>();
        let y_fact = rng.gen::<f32>();

        let x = STAR_SIZE/2.0 + (window.width() - STAR_SIZE)*x_fact;
        let y = STAR_SIZE/2.0 + (window.height() - STAR_SIZE)*y_fact;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star{}
        ));
    }
}


fn collect_star(
    player_query: Query<&Transform, With<Player>>,
    stars_query: Query<(Entity, &Transform), With<Star>>,
    mut commands: Commands,
    mut score: ResMut<Score>
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (entity, star_transform) in stars_query.iter()  {
            
            let distance = player_transform.translation.distance(star_transform.translation);
            if distance <= PLAYER_SIZE/2.0 + STAR_SIZE/2.0 {

                // vanish star
                commands.entity(entity).despawn();
                
                // collect point
                score.value += 1;
                //add_star_score()
            }
        }
    }
}


fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score is {}", score.value);
    }
}


fn update_spawn_star_timer(
    mut timer_res: ResMut<StarSpawnTimer>,
    time: Res<Time>
) {
    timer_res.timer.tick(time.delta());
}


fn spawn_star_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let x_fact = random::<f32>();
        let y_fact = random::<f32>();

        let x = STAR_SIZE/2.0 + (window.width() - STAR_SIZE)*x_fact;
        let y = STAR_SIZE/2.0 + (window.height() - STAR_SIZE)*y_fact;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },

            Star {}
        ));
    }
}


fn exit_game(
    input_ref: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<AppExit>
) {
    if input_ref.pressed(KeyCode::Escape) {
        event_writer.send(AppExit);
    }
}



fn update_enemy_spawn_timer(
    mut timer_ref: ResMut<SpawnEmemyTimer>,
    time: Res<Time>
) {
    timer_ref.timer.tick(time.delta());
}

fn spawn_enemy_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    enemy_spawn_timer: Res<SpawnEmemyTimer>,
    assets_server: Res<AssetServer>
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        
        let x_fact: f32 = random();
        let y_fact: f32 = random();

        let x = ENEMY_SIZE/2.0 + (window.width() - ENEMY_SIZE)*x_fact;
        let y = ENEMY_SIZE/2.0 + (window.height() - ENEMY_SIZE)*y_fact;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: assets_server.load("sprites/ball_red_large.png"),
                ..default()
            },

            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>())
            }
        ));
    }
}



fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut app_exit_event_writer: EventWriter<AppExit>
) {
    for (_, game_over) in game_over_event_reader.read().enumerate() {
        println!("Game Over! Score: {}", game_over.score);
        app_exit_event_writer.send(AppExit);
    }
}