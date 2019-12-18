/*
use crate::utils::Spritesheet;
use graphics::math::Vec2d;
use piston::input::Key;

const TEXTURE_FOLDER: &str = "assets/textures/player/";

pub enum PlayerTextureName {
    FaceDown,
    FaceUp,
    FaceLeft,
    FaceRight,
}

impl PlayerTextureName {
    pub fn as_str(&self) -> &str {
        match self {
            PlayerTextureName::FaceDown => "face_down",
            PlayerTextureName::FaceUp => "face_up",
            PlayerTextureName::FaceLeft => "face_left",
            PlayerTextureName::FaceRight => "face_right",
        }
    }
}

pub struct Player {
    pub player_id: PlayerId,
    pub spritesheet: Spritesheet,
    pub position: Vec2d,
    pub speed: Vec2d,
    pub move_direction_stack: Vec<MoveDirection>,
}

impl Player {
    pub fn new(player_id: PlayerId, pos: Vec2d) -> Player {
        let spritesheet = Spritesheet::new(
            TEXTURE_FOLDER,
            &(player_id.as_str().to_string() + ".xml"),
            PlayerTextureName::FaceDown.as_str(),
        );

        Player {
            player_id,
            spritesheet,
            position: pos,
            speed: [0.0; 2],
            move_direction_stack: Vec::new(),
        }
    }

    pub fn get_move_direction(&self, key: &Key) -> MoveDirection {
        match self.player_id {
            PlayerId::Player1 => match key {
                Key::Left => MoveDirection::Left,
                Key::Right => MoveDirection::Right,
                Key::Up => MoveDirection::Up,
                Key::Down => MoveDirection::Down,
                _ => MoveDirection::Standing,
            },
            PlayerId::Player2 => match key {
                Key::A => MoveDirection::Left,
                Key::D => MoveDirection::Right,
                Key::W => MoveDirection::Up,
                Key::S => MoveDirection::Down,
                _ => MoveDirection::Standing,
            },
            _ => MoveDirection::Standing,
        }
    }
}



#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PlayerId {
    Player1,
    Player2,
    Player3,
    Player4,
}

impl PlayerId {
    pub fn as_str(&self) -> &str {
        match self {
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
            0 => PlayerId::Player1,
            1 => PlayerId::Player2,
            2 => PlayerId::Player3,
            3 => PlayerId::Player4,
            _ => panic!(format!("Cannot create a PlayerId from the number {}", num)),
        }
    }
}
*/

use engine::asset::{TileId, Tileset};
use engine::game_state::EventHandler;
use engine::tile::TileUuid;
use piston::input::*;
use std::collections::HashMap;

pub type PlayerControlsMap = HashMap<Button, PlayerAction>;

pub struct Player {
    pub id: PlayerId,
    pub tile_uuid: TileUuid,
    pub face_directions_to_tile_ids: HashMap<PlayerFaceDirection, TileId>,
    move_direction_stack: Vec<MoveDirection>,
    controls_map: PlayerControlsMap,
}

impl Player {
    pub fn new(
        id: PlayerId,
        tile_uuid: TileUuid,
        face_directions_to_tile_ids: HashMap<PlayerFaceDirection, TileId>,
        controls_map: PlayerControlsMap,
    ) -> Player {
        Player {
            id,
            tile_uuid,
            face_directions_to_tile_ids,
            move_direction_stack: Vec::new(),
            controls_map,
        }
    }

    pub fn map_face_directions_to_tile_ids(
        tileset: &Tileset,
    ) -> HashMap<PlayerFaceDirection, TileId> {
        tileset
            .properties
            .iter()
            .filter_map(
                |(&tile_id, properties)| match properties.get("face_direction") {
                    Some(tiled::PropertyValue::StringValue(face_direction)) => {
                        Some((PlayerFaceDirection::from(face_direction.as_ref()), tile_id))
                    }
                    _ => None,
                },
            )
            .collect()
    }
}

impl EventHandler for Player {
    fn handle_event(&mut self, event: &Event) {
        let player_action = if let Some(button) = event.press_args() {
            self.controls_map
                .get(&button)
                .map(|&action| (action, ButtonState::Pressed))
        } else if let Some(button) = event.release_args() {
            self.controls_map
                .get(&button)
                .map(|&action| (action, ButtonState::Released))
        } else {
            return;
        };

        if player_action.is_none() {
            return;
        }

        match player_action.unwrap() {
            (action, ButtonState::Pressed) => match action {
                PlayerAction::Movement(move_direction) => {
                    if !self.move_direction_stack.contains(&move_direction) {
                        self.move_direction_stack.push(move_direction);
                    }
                }
            },
            (action, ButtonState::Released) => match action {
                PlayerAction::Movement(move_direction) => {
                    self.move_direction_stack
                        .iter()
                        .position(|stored_move_direction| *stored_move_direction == move_direction)
                        .map(|index| self.move_direction_stack.remove(index));
                }
            },
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum PlayerId {
    Player1,
    Player2,
    Player3,
    Player4,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum PlayerFaceDirection {
    Down,
    Up,
    Left,
    Right,
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

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum ButtonState {
    Pressed,
    Released,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum PlayerAction {
    Movement(MoveDirection),
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}
