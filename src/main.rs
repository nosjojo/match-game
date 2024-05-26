use bevy::window::PrimaryWindow;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_mod_picking::debug::DebugPickingMode;
use bevy_mod_picking::events::{Click, Pointer};
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle};
use std::f32::consts::PI;
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
        .add_plugins((DefaultPlugins, DefaultPickingPlugins))
        .insert_resource(DebugPickingMode::Normal)
        .add_systems(Startup, (setup.before(load_cards), load_cards))
        .add_systems(Update, (flip_card, my_cursor_system))
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
        // assuming there is exact000000ly one main camera entity, so Query::single() is OK
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
    camera.transform = Transform::from_xyz(910., 380., 0.);
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

// #[derive(Clone, Copy, Debug)]
// enum Value {
//     Ace = 1,
//     Two,
//     Three,
//     Four,
//     Five,
//     Six,
//     Seven,
//     Eight,
//     Nine,
//     Ten,
//     Jack,
//     Queen,
//     King,
// }

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

fn load_cards(mut commands: Commands, asset_server: Res<AssetServer>) {
    let suits = Vec::from([Suit::Hearts, Suit::Clubs, Suit::Spades, Suit::Diamonds]);
    let numbers = vec![
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
    ];

    let back_path = "cards/cardBack_blue2.png";
    let back_sprite: Handle<Image> = asset_server.load(back_path);

    for (x, suit) in suits.iter().enumerate() {
        for (val, num) in numbers.iter().enumerate() {
            let tform = Transform::from_xyz(val as f32 * 140., x as f32 * 190., 1.);
            let player_back = AnimationPlayer::default();
            let player_front = AnimationPlayer::default();
            let back_id = commands
                .spawn((
                    PickableBundle::default(),
                    player_back,
                    CardBack,
                    Card,
                    SpriteBundle {
                        sprite: Sprite { ..default() },
                        texture: back_sprite.clone(),

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
                    PickableBundle::default(),
                    player_front,
                    card,
                    Card,
                    Name::new("card"),
                    SpriteBundle {
                        sprite: Sprite { ..default() },
                        texture: img.clone(),
                        transform: tform,
                        global_transform: GlobalTransform::default(),
                        ..default()
                    },
                ))
                .id();

            commands.entity(front_id).add_child(back_id);
        }
    }
}

fn animation_card_flip_half() -> AnimationClip {
    let mut animation = AnimationClip::default();
    let card_name = Name::new("card");
    animation.add_curve_to_path(
        EntityPath {
            parts: vec![card_name],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0, 1.0],
            keyframes: Keyframes::Rotation(vec![
                Quat::IDENTITY,
                Quat::from_axis_angle(Vec3::Y, PI / 2.),
            ]),
            interpolation: Interpolation::Linear,
        },
    );
    return animation;
}

fn flip_card(
    mut query: Query<&mut AnimationPlayer>,
    mut clicked: EventReader<Pointer<Click>>,
    mut animations: ResMut<Assets<AnimationClip>>,
) {
    for item in clicked.read() {
        if let Ok(mut player) = query.get_mut(item.target) {
            player.play(animations.add(animation_card_flip_half()));
            dbg!(item.target);
        }
    }
}
