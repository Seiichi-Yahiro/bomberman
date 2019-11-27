use crate::utils::Spritesheet;
use graphics::math::Vec2d;
use piston::input::Key;

const TEXTURE_FOLDER: &str = "app/assets/textures/player/";

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
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
    Standing,
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
