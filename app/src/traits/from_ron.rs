use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

pub trait FromRON
where
    for<'de> Self: Deserialize<'de>,
{
    fn load_from_ron_file(path: &Path) -> Self {
        let file = File::open(path)
            .unwrap_or_else(|e| panic!("Failed to open RON file ({}): {}", path.display(), e));
        match from_reader(file) {
            Ok(x) => x,
            Err(e) => panic!("Failed to parse RON file ({}): {}", path.display(), e),
        }
    }
}
