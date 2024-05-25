use bevy::render::view::visibility;
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, render::camera::ScalingMode};
use std::fmt;

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .init_resource::<MyWorldCoords>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup.before(load_cards), load_cards))
        .add_systems(Update, (my_cursor_system, spin))
        .run();
}

// example code
fn my_cursor_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut mycoords: ResMut<MyWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if keys.just_pressed(KeyCode::Space) {
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
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform = Transform::from_xyz(0., 0., 0.);
    camera.projection.scaling_mode = ScalingMode::AutoMax {
        max_width: 3800.,
        max_height: 2100.,
    };
    commands.spawn((camera, MainCamera));
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug)]
enum Value {
    Ace = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Component, Debug)]
struct GameCard {
    front_texture: Entity,
    value: i8,
    suit: Suit,
}

#[derive(Component, Debug)]
struct Card;
#[derive(Component, Debug)]
struct CardBack;

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

    let back_path = "cards/cardBack_blue2.png";
    let back_sprite: Handle<Image> = asset_server.load(back_path);

    for (x, suit) in suits.iter().enumerate() {
        for (val, num) in numbers.iter().enumerate() {
            let tform = Transform::from_xyz(val as f32 * 140., x as f32 * 190., 1.);

            let back_id = commands
                .spawn((
                    CardBack,
                    Card,
                    SpriteBundle {
                        sprite: Sprite { ..default() },
                        texture: back_sprite.clone(),
                        transform: tform,
                        global_transform: GlobalTransform::default(),
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                ))
                .id();

            let path = format!("cards/card{suit}{num}.png");
            let img = asset_server.load(path);
            let card: GameCard = GameCard {
                front_texture: back_id,
                value: val as i8,
                suit: *suit,
            };
            let front_id = commands
                .spawn((
                    card,
                    Card,
                    SpriteBundle {
                        sprite: Sprite { ..default() },
                        texture: img.clone(),
                        transform: Transform::from_xyz(val as f32 * 140., x as f32 * 190., 1.),
                        global_transform: GlobalTransform::default(),
                        ..default()
                    },
                ))
                .id();

            commands.entity(front_id).add_child(back_id);
        }
    }
}

use std::f32::consts::PI;
fn spin(
    windows: Query<&Window>,
    mut query: Query<(Entity, &GameCard, &mut Transform, &Children)>,
    mut children_query: Query<(Entity, &mut Visibility)>,
) {
    for (entity, card, mut t, children) in &mut query {
        t.rotate_local_y(PI / 180.);

        let (x, y, z) = t.rotation.to_euler(EulerRot::XYZ);
        let angle = y * 180. / PI;
        if card.suit == Suit::Diamonds && card.value == 2 {
            if (y * 180. / PI) >= 45.0 || (y * 180. / PI) <= -45.0 {
                for child in children {
                    if let Ok((child_entity, mut vis)) = children_query.get_mut(*child) {
                        *vis = Visibility::Visible;
                    }
                    if let Ok((child_entity, mut vis)) = children_query.get_mut(entity) {
                        *vis = Visibility::Hidden;
                    }
                }
            } else {
                for child in children {
                    if let Ok((child_entity, mut vis)) = children_query.get_mut(*child) {
                        *vis = Visibility::Hidden;
                    }
                    if let Ok((child_entity, mut vis)) = children_query.get_mut(entity) {
                        *vis = Visibility::Visible;
                    }
                }
            }
        }
    }
}
