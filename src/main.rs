use bevy::prelude::*;
use rand::prelude::*;

const UNIT_WIDTH: u32 = 40;
const UNIT_HEIGHT: u32 = 40;

const X_LENGTH: u32 = 10;
const Y_LENGTH: u32 = 18;

const SCREEN_WIDTH: u32 = UNIT_WIDTH * X_LENGTH;
const SCREEN_HEIGHT: u32 = UNIT_HEIGHT * Y_LENGTH;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component, Clone, PartialEq, Eq)]
struct BlockPatterns(Vec<Vec<(i32, i32)>>);

struct NewBlockEvent;
struct GameTimer(Timer);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Tetris".to_string(),
            width: SCREEN_WIDTH as f32,
            height: SCREEN_HEIGHT as f32,
            ..Default::default()
        })
        .insert_resource(BlockPatterns(vec![
            vec![(0, 0), (0, -1), (0, 1), (0, 2)],  // I
            vec![(0, 0), (0, -1), (0, 1), (-1, 1)], // L
            vec![(0, 0), (0, -1), (0, 1), (1, 1)],  // 逆L
            vec![(0, 0), (0, -1), (1, 0), (1, 1)],  // Z
            vec![(0, 0), (1, 0), (0, 1), (1, -1)],  // 逆Z
            vec![(0, 0), (0, 1), (1, 0), (1, 1)],   // 四角
            vec![(0, 0), (-1, 0), (1, 0), (0, 1)],  // T
        ]))
        .insert_resource(GameTimer(Timer::new(
            std::time::Duration::from_millis(400),
            true,
        )))
        .add_event::<NewBlockEvent>()
        .add_startup_system(setup_camera)
        .add_startup_system(setup)
        .add_system(position_transform)
        .add_system(spawn_block)
        .add_system(game_timer)
        .add_system(block_fall)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

//See to URL 'https://bevyengine.org/learn/book/migration-guides/0.4-0.5/#simplified-events'
fn setup(mut new_block_events: EventWriter<NewBlockEvent>) {
    new_block_events.send(NewBlockEvent);
}

fn position_transform(mut position_query: Query<(&Position, &mut Transform, &mut Sprite)>) {
    let origin_x = UNIT_WIDTH as i32 / 2 - SCREEN_WIDTH as i32 / 2;
    let origin_y = UNIT_HEIGHT as i32 / 2 - SCREEN_HEIGHT as i32 / 2;
    position_query
        .iter_mut()
        .for_each(|(pos, mut transform, mut sprite)| {
            transform.translation = Vec3::new(
                (origin_x + pos.x as i32 * UNIT_WIDTH as i32) as f32,
                (origin_y + pos.y as i32 * UNIT_HEIGHT as i32) as f32,
                0.0,
            );
            sprite.custom_size = Some(Vec2::new(UNIT_WIDTH as f32, UNIT_HEIGHT as f32))
        });
}

fn spawn_block_element(commands: &mut Commands, color: Color, position: Position) {
    //See to URL 'https://bevyengine.org/learn/book/migration-guides/0.5-0.6/#spritebundle-and-sprite'
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(position);
}

fn next_color() -> Color {
    let colors = vec![
        Color::rgb_u8(64, 230, 100),
        Color::rgb_u8(220, 64, 90),
        Color::rgb_u8(70, 150, 210),
        Color::rgb_u8(220, 230, 70),
        Color::rgb_u8(35, 220, 241),
        Color::rgb_u8(240, 140, 70),
    ];
    let mut rng = rand::thread_rng();
    let mut color_index: usize = rng.gen();
    color_index %= colors.len();
    colors[color_index].clone()
}

fn next_block(block_patterns: &Vec<Vec<(i32, i32)>>) -> Vec<(i32, i32)> {
    let mut rng = rand::thread_rng();
    let mut pattern_index: usize = rng.gen();
    pattern_index %= block_patterns.len();

    block_patterns[pattern_index].clone()
}

fn spawn_block(
    mut commands: Commands,
    block_patterns: Res<BlockPatterns>,
    //See to URL 'https://bevyengine.org/learn/book/migration-guides/0.4-0.5/#simplified-events'
    mut new_block_events_reader: EventReader<NewBlockEvent>,
) {
    if new_block_events_reader.iter().next().is_none() {
        return;
    }

    let new_block = next_block(&block_patterns.0);
    let new_color = next_color();

    // ブロックの初期位置
    let initial_x = X_LENGTH / 2;
    let initial_y = Y_LENGTH - 4;

    new_block.iter().for_each(|(r_x, r_y)| {
        spawn_block_element(
            &mut commands,
            new_color.clone(),
            Position {
                x: (initial_x as i32 + r_x),
                y: (initial_y as i32 + r_y),
            },
        );
    });
}

fn game_timer(time: Res<Time>, mut timer: ResMut<GameTimer>) {
    //See to URL 'https://bevyengine.org/learn/book/migration-guides/0.4-0.5/#timer-now-uses-duration'
    timer.0.tick(time.delta());
}

fn block_fall(timer: ResMut<GameTimer>, mut block_query: Query<(Entity, &mut Position)>) {
    if !timer.0.finished() {
        return;
    }

    block_query.iter_mut().for_each(|(_, mut pos)| {
        pos.y -= 1;
    });
}