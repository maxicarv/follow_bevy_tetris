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

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Tetris".to_string(),
            width: SCREEN_WIDTH as f32,
            height: SCREEN_HEIGHT as f32,
            ..Default::default()
        })
        .add_startup_system(setup_camera)
        .add_system(position_transform)
        .add_system(spawn_block_element)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
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

fn spawn_block_element(mut commands: Commands) {
    //See to URL 'https://bevyengine.org/learn/book/migration-guides/0.5-0.6/#spritebundle-and-sprite'
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

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: colors[color_index],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Position { x: 1, y: 5 });
}
