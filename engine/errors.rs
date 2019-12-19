use std::fmt::{Error, Formatter};

#[derive(Debug)]
pub enum TileEngineError {
    WrongFileType(String),
    TextureDirectoryNotFound(String),
    TileMapParseError(tiled::TiledError),
}

impl std::fmt::Display for TileEngineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let description = match self {
            TileEngineError::WrongFileType(e) => format!("WrongFileTypeError: {}", e),
        };

        write!(f, "{}", description.as_str())
    }
}

impl std::error::Error for TileEngineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
