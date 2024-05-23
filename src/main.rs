use bevy::window::PrimaryWindow;
use bevy::{
    prelude::*,
    render::camera::{self, ScalingMode},
    transform::commands,
    utils::dbg,
    window::WindowResolution,
};
use std::{f32::consts::PI, fmt};

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

fn my_cursor_system(
    mut mycoords: ResMut<MyWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
        eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    }
}
fn main() {
    App::new()
        .init_resource::<MyWorldCoords>()
        .add_plugins(DefaultPlugins)
        //.add_systems(Startup, (setup.before(render_cards), render_cards))
        .add_systems(Startup, (setup.before(load_cards), load_cards))
        .add_systems(Update, move_camera)
        //.add_systems(Update, (spin, move_camera, my_cursor_system))
        .run();
}

fn setup(mut commands: Commands, windows: Query<&Window>, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.transform = Transform::from_xyz(0., 0., 0.);
    camera.projection.scaling_mode = ScalingMode::AutoMax {
        max_width: 3800.,
        max_height: 2100.,
    };
    commands.spawn((camera, MainCamera));
}

#[derive(Clone, Copy, Debug)]
enum Suit {
    Hearts,
    Clubs,
    Spades,
    Diamonds,
}
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Clubs => write!(f, "Clubs"),
            Suit::Hearts => write!(f, "Hearts"),
            Suit::Diamonds => write!(f, "Diamonds"),
            Suit::Spades => write!(f, "Spades"),
        }
    }
}

#[derive(Component, Debug)]
struct GameCard {
    front_texture: i32,
    value: i8,
    suit: Suit,
}

#[derive(Component)]
struct Deck {
    cards: Vec<GameCard>,
    size: usize,
}
fn load_cards(mut commands: Commands, windows: Query<&Window>, asset_server: Res<AssetServer>) {
    let window = windows.single();

    let ceiling = window.height() / 2.;
    let _ground = -window.height() / 2.;

    let _wall_left: f32 = -window.width() / 2.;
    let wall_right: f32 = window.width() / 2.;

    let suits = Vec::from([Suit::Hearts, Suit::Clubs, Suit::Spades, Suit::Diamonds]);
    let numbers = vec![
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
    ];

    let mut cards: Vec<GameCard> = Vec::with_capacity(suits.len() * numbers.len());
    for (x, suit) in suits.iter().enumerate() {
        for (val, num) in numbers.iter().enumerate() {
            let mut tform = Transform::from_xyz(
                (val as f32 * 140.) + (140. / 2.) - wall_right,
                (x as f32 * 190.) + (190. / 2.) - ceiling,
                1.,
            );
            let path = format!("cards/card{suit}{num}.png");
            let img = asset_server.load(path);
            let card: GameCard = GameCard {
                front_texture: 0,
                value: val as i8,
                suit: *suit,
            };
            commands.spawn((
                card,
                SpriteBundle {
                    sprite: Sprite {
                        anchor: bevy::sprite::Anchor::TopLeft,
                        ..default()
                    },
                    texture: img.clone(),
                    transform: tform,
                    global_transform: GlobalTransform::default(),
                    ..default()
                },
            ));
        }
    }
}

fn spin(windows: Query<&Window>, mut query: Query<(Entity, &GameCard, &mut Transform)>) {
    for (entity, card, mut e) in &mut query {
        e.rotate_local_y(PI / 180.);
    }
}

fn move_camera(
    mut camera_query: Query<(&Camera, &GlobalTransform, &mut Transform)>,
    windows: Query<&Window>,
) {
    let (camera, camera_global_transform, mut camera_transform) = camera_query.single();
    let Some(point) = camera.world_to_viewport(
        camera_global_transform,
        Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        },
    ) else {
        return;
    };

    camera_transform = &camera_transform.with_translation(Vec3 {
        x: point.x,
        y: point.y,
        z: 0.0,
    });
}
