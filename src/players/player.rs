use engine::asset::{PropertyValue, TileId, Tileset};
use engine::game_state::{Button, Event, EventHandler, PressEvent, ReleaseEvent};
use std::collections::HashMap;

pub type PlayerControlsMap = HashMap<Button, PlayerAction>;

pub struct Player {
    pub id: PlayerId,
    //pub tile_uuid: TileUuid,
    pub face_directions_to_tile_ids: HashMap<PlayerFaceDirection, TileId>,
    move_direction_stack: Vec<MoveDirection>,
    controls_map: PlayerControlsMap,
}

impl Player {
    pub fn new(
        id: PlayerId,
        //tile_uuid: TileUuid,
        face_directions_to_tile_ids: HashMap<PlayerFaceDirection, TileId>,
        controls_map: PlayerControlsMap,
    ) -> Player {
        Player {
            id,
            //tile_uuid,
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
                    Some(PropertyValue::StringValue(face_direction)) => {
                        Some((PlayerFaceDirection::from(face_direction.as_ref()), tile_id))
                    }
                    _ => None,
                },
            )
            .collect()
    }

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
