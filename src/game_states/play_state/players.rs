use crate::game_states::play_state::components;
use crate::tiles::animation::Animation;
use crate::tiles::tilemap::Tilemap;
use crate::tiles::tileset::{TileId, TilePosition, Tileset};
use crate::utils::asset_storage::AssetStorage;
use legion::entity::Entity;
use legion::world::World;
use piston::input::{Button, Key};
use std::collections::HashMap;
use tiled::PropertyValue;

pub struct Players {
    pub players: Vec<Entity>,
}

impl Players {
    pub fn new() -> Players {
        Players { players: vec![] }
    }

    pub fn create_player(
        &mut self,
        id: PlayerId,
        player_spawns: &HashMap<PlayerId, TilePosition>,
        asset_storage: &AssetStorage,
        tilemap: &Tilemap,
        world: &mut World,
    ) {
        let [x, y] = *player_spawns.get(&id).unwrap();
        let tileset = asset_storage.get_asset::<Tileset>(id.as_str());
        let tile_id = PlayerFaceDirection::Down.get_tile_id(&tileset).unwrap();

        let player = world
            .insert(
                (components::Layer(1), components::Player(id)),
                vec![(
                    components::ScreenPosition([x as f64, y as f64]),
                    components::PreviousScreenPosition([x as f64, y as f64]),
                    components::HitBox(tileset.hit_boxes[&tile_id]),
                    components::DefaultTileId(tile_id),
                    components::CurrentTileId(tile_id),
                    components::Tileset(tileset.clone()),
                    components::Speed(1.0),
                    components::MoveDirectionStack(vec![]),
                    Self::create_player_controls(id),
                    components::AnimationType::Ownd(
                        tileset
                            .animation_frames_holder
                            .get(&tile_id)
                            .cloned()
                            .map(|frames| {
                                Animation::builder(frames)
                                    .looping(true)
                                    .paused(false)
                                    .build()
                            })
                            .unwrap(),
                    ),
                )],
            )
            .first()
            .copied()
            .unwrap();

        self.players.push(player);
    }

    fn create_player_controls(player_id: PlayerId) -> components::Controls {
        let mut controls = HashMap::new();

        match player_id {
            PlayerId::Player1 => {
                controls.insert(
                    Button::Keyboard(Key::Left),
                    PlayerCommand::Movement(Direction::Left),
                );
                controls.insert(
                    Button::Keyboard(Key::Right),
                    PlayerCommand::Movement(Direction::Right),
                );
                controls.insert(
                    Button::Keyboard(Key::Up),
                    PlayerCommand::Movement(Direction::Up),
                );
                controls.insert(
                    Button::Keyboard(Key::Down),
                    PlayerCommand::Movement(Direction::Down),
                );
                controls.insert(Button::Keyboard(Key::RCtrl), PlayerCommand::Bomb);
            }
            PlayerId::Player2 => {
                controls.insert(
                    Button::Keyboard(Key::A),
                    PlayerCommand::Movement(Direction::Left),
                );
                controls.insert(
                    Button::Keyboard(Key::D),
                    PlayerCommand::Movement(Direction::Right),
                );
                controls.insert(
                    Button::Keyboard(Key::W),
                    PlayerCommand::Movement(Direction::Up),
                );
                controls.insert(
                    Button::Keyboard(Key::S),
                    PlayerCommand::Movement(Direction::Down),
                );
                controls.insert(Button::Keyboard(Key::LCtrl), PlayerCommand::Bomb);
            }
            PlayerId::Player3 => {}
            PlayerId::Player4 => {}
        }

        components::Controls(controls)
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum PlayerId {
    Player1,
    Player2,
    Player3,
    Player4,
}

impl PlayerId {
    pub fn as_str(&self) -> &'static str {
        self.into()
    }
}

impl From<&PlayerId> for &str {
    fn from(player_id: &PlayerId) -> Self {
        match player_id {
            PlayerId::Player1 => "player1",
            PlayerId::Player2 => "player2",
            PlayerId::Player3 => "player3",
            PlayerId::Player4 => "player4",
        }
    }
}

impl From<u32> for PlayerId {
    fn from(num: u32) -> Self {
        match num {
            1 => PlayerId::Player1,
            2 => PlayerId::Player2,
            3 => PlayerId::Player3,
            4 => PlayerId::Player4,
            _ => panic!(format!("Cannot create PlayerId from this number {}", num)),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum PlayerFaceDirection {
    Down,
    Up,
    Left,
    Right,
}

impl PlayerFaceDirection {
    pub fn get_tile_id(&self, tileset: &Tileset) -> Option<TileId> {
        tileset
            .properties
            .iter()
            .find(
                |(_tile_id, properties)| match properties.get("face_direction") {
                    Some(PropertyValue::StringValue(face_direction)) => {
                        self.as_str() == face_direction.as_str()
                    }
                    _ => false,
                },
            )
            .map(|(tile_id, _)| *tile_id)
    }

    pub fn as_str(&self) -> &str {
        self.into()
    }
}

impl From<&str> for PlayerFaceDirection {
    fn from(face_direction: &str) -> Self {
        match face_direction {
            "down" => PlayerFaceDirection::Down,
            "up" => PlayerFaceDirection::Up,
            "left" => PlayerFaceDirection::Left,
            "right" => PlayerFaceDirection::Right,
            _ => panic!(format!(
                "Cannot create PlayerFaceDirection from {}",
                face_direction
            )),
        }
    }
}

impl From<Direction> for PlayerFaceDirection {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => PlayerFaceDirection::Up,
            Direction::Down => PlayerFaceDirection::Down,
            Direction::Left => PlayerFaceDirection::Left,
            Direction::Right => PlayerFaceDirection::Right,
        }
    }
}

impl From<&PlayerFaceDirection> for &str {
    fn from(player_face_direction: &PlayerFaceDirection) -> Self {
        match player_face_direction {
            PlayerFaceDirection::Down => "down",
            PlayerFaceDirection::Up => "up",
            PlayerFaceDirection::Left => "left",
            PlayerFaceDirection::Right => "right",
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum PlayerCommand {
    Movement(Direction),
    Bomb,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
