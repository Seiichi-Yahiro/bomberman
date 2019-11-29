pub fn load_tileset(folder: &str, tileset_file: &str) -> tiled::Tileset {
    let path = format!("{}{}", folder, tileset_file);
    tiled::parse_tileset(std::fs::File::open(path).unwrap(), 1).unwrap()
}
