use engine::animation::Animation;
use engine::asset::{AssetStorage, PropertyValue, TileId, Tileset};
use engine::components::{
    AnimationType, CurrentTileId, DefaultTileId, Layer, MapPosition, ScreenPosition, TilesetType,
};
use engine::game_state::{
    Button, Drawable, Event, EventHandler, GlGraphics, Matrix2d, PressEvent, ReleaseEvent,
    Updatable,
};
use engine::legion::{entity::Entity, world::World};
use std::collections::HashMap;

pub type PlayerControlsMap = HashMap<Button, PlayerAction>;

pub struct Player {}

impl Player {
    pub fn create_player(
        id: PlayerId,
        player_spawns: &HashMap<PlayerId, [u32; 2]>,
        asset_storage: &AssetStorage,
        world: &mut World,
    ) -> Entity {
        let [x, y] = player_spawns.get(&id).unwrap();
        let tileset = asset_storage.get_asset::<Tileset>(id.to_str());
        let face_directions_to_tile_ids = Player::map_face_directions_to_tile_ids(&tileset);
        let tile_id = face_directions_to_tile_ids
            .get(&PlayerFaceDirection::Down)
            .unwrap()
            .clone();

        world
            .insert(
                (Layer(1),),
                vec![(
                    MapPosition::new(*x, *y),
                    ScreenPosition::new(*x as f64, *y as f64),
                    DefaultTileId(tile_id),
                    CurrentTileId(tile_id),
                    TilesetType::Tileset(id.to_str()),
                    AnimationType::Ownd(
                        tileset
                            .animation_frames_holder
                            .get(&tile_id)
                            .cloned()
                            .map(|frames| {
                                let mut animation = Animation::new(frames);
                                animation.play();
                                animation
                            }),
                    ),
                )],
            )
            .first()
            .copied()
            .unwrap()
    }

    pub fn map_face_directions_to_tile_ids(
        tileset: &Tileset,
    ) -> HashMap<PlayerFaceDirection, TileId> {
        tileset
            .properties
            .iter()
            .filter_map(
                |(&tile_id, properties)| match properties.get("face_direction") {
                    Some(PropertyValue::StringValue(face_direction)) => {
                        Some((PlayerFaceDirection::from(face_direction.as_ref()), tile_id))
                    }
                    _ => None,
                },
            )
            .collect()
    }
    /*
    pub fn get_current_move_direction(&self) -> Option<MoveDirection> {
        self.move_direction_stack.last().cloned()
    }

    pub fn get_tile_id_for_move_direction(&self, move_direction: MoveDirection) -> Option<TileId> {
        let player_face_direction = match move_direction {
            MoveDirection::Up => PlayerFaceDirection::Up,
            MoveDirection::Down => PlayerFaceDirection::Down,
            MoveDirection::Left => PlayerFaceDirection::Left,
            MoveDirection::Right => PlayerFaceDirection::Right,
        };
        self.face_directions_to_tile_ids
            .get(&player_face_direction)
            .copied()
    }*/
}

/*impl EventHandler for Player {
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
}*/

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum PlayerId {
    Player1,
    Player2,
    Player3,
    Player4,
}

impl PlayerId {
    pub fn to_str(&self) -> &'static str {
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
