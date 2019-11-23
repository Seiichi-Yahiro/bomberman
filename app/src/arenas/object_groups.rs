pub enum ArenaObjectGroup {
    SoftBlockAreas,
    PlayerSpawns,
}

impl ArenaObjectGroup {
    pub fn as_str(&self) -> &str {
        match self {
            ArenaObjectGroup::SoftBlockAreas => "soft_block_areas",
            ArenaObjectGroup::PlayerSpawns => "player_spawns",
        }
    }
}

pub enum SoftBlockAreasProperties {
    SpawnChance,
}

impl SoftBlockAreasProperties {
    pub fn as_str(&self) -> &str {
        match self {
            SoftBlockAreasProperties::SpawnChance => "spawn_chance",
        }
    }
}

pub enum PlayerSpawnsProperties {
    PlayerId,
}

impl PlayerSpawnsProperties {
    pub fn as_str(&self) -> &str {
        match self {
            PlayerSpawnsProperties::PlayerId => "player_id",
        }
    }
}
