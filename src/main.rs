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
        .add_systems(Startup, load_cards)
        .add_systems(Update, (spin, move_camera,my_cursor_system))
        .run();
}

// fn setup(mut commands: Commands, windows: Query<&Window>, asset_server: Res<AssetServer>) {
//     commands.spawn(Camera2dBundle::default());
//     load_cards(commands, asset_server);
// }

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
    let mut camera = Camera2dBundle::default();

    let window = windows.single();

    let ceiling = window.height() / 2.;
    let ground = -window.height() / 2.;

    let wall_left: f32 = -window.width() / 2.;
    let wall_right: f32 = window.width() / 2.;

    camera.projection.scaling_mode = ScalingMode::AutoMax {
        max_width: 3800.,
        max_height: 2100.,
    };
    camera.transform = Transform::from_xyz(0., 0., 0.);

    commands.spawn((camera, MainCamera));

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
            let cid = commands.spawn((
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

    // for i in 0..card_count {
    //     let card: GameCard = GameCard {
    //         entity: {
    //             commands
    //                 .spawn(SpriteBundle {
    //                     sprite: Sprite {
    //                         anchor: bevy::sprite::Anchor::TopLeft,
    //                         ..default()
    //                     },
    //                     texture: card_back_img.clone(),
    //                     ..default()
    //                 })
    //                 .id()
    //         },
    //         position: Position { x: 0, y: 0 },
    //         front_texture: 0,
    //     };

    //     cards.push(card)
    // }
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
    dbg!(camera_transform);
    camera_transform = &camera_transform.with_translation(Vec3 {
        x: point.x,
        y: point.y,
        z: 0.0,
    });
    dbg!(point);
}
fn render_cards(windows: Query<&Window>, mut query: Query<(Entity, &GameCard, &mut Transform)>) {
    let window = windows.single();

    let ceiling = window.height() / 2.;
    let ground = -window.height() / 2.;

    let wall_left: f32 = -window.width() / 2.;
    let wall_right: f32 = window.width() / 2.;
    dbg!(ceiling);
    dbg!(ground);
    dbg!(wall_left);
    dbg!(wall_right);
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    for (entity, card, mut e) in &mut query {
        //dbg!(entity);
        //dbg!(card);
        let mut row: i32 = 0;
        match card.suit {
            Suit::Clubs => {
                row = 1;
            }
            Suit::Diamonds => {
                row = 2;
            }
            Suit::Hearts => {
                row = 3;
            }
            Suit::Spades => {
                row = 4;
            }
        }
        //*e = Transform::from_xyz((j as f32 * 140.)+(140./2.), (row as f32 * 190.)+(190./2.), 1.);
        //e.translation.x = wall_left + (j as f32 * 140.0);
        //e.translation.y = ceiling - (row as f32 * 190.0);
        dbg!(e.translation.x);
        dbg!(e.translation.y);
        i += 1;
        if i % 13 == 0 {
            j += 1;
        }
        dbg!(i);
        dbg!(j);
    }
}
