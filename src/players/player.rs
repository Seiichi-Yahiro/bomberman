use engine::animation::Animation;
use engine::asset::{AssetStorage, PropertyValue, TileId, Tileset};
use engine::components::{
    AnimationType, Command, Controls, ControlsMap, CurrentTileId, DefaultTileId, Layer,
    MapPosition, ScreenPosition,
};
use engine::game_state::input::{ButtonEvent, ButtonState};
use engine::game_state::{
    input::{Button, Key},
    Event,
};
use engine::legion::prelude::*;
use engine::legion::{entity::Entity, world::World};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct MoveDirectionStack(pub Vec<MoveDirection>);

pub struct Player {}

impl Player {
    pub fn create_player(
        id: PlayerId,
        player_spawns: &HashMap<PlayerId, [u32; 2]>,
        asset_storage: &AssetStorage,
        world: &mut World,
    ) -> Entity {
        let [x, y] = player_spawns.get(&id).unwrap();
        let tileset = asset_storage.get_asset::<Tileset>(id.as_str());
        let tile_id = PlayerFaceDirection::Down.get_tile_id(&tileset).unwrap();

        world
            .insert(
                (Layer(1), id),
                vec![(
                    MapPosition::new(*x, *y),
                    ScreenPosition::new(*x as f64, *y as f64),
                    DefaultTileId(tile_id),
                    CurrentTileId(tile_id),
                    Arc::clone(&tileset),
                    Self::create_player_controls(id),
                    MoveDirectionStack(vec![]),
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

    fn create_player_controls(player_id: PlayerId) -> Controls {
        let mut controls: ControlsMap = HashMap::new();

        match player_id {
            PlayerId::Player1 => {
                controls.insert(
                    Button::Keyboard(Key::Left),
                    Box::new(move |button_state: ButtonState| {
                        Self::create_move_command(player_id, MoveDirection::Left, button_state)
                    }),
                );
                controls.insert(
                    Button::Keyboard(Key::Right),
                    Box::new(move |button_state: ButtonState| {
                        Self::create_move_command(player_id, MoveDirection::Right, button_state)
                    }),
                );
                controls.insert(
                    Button::Keyboard(Key::Up),
                    Box::new(move |button_state: ButtonState| {
                        Self::create_move_command(player_id, MoveDirection::Up, button_state)
                    }),
                );
                controls.insert(
                    Button::Keyboard(Key::Down),
                    Box::new(move |button_state: ButtonState| {
                        Self::create_move_command(player_id, MoveDirection::Down, button_state)
                    }),
                );
            }
            PlayerId::Player2 => {
                controls.insert(
                    Button::Keyboard(Key::A),
                    Box::new(move |button_state: ButtonState| {
                        Self::create_move_command(player_id, MoveDirection::Left, button_state)
                    }),
                );
                controls.insert(
                    Button::Keyboard(Key::D),
                    Box::new(move |button_state: ButtonState| {
                        Self::create_move_command(player_id, MoveDirection::Right, button_state)
                    }),
                );
                controls.insert(
                    Button::Keyboard(Key::W),
                    Box::new(move |button_state: ButtonState| {
                        Self::create_move_command(player_id, MoveDirection::Up, button_state)
                    }),
                );
                controls.insert(
                    Button::Keyboard(Key::S),
                    Box::new(move |button_state: ButtonState| {
                        Self::create_move_command(player_id, MoveDirection::Down, button_state)
                    }),
                );
            }
            PlayerId::Player3 => {}
            PlayerId::Player4 => {}
        }

        Controls(controls)
    }

    fn create_move_command(
        player_id: PlayerId,
        move_direction: MoveDirection,
        button_state: ButtonState,
    ) -> Command {
        let command = move |world: &mut World| {
            Self::store_move_direction(world, player_id, move_direction, button_state);
        };

        Box::new(command)
    }

    pub fn handle_event(world: &mut World, event: &Event) {
        let mut commands: Vec<Command> = vec![];
        let query = <Read<Controls>>::query();

        for controls in query.iter(world) {
            if let Some(button_args) = event.button_args() {
                if let Some(command_factory) = controls.0.get(&button_args.button) {
                    commands.push(command_factory(button_args.state));
                }
            };
        }

        commands.into_iter().for_each(|command| {
            command(world);
        });
    }

    fn store_move_direction(
        world: &mut World,
        player_id: PlayerId,
        move_direction: MoveDirection,
        button_state: ButtonState,
    ) {
        let query = <Write<MoveDirectionStack>>::query().filter(tag_value(&player_id));

        for mut move_direction_stack in query.iter(world) {
            match button_state {
                ButtonState::Press => {
                    move_direction_stack.0.push(move_direction);
                }
                ButtonState::Release => {
                    move_direction_stack
                        .0
                        .iter()
                        .position(|stored_move_direction| *stored_move_direction == move_direction)
                        .map(|index| move_direction_stack.0.remove(index));
                }
            }
        }
    }

    pub fn create_turn_player_system() -> Box<dyn Schedulable> {
        SystemBuilder::new("turn_player")
            .read_component::<DefaultTileId>()
            .write_component::<DefaultTileId>()
            .write_component::<CurrentTileId>()
            .with_query(
                <(Read<MoveDirectionStack>, Read<Arc<Tileset>>)>::query()
                    .filter(changed::<MoveDirectionStack>()),
            )
            .build(move |_commands, world, _resources, query| {
                for (entity, (move_direction_stack, tileset)) in query.iter_entities(&mut *world) {
                    let tile_id = move_direction_stack
                        .0
                        .last()
                        .map(|move_direction| match move_direction {
                            MoveDirection::Up => PlayerFaceDirection::Up,
                            MoveDirection::Down => PlayerFaceDirection::Down,
                            MoveDirection::Left => PlayerFaceDirection::Left,
                            MoveDirection::Right => PlayerFaceDirection::Right,
                        })
                        .and_then(|face_direction| face_direction.get_tile_id(&*tileset));

                    if let Some(tile_id) = tile_id {
                        if tile_id != world.get_component::<DefaultTileId>(entity).unwrap().0 {
                            world.get_component_mut::<DefaultTileId>(entity).unwrap().0 = tile_id;
                            world.get_component_mut::<CurrentTileId>(entity).unwrap().0 = tile_id;
                        }
                    }
                }
            })
    }

    pub fn create_move_player_system() -> Box<dyn Schedulable> {
        SystemBuilder::new("move_player")
            .write_component::<ScreenPosition>()
            .with_query(<Read<MoveDirectionStack>>::query())
            .build(move |_commands, world, _resources, query| {
                for (entity, (move_direction_stack)) in query.iter_entities(&mut *world) {
                    if let Some(move_direction) = move_direction_stack.0.last() {
                        let mut screen_position =
                            world.get_component_mut::<ScreenPosition>(entity).unwrap();
                        match move_direction {
                            MoveDirection::Up => screen_position.translate(0.0, -1.0),
                            MoveDirection::Down => screen_position.translate(0.0, 1.0),
                            MoveDirection::Left => screen_position.translate(-1.0, 0.0),
                            MoveDirection::Right => screen_position.translate(1.0, 0.0),
                        }
                    }
                }
            })
    }

    pub fn update(world: &mut World, asset_storage: &AssetStorage, dt: f64) {}
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
                |(tile_id, properties)| match properties.get("face_direction") {
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
